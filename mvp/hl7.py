"""HL7 v2 mínimo: escribir y leer mensajes, el transporte MLLP y el receptor de Medici.

Solo biblioteca estándar, para que la demo se instale sin nada más. En el
hospital se usaría una librería completa (hl7apy) y el perfil de mensajes que
dé Informática ⚠️. Aquí van los mensajes estándar:

    ADT^A04  paciente                    REF^I12  derivación de Primaria
    SIU^S12  cita nueva                  SIU^S13  cita reprogramada
    SIU^S14  cita modificada (atendida)  SIU^S15  cita anulada
    SIU^S26  cita no realizada           OMG^O19  petición de prueba
    ORU^R01  resultado de prueba

Medici solo recibe: guarda cada mensaje en su copia y contesta ACK.
"""

import socket
import socketserver
from datetime import datetime

INICIO, FIN = b"\x0b", b"\x1c\x0d"          # MLLP: <VT> mensaje <FS><CR>
ESCAPES = [("\\", "\\E\\"), ("|", "\\F\\"), ("^", "\\S\\"), ("~", "\\R\\"), ("&", "\\T\\")]


# ---------------------------------------------------------------- escribir

def escapar(texto) -> str:
    texto = "" if texto is None else str(texto)
    for crudo, esc in ESCAPES:
        texto = texto.replace(crudo, esc)
    return texto.replace("\r", " ").replace("\n", " ")


def ts(valor) -> str:
    """Fecha HL7: AAAAMMDDhhmmss."""
    if valor is None:
        return ""
    if not isinstance(valor, datetime):
        valor = datetime(valor.year, valor.month, valor.day)
    return valor.strftime("%Y%m%d%H%M%S")


def segmento(nombre: str, *campos) -> str:
    """Cada campo es un texto o una lista de componentes (se unen con ^)."""
    partes = [nombre]
    for c in campos:
        partes.append("^".join(escapar(x) for x in c) if isinstance(c, (list, tuple)) else escapar(c))
    return "|".join(partes)


def mensaje(tipo: str, evento: str, estructura: str, control: str, cuando: datetime, *segmentos) -> str:
    msh = (f"MSH|^~\\&|HCIS|HUP|MEDICI|HUP|{ts(cuando)}||{tipo}^{evento}^{estructura}|"
           f"{escapar(control)}|P|2.5")
    return "\r".join([msh, *segmentos])


# ---------------------------------------------------------------- leer

def desescapar(texto: str) -> str:
    for crudo, esc in reversed(ESCAPES):
        texto = texto.replace(esc, crudo)
    return texto


class Mensaje:
    def __init__(self, texto: str):
        self.texto = texto.strip("\r\n")
        self.segmentos: dict[str, list[list[str]]] = {}
        for linea in self.texto.split("\r"):
            if not linea:
                continue
            campos = linea.split("|")
            if campos[0] == "MSH":
                campos = ["MSH", "|"] + campos[1:]      # así MSH-9 es campos[9], como en la norma
            self.segmentos.setdefault(campos[0], []).append(campos)

    def campo(self, seg: str, n: int, comp: int = 1, cual: int = 0) -> str:
        try:
            valor = self.segmentos[seg][cual][n]
        except (KeyError, IndexError):
            return ""
        partes = valor.split("^")
        return desescapar(partes[comp - 1]) if comp - 1 < len(partes) else ""

    @property
    def tipo(self) -> str:
        return f"{self.campo('MSH', 9, 1)}^{self.campo('MSH', 9, 2)}"

    @property
    def control(self) -> str:
        return self.campo("MSH", 10)

    def obx(self) -> dict[str, str]:
        return {self.campo("OBX", 3, cual=i): self.campo("OBX", 5, cual=i)
                for i in range(len(self.segmentos.get("OBX", [])))}


def fecha(valor: str):
    return datetime.strptime(valor[:14].ljust(14, "0"), "%Y%m%d%H%M%S") if valor else None


def ack(m: Mensaje, codigo: str = "AA", texto: str = "") -> str:
    return "\r".join([
        f"MSH|^~\\&|MEDICI|HUP|HCIS|HUP|{ts(datetime.now())}||ACK^{m.campo('MSH', 9, 2)}|A{m.control}|P|2.5",
        f"MSA|{codigo}|{m.control}|{escapar(texto)}"])


# ---------------------------------------------------------------- MLLP

def _leer_trama(sock) -> bytes | None:
    datos = b""
    while FIN not in datos:
        trozo = sock.recv(65536)
        if not trozo:
            return None
        datos += trozo
    return datos[datos.index(INICIO) + 1:datos.index(FIN)]


class Emisor:
    """Una conexión MLLP: manda mensajes y espera su ACK."""

    def __init__(self, host: str, puerto: int):
        self.sock = socket.create_connection((host, puerto), timeout=30)

    def mandar(self, texto: str) -> Mensaje:
        self.sock.sendall(INICIO + texto.encode("utf-8") + FIN)
        respuesta = _leer_trama(self.sock)
        if respuesta is None:
            raise ConnectionError("El receptor cerró la conexión")
        return Mensaje(respuesta.decode("utf-8"))

    def cerrar(self):
        self.sock.close()


def escuchar(host: str, puerto: int, manejar) -> socketserver.ThreadingTCPServer:
    """Receptor MLLP. `manejar(Mensaje) -> str` devuelve el ACK. Corre en otro hilo."""

    class Conexion(socketserver.BaseRequestHandler):
        def handle(self):
            while True:
                trama = _leer_trama(self.request)
                if trama is None:
                    return
                m = Mensaje(trama.decode("utf-8"))
                try:
                    respuesta = manejar(m)
                except Exception as e:                      # el emisor recibe AE y sigue
                    respuesta = ack(m, "AE", f"{e.__class__.__name__}: {e}")
                self.request.sendall(INICIO + respuesta.encode("utf-8") + FIN)

    servidor = socketserver.ThreadingTCPServer((host, puerto), Conexion)
    servidor.daemon_threads = True
    return servidor


# ---------------------------------------------------------------- receptor de Medici

def _guardar(con, tabla: str, clave: str, fila: dict) -> None:
    """Sustituye la fila con esa clave (la copia de Medici no lleva claves primarias)."""
    con.execute(f"DELETE FROM {tabla} WHERE {clave} = ?", [fila[clave]])
    columnas = ", ".join(fila)
    con.execute(f"INSERT INTO {tabla} ({columnas}) VALUES ({', '.join('?' * len(fila))})", list(fila.values()))


def aplicar(con, m: Mensaje) -> str:
    """Guarda el mensaje en la copia de Medici. Devuelve un resumen de una línea."""
    nhc = m.campo("PID", 3)
    tipo = m.tipo

    if tipo in ("ADT^A04", "ADT^A08"):
        o = m.obx()
        _guardar(con, "pacientes", "id_paciente", {
            "id_paciente": nhc, "edad": int(o.get("EDAD") or 0) or None, "sexo": m.campo("PID", 8),
            "preferencias_contacto": o.get("CONTACTO"), "necesidades_accesibilidad": o.get("ACCESIBILIDAD")})
        return f"paciente {nhc}"

    if tipo == "REF^I12":
        id_der = m.campo("RF1", 6)
        _guardar(con, "derivaciones_ap", "id_derivacion", {
            "id_derivacion": id_der, "id_paciente": nhc, "fecha_solicitud": fecha(m.campo("RF1", 7)),
            "especialidad_destino": m.campo("RF1", 3), "prioridad": m.campo("RF1", 2),
            "motivo_derivacion": m.campo("RF1", 10), "observaciones_medico": m.campo("NTE", 3),
            "estado_solicitud": m.campo("RF1", 1)})
        return f"derivación {id_der} de {nhc}"

    if tipo.startswith("SIU^"):
        id_cita, id_hueco, id_agenda = m.campo("SCH", 1), m.campo("SCH", 2), m.campo("AIL", 3, 1)
        inicio = fecha(m.campo("SCH", 11, 4))
        estado = m.campo("SCH", 25)
        if id_agenda and not con.execute("SELECT 1 FROM agendas WHERE id_agenda = ?", [id_agenda]).fetchone():
            con.execute("INSERT INTO agendas (id_agenda, servicio, especialidad) VALUES (?, ?, ?)",
                        [id_agenda, m.campo("AIL", 3, 4), m.campo("AIL", 3, 4)])
        if id_hueco and inicio:
            _guardar(con, "huecos_agenda", "id_hueco", {
                "id_hueco": id_hueco, "id_agenda": id_agenda, "fecha": inicio.date(), "hora_inicio": inicio.time()})
        anterior = con.execute("SELECT id_hueco, estado_actual FROM citas WHERE id_cita = ?", [id_cita]).fetchone()
        _guardar(con, "citas", "id_cita", {
            "id_cita": id_cita, "id_paciente": nhc, "id_derivacion": m.campo("SCH", 4) or None,
            "id_hueco": id_hueco or (anterior[0] if anterior else None), "tipo_visita": m.campo("SCH", 7),
            "estado_actual": estado, "prioridad": m.campo("SCH", 8)})
        _guardar(con, "movimientos_citas", "id_movimiento", {
            "id_movimiento": m.control, "id_cita": id_cita, "fecha_hora_movimiento": fecha(m.campo("EVN", 2)),
            "tipo_movimiento": m.campo("EVN", 4), "origen_movimiento": m.campo("EVN", 5),
            "motivo": m.campo("SCH", 6), "estado_anterior": anterior[1] if anterior else None,
            "estado_nuevo": estado, "id_hueco_anterior": anterior[0] if anterior else None,
            "id_hueco_nuevo": id_hueco or None})
        return f"cita {id_cita} de {nhc}: {m.campo('EVN', 4).lower()} → {estado}"

    if tipo in ("OMG^O19", "ORU^R01"):
        id_prueba = m.campo("ORC", 2)
        orc = m.campo("ORC", 5)
        estado = {"SC": "PROGRAMADA", "CM": "REALIZADA", "DC": "NO_REALIZADA"}.get(orc, orc)
        _guardar(con, "pruebas", "id_prueba", {
            "id_prueba": id_prueba, "id_paciente": nhc, "tipo_prueba": m.campo("OBR", 4),
            "fecha_solicitud": fecha(m.campo("OBR", 6)), "fecha_programada": fecha(m.campo("TQ1", 7)),
            "fecha_realizacion": fecha(m.campo("OBR", 7)) if estado == "REALIZADA" else None,
            "estado_prueba": estado, "resultado_disponible": 1 if m.campo("OBR", 25) == "F" else 0})
        return f"prueba {id_prueba} de {nhc}: {estado}"

    raise ValueError(f"Mensaje no soportado: {tipo}")
