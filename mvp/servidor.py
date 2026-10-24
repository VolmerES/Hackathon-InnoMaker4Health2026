"""Servidor de la demo: la réplica de HCIS y la API de Medicci.

    python mvp/servidor.py                 # http://localhost:8000
    python mvp/servidor.py --puerto 8080

Solo usa la biblioteca estándar (y duckdb). En producción sería FastAPI detrás
de Caddy; la API es la misma.

API de Medicci (la que llama la burbuja):
    POST /api/sesion                    {nhc, medico}  → abre sesión atada a ese paciente y a ese médico
    GET  /api/sesion/<id>/tarjeta                      → «3 cosas antes de que entre»
    GET  /api/sesion/<id>/filas                        → sus datos, para el resumen de la burbuja
    POST /api/sesion/<id>/pregunta      {texto}        → respuesta con sus fuentes
    POST /api/sesion/<id>/registro      {accion}       → lo que hace en pantalla (ampliar, copiar…)
    GET  /api/estado                                   → si el modelo local responde

Sin médico no se abre ningún paciente, y todo lo que ve queda en el registro de
accesos (mvp/registro.py), encadenado para que no se pueda tocar sin que se note.

Lo que hace de HCIS en la demo (en el hospital es el propio HCIS):
    GET  /api/hcis/agenda                        → la agenda del día
    GET  /api/hcis/paciente/<nhc>                → lo que HCIS enseña del paciente

Con --datos hl7 arranca con la copia vacía y un receptor MLLP (puerto 2575):
los datos llegan como mensajes HL7 (mvp/hl7_emisor.py hace de HCIS).
    GET  /api/hl7/ultimos                        → los últimos mensajes recibidos

Para el panel de la demo (web/registro.html):
    GET  /api/registro?medico=&paciente=          → el registro de accesos y si la cadena está entera
"""

import argparse
import json
import sys
import threading
import time
import urllib.request
import uuid
from collections import Counter
from datetime import datetime
from urllib.parse import parse_qs, urlparse
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from agenda import elegir_agenda
from config import valor
from alertas import tarjeta
from paciente import cargar, cargar_vacia, datos_del_paciente
from registro import Registro, comprobar as comprobar_registro, leer as leer_registro
from responder import MODELO, URL, responder

AQUI = Path(__file__).resolve().parent
WEB = AQUI / "web"
FUENTES = AQUI.parent / "medicci-rs" / "fuentes"
MARCA = AQUI.parent / "marca"                       # la identidad: medicci.css, el isotipo     # Noto Sans, las mismas que la burbuja
# Lo que se puede apuntar desde la pantalla, además de lo que ya ve el servidor
ACCIONES = {"ampliar_resumen", "copiar_nota", "ver_pestana", "cerrar"}


class Medicci:
    def __init__(self, datos: str = "csv"):
        print("Cargando los datos…" if datos == "csv" else "Copia vacía: esperando mensajes HL7…", flush=True)
        self.con = cargar() if datos == "csv" else cargar_vacia()
        self.cerrojo = threading.Lock()       # DuckDB: una consulta a la vez en esta conexión
        self.sesiones: dict[str, dict] = {}    # sesión → paciente, médico y lo último que vio
        self.registro = Registro()
        self.hl7_recibidos: list[dict] = []   # los últimos mensajes, para enseñarlos en la demo
        self.simulacion = {"n": 0, "accion": "", "texto": ""}   # el paso de la simulación (simular.py)
        self.hl7_puerto = 2575
        self._agenda: list[dict] = []
        self._agenda_fija = datos == "csv"
        self._version = 0                     # sube con cada mensaje HL7
        self._version_agenda = -1
        self._ultimo_hl7 = 0.0

    def filas(self, nhc: str):
        with self.cerrojo:
            return datos_del_paciente(self.con, nhc)

    @property
    def agenda(self) -> list[dict]:
        # Con HL7 la copia se va llenando: la agenda se recalcula mientras llegan
        # mensajes y se fija cuando está completa y lleva 3 s sin llegar ninguno.
        # Así un evento en directo (anular una cita) no cambia la agenda.
        if self._agenda_fija and self._agenda:
            return self._agenda
        if self._version != self._version_agenda or not self._agenda:
            with self.cerrojo:
                self._version_agenda = self._version
                self._agenda = elegir_agenda(self.con, lambda n: datos_del_paciente(self.con, n))
        if len(self._agenda) >= 8 and time.monotonic() - self._ultimo_hl7 > 3:
            self._agenda_fija = True
        return self._agenda

    def evento_hl7(self, nhc: str, cual: str) -> str:
        """Simulación: HCIS manda ahora un mensaje nuevo (un resultado, una
        anulación). Entra por el receptor MLLP, como en el hospital."""
        from hl7 import Emisor, Mensaje
        from hl7_emisor import evento
        with self.cerrojo:
            texto = evento(self.con, nhc, cual)
        emisor = Emisor("127.0.0.1", self.hl7_puerto)
        try:
            return emisor.mandar(texto).campo("MSA", 1)
        finally:
            emisor.cerrar()

    def recibir_hl7(self, m):
        """Lo que hace el receptor con cada mensaje: guardarlo en la copia y contestar ACK."""
        from hl7 import ack, aplicar
        with self.cerrojo:
            resumen = aplicar(self.con, m)
            self._version += 1
            self._ultimo_hl7 = time.monotonic()
        self.hl7_recibidos = ([{"hora": datetime.now().strftime("%H:%M:%S"), "tipo": m.tipo,
                                "control": m.control, "resumen": resumen, "texto": m.texto}]
                              + self.hl7_recibidos)[:40]
        return ack(m)

    def fuentes(self, filas, refs: list[str]) -> list[dict]:
        por_ref = {f.ref: f for f in filas}
        return [{"ref": r, "texto": por_ref[r].texto} for r in refs if r in por_ref]

    def registrar(self, sesion: str, accion: str, **datos):
        s = self.sesiones.get(sesion, {})
        self.registro.apunta(medico=s.get("medico"), paciente=s.get("nhc"), sesion=sesion,
                             origen=s.get("origen"), accion=accion, **datos)


class Peticion(SimpleHTTPRequestHandler):
    medicci: Medicci = None
    url_modelo = URL
    modelo = MODELO

    def __init__(self, *a, **kw):
        super().__init__(*a, directory=str(WEB), **kw)

    def translate_path(self, path):
        ruta = path.split("?")[0]
        if ruta.startswith("/fuentes/"):
            return str(FUENTES / Path(ruta).name)
        if ruta.startswith("/marca/"):
            return str(MARCA / Path(ruta).name)
        return super().translate_path(path)

    def end_headers(self):
        # Que el navegador no se quede con una versión vieja de las páginas:
        # pasó con la burbuja web, que seguía saliendo después de quitarla.
        self.send_header("Cache-Control", "no-cache")
        super().end_headers()

    def log_message(self, formato, *args):
        if "/api/" in self.path:
            sys.stderr.write(f"{self.command} {self.path}\n")

    def _json(self, codigo: int, cuerpo) -> None:
        datos = json.dumps(cuerpo, ensure_ascii=False, default=str).encode()
        self.send_response(codigo)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(datos)))
        self.end_headers()
        self.wfile.write(datos)

    def _cuerpo(self) -> dict:
        n = int(self.headers.get("Content-Length") or 0)
        try:
            datos = json.loads(self.rfile.read(n) or b"{}")
        except ValueError:
            return {}
        return datos if isinstance(datos, dict) else {}

    def _sesion(self, partes):
        s = self.medicci.sesiones.get(partes[2]) if len(partes) > 3 else None
        if not s:
            self._json(404, {"error": "Sesión desconocida: vuelva a abrir al paciente."})
            return None
        return s["nhc"]

    def do_GET(self):
        partes = self.path.split("?")[0].strip("/").split("/")
        if partes[0] != "api":
            return super().do_GET()
        m = self.medicci
        if partes[1:] == ["estado"]:
            try:
                urllib.request.urlopen(f"{self.url_modelo.rstrip('/')}/models", timeout=3).read()
                ok = True
            except Exception:
                ok = False
            return self._json(200, {"modelo": self.modelo, "responde": ok})
        if partes[1:] == ["hcis", "agenda"]:
            return self._json(200, m.agenda)
        if partes[1:] == ["hl7", "ultimos"]:
            return self._json(200, m.hl7_recibidos)
        if partes[1:] == ["simulacion"]:
            return self._json(200, m.simulacion)
        if partes[1:] == ["registro"]:
            # Para el panel de la demo. En el hospital solo lo abre quien audita
            # (el delegado de protección de datos), con su propio usuario.
            q = {k: v[0] for k, v in parse_qs(urlparse(self.path).query).items()}
            todas = leer_registro(m.registro.ruta)
            fallos = comprobar_registro(todas)
            lineas = [l for l in todas if (not q.get("medico") or l.get("medico") == q["medico"])
                      and (not q.get("paciente") or l.get("paciente") == q["paciente"])]
            return self._json(200, {
                "lineas": lineas[::-1][:int(q.get("limite") or 300)], "total": len(lineas),
                "medicos": Counter(l.get("medico") for l in todas if l.get("medico")).most_common(),
                "pacientes": len({l.get("paciente") for l in todas if l.get("paciente")}),
                "cadena": {"encadenadas": sum("huella" in l for l in todas), "fallos": fallos}})
        if partes[1:3] == ["hcis", "paciente"] and len(partes) == 4:
            filas = m.filas(partes[3])
            if not filas:
                return self._json(404, {"error": "Paciente desconocido"})
            return self._json(200, {"filas": [{"ref": f.ref, "tabla": f.tabla, "texto": f.texto,
                                               "campos": f.campos}
                                              for f in filas if not f.campos.get("vacia")]})
        if partes[1] == "sesion" and partes[-1] == "tarjeta":
            nhc = self._sesion(partes)
            if not nhc:
                return
            filas = m.filas(nhc)
            t = tarjeta(filas)
            for a in t["alertas"]:
                a["fuentes"] = m.fuentes(filas, a["fuentes"])
            # se pide cada pocos segundos: se apunta la primera vez y cada vez que cambia
            vista = [a["texto"] for a in t["alertas"]]
            if m.sesiones[partes[2]].get("tarjeta") != vista:
                m.sesiones[partes[2]]["tarjeta"] = vista
                m.registrar(partes[2], "tarjeta", alertas=vista,
                            fuentes=[f["ref"] for a in t["alertas"] for f in a["fuentes"]])
            return self._json(200, t)
        if partes[1] == "sesion" and partes[-1] == "filas":
            nhc = self._sesion(partes)
            if not nhc:
                return
            filas = [f for f in m.filas(nhc) if not f.campos.get("vacia")]
            m.registrar(partes[2], "resumen", filas=len(filas))
            return self._json(200, {"filas": [{"ref": f.ref, "tabla": f.tabla, "texto": f.texto,
                                               "campos": f.campos} for f in filas]})
        self._json(404, {"error": "No existe"})

    def do_POST(self):
        partes = self.path.strip("/").split("/")
        m = self.medicci
        # La simulación del día del médico (mvp/simular.py): la página de HCIS
        # dice en qué paso va, y la burbuja (--simulacion) hace lo que toca.
        if partes == ["api", "simulacion", "paso"]:
            cuerpo = self._cuerpo()
            m.simulacion = {"n": m.simulacion["n"] + 1, "accion": str(cuerpo.get("accion", ""))[:40],
                            "texto": str(cuerpo.get("texto", ""))[:300]}
            return self._json(200, m.simulacion)
        if partes == ["api", "simulacion", "evento"]:
            cuerpo = self._cuerpo()
            try:
                ack = m.evento_hl7(str(cuerpo.get("nhc", "")), str(cuerpo.get("tipo", "resultado")))
            except (SystemExit, OSError) as e:
                return self._json(400, {"error": str(e)})
            return self._json(200, {"ack": ack})
        if partes[:2] == ["api", "sesion"] and len(partes) == 2:
            cuerpo = self._cuerpo()
            nhc = str(cuerpo.get("nhc", "")).strip()
            # El médico lo pone la sesión de Windows (la burbuja).
            # En el hospital vendría firmado por el inicio de sesión único, no escrito.
            medico = str(cuerpo.get("medico") or self.headers.get("X-Medicci-Medico") or "").strip()[:80]
            if not medico:
                return self._json(401, {"error": "No sé qué médico es: Medicci solo abre pacientes "
                                                 "a un usuario identificado en HCIS."})
            filas = m.filas(nhc) if nhc else []
            if not filas:
                return self._json(404, {"error": f"No hay datos del paciente {nhc or '(vacío)'}."})
            sesion = uuid.uuid4().hex[:12]
            # la sesión queda atada a este paciente y a este médico
            m.sesiones[sesion] = {"nhc": nhc, "medico": medico,
                                  "origen": str(cuerpo.get("origen") or "")[:20] or None}
            m.registrar(sesion, "abrir", equipo=self.client_address[0])
            ficha = filas[0].campos
            return self._json(200, {"sesion": sesion, "nhc": nhc, "edad": ficha.get("edad"),
                                    "filas": len(filas)})
        if partes[:2] == ["api", "sesion"] and partes[-1] == "pregunta":
            nhc = self._sesion(partes)
            if not nhc:
                return
            texto = str(self._cuerpo().get("texto", "")).strip()[:500]
            if not texto:
                return self._json(400, {"error": "Pregunta vacía"})
            filas = m.filas(nhc)
            inicio = time.perf_counter()
            try:
                r = responder(filas, texto, self.url_modelo, self.modelo)
            except Exception as e:
                return self._json(503, {"error": f"El modelo local no responde ({e.__class__.__name__})."})
            m.registrar(partes[2], "pregunta", pregunta=texto, respuesta=r.respuesta, consta=r.consta,
                        fuentes=r.fuentes, segundos=round(time.perf_counter() - inicio, 2))
            return self._json(200, {"respuesta": r.respuesta, "consta": r.consta,
                                    "fuentes": m.fuentes(r.usadas, r.fuentes), "segundos": round(r.segundos, 1)})
        if partes[:2] == ["api", "sesion"] and partes[-1] == "registro":
            if not self._sesion(partes):
                return
            cuerpo = self._cuerpo()
            accion = str(cuerpo.get("accion", ""))
            if accion not in ACCIONES:
                return self._json(400, {"error": "Acción desconocida"})
            m.registrar(partes[2], accion, detalle=str(cuerpo.get("detalle") or "")[:200] or None)
            return self._json(200, {"ok": True})
        self._json(404, {"error": "No existe"})


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--host", default=valor("MEDICCI_ESCUCHA", "127.0.0.1"))
    ap.add_argument("--puerto", type=int, default=int(valor("MEDICCI_PUERTO", "8000")))
    ap.add_argument("--url", default=URL, help="servidor del modelo, compatible con OpenAI")
    ap.add_argument("--modelo", default=MODELO)
    ap.add_argument("--datos", choices=["csv", "hl7"], default=valor("MEDICCI_DATOS", "csv"),
                    help="csv: lee los CSV; hl7: copia vacía que se llena con mensajes HL7")
    ap.add_argument("--hl7-puerto", type=int, default=int(valor("MEDICCI_HL7_PUERTO", "2575")))
    args = ap.parse_args()

    Peticion.medicci = Medicci(args.datos)
    if args.datos == "hl7":
        from hl7 import escuchar
        Peticion.medicci.hl7_puerto = args.hl7_puerto
        receptor = escuchar(args.host, args.hl7_puerto, Peticion.medicci.recibir_hl7)
        threading.Thread(target=receptor.serve_forever, daemon=True).start()
        print(f"Receptor HL7 (MLLP) escuchando en el puerto {args.hl7_puerto}", flush=True)
    Peticion.url_modelo, Peticion.modelo = args.url, args.modelo
    servidor = ThreadingHTTPServer((args.host, args.puerto), Peticion)
    print(f"Medicci listo: http://localhost:{args.puerto}  (modelo {args.modelo} en {args.url})", flush=True)
    servidor.serve_forever()


if __name__ == "__main__":
    main()
