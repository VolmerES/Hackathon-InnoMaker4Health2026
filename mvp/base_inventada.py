"""Una base inventada con dos pacientes y un historial realista, para probar
Medicci sin los CSV del reto.

Son María Alonso Vega y Ramón Esteve Gálvez, los de las réplicas de HCIS y
SELENE (demo/web): lo que cuentan sus informes queda aquí como lo guardan las
tablas de consultas externas. Derivaciones de Primaria, citas con sus cambios
y anulaciones, avisos que no llegan, pruebas que se hacen y otras que se
quedan sin hacer, años de seguimiento.

    python mvp/base_inventada.py                # los 11 CSV en data/
    python mvp/base_inventada.py --carpeta otra/

Las columnas son las del modelo de datos (docs/CONTEXTO.md). Además van sus
informes clínicos (DOCUMENTOS_HCE.csv: endoscopias, altas de Urgencias, notas de
Primaria, analíticas…), que en el hospital están en HCIS y no en los CSV del
reto. Son los mismos de bd/sql/04_datos.sql, la base de la burbuja. Si en data/
ya están los CSV del reto, no los pisa. Todo es inventado.
"""

import argparse
import csv
import sqlite3
import sys
from datetime import datetime, timedelta
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
HOY = datetime(2026, 10, 24, 9, 0)
MARCA = "BASE_INVENTADA.txt"
SQL = RAIZ / "bd" / "sql"
# el NHC de la base de la burbuja → el de los CSV
NHC = {"PAC-00417": "P0000417", "PAC-02856": "P0002856"}

CABECERAS = {
    "PACIENTES": ["id_paciente", "edad", "sexo", "preferencias_contacto", "necesidades_accesibilidad"],
    "DERIVACIONES_AP": ["id_derivacion", "id_paciente", "fecha_solicitud", "especialidad_destino", "prioridad",
                        "motivo_derivacion", "observaciones_medico", "estado_solicitud"],
    "AGENDAS": ["id_agenda", "servicio", "especialidad", "tipo_consulta", "profesional_ficticio", "duracion_cita",
                "horario_inicio", "horario_fin", "capacidad"],
    "HUECOS_AGENDA": ["id_hueco", "id_agenda", "fecha", "hora_inicio", "hora_fin", "estado_hueco", "tipo_hueco"],
    # las cuatro últimas no salen en el diagrama: con el nombre de lo que guardan
    "CITAS": ["id_cita", "id_paciente", "id_derivacion", "id_hueco", "fecha_asignacion", "tipo_visita",
              "estado_actual", "prioridad", "estado_final", "reprogramada", "anulada", "observaciones"],
    "MOVIMIENTOS_CITAS": ["id_movimiento", "id_cita", "fecha_hora_movimiento", "tipo_movimiento",
                          "origen_movimiento", "motivo", "estado_anterior", "estado_nuevo",
                          "id_hueco_anterior", "id_hueco_nuevo"],
    "OCUPACIONES_HUECO": ["id_ocupacion", "id_hueco", "id_cita", "inicio_ocupacion", "fin_ocupacion",
                          "motivo_fin", "ocupacion_activa"],
    "COMUNICACIONES": ["id_comunicacion", "id_cita", "id_paciente", "id_hueco_ofertado", "canal", "motivo",
                       "fecha_envio", "respuesta_paciente", "resultado"],
    "TAREAS_ADMINISTRATIVAS": ["id_tarea", "id_cita", "perfil_profesional", "tipo_tarea", "fecha_hora_inicio",
                               "duracion_minutos", "automatizable", "incidencia_asociada"],
    "CIRCUITO_CONSULTA": ["id_registro", "id_cita", "hora_llegada", "hora_admision", "hora_entrada_consulta",
                          "hora_salida_consulta", "tiempo_espera", "tiempo_atencion"],
    "PRUEBAS": ["id_prueba", "id_paciente", "id_cita_origen", "id_cita_revision", "tipo_prueba", "fecha_solicitud",
                "fecha_programada", "fecha_realizacion", "estado_prueba", "resultado_disponible"],
    # no es de los CSV del reto: los informes de HCIS
    "DOCUMENTOS_HCE": ["id_documento", "id_paciente", "fecha_documento", "tipo_documento", "servicio", "autor", "texto"],
}


# Los resultados que añade esta base a los de bd/sql (sin repetir sus números de documento)
MAS_INFORMES = [
    ["D19", "P0000417", "2026-10-20", "Resultado de pruebas", "Laboratorio · Atención Primaria", "—",
     "Calprotectina fecal: 38 µg/g (valor de referencia < 50 µg/g).\n"
     "Comentario del laboratorio: sin datos de inflamacion intestinal.\n"
     "Solicitada por: Dra. C. Herrero Nieto (C.S. Ensanche de Vallecas), 12/10/2026."],
    ["E15", "P0002856", "2025-01-15", "Analítica", "Laboratorio · Preoperatorio de Oftalmología", "—",
     "ANALITICA PREOPERATORIA (facoemulsificacion OD).\n"
     "Hemograma: Hb 11.8, Hto 36%, VCM 82, HCM 27, leucos 6.800, plaquetas 248.000.\n"
     "Coagulacion: INR 2.4 (anticoagulado con acenocumarol).\n"
     "Bioquimica: glucosa 99, Cr 1.55, FG 45, Na 140, K 4.5.\n"
     "Comentario: anemia leve microcitica. Valorar en la consulta de preanestesia."],
    ["E16", "P0002856", "2025-01-15", "Electrocardiograma", "Preoperatorio de Oftalmología", "—",
     "ECG: fibrilacion auricular con respuesta ventricular controlada a 78 lpm.\n"
     "Eje normal. QRS estrecho. Sin alteraciones agudas de la repolarizacion.\n"
     "Sin cambios respecto a ECG previos."],
    ["E17", "P0002856", "2026-05-05", "Informe de consulta", "Cardiología", "Dr. A. Peñalver Soto",
     "Revision anual. FA permanente con buen control de frecuencia. Asintomatico\n"
     "cardiologico; refiere mas cansancio al andar desde hace meses.\n"
     "ECG: FA a 74 lpm, sin cambios.\n"
     "Medicacion: acenocumarol segun INR, bisoprolol 2.5 mg/24h. Dice que dejo el\n"
     "Adiro, aunque no lo tiene claro y no se puede comprobar en la receta.\n"
     "En la analitica de marzo Hb 10.2 con ferritina 11: se recomienda completar el\n"
     "estudio de la anemia por su MAP. Revision en 1 año.\n"
     "Falto a la revision de marzo: no le llego la llamada (hipoacusia). Se anota\n"
     "que los avisos se hagan a su hijo."],
    ["E18", "P0002856", "2026-05-26", "Incidencia de citación", "Radiodiagnóstico", "Admisión de Radiología",
     "Paciente NO PRESENTADO a ECOGRAFIA ABDOMINAL programada el 26/05/2026 a las\n"
     "12:20 (peticion de Atencion Primaria del 20/03/2026, motivo: anemia ferropenica).\n"
     "Recordatorio telefonico del 24/05/2026: no contesta (3 intentos).\n"
     "Se devuelve la peticion a su medico de Atencion Primaria para nueva solicitud."],
]


def informes() -> list[list]:
    """Los informes de los dos, de la base de la burbuja (bd/sql), tal cual, y los de MAS_INFORMES."""
    con = sqlite3.connect(":memory:")
    for f in sorted(SQL.glob("0*.sql")):
        con.executescript(f.read_text(encoding="utf-8"))
    return [[d[0], NHC[d[1]], *d[2:]] for d in con.execute(
        "SELECT id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, texto "
        "FROM documentos_hce WHERE id_paciente IN (?, ?) ORDER BY id_paciente, fecha_documento", list(NHC))] \
        + MAS_INFORMES

# Primeras y sucesivas de cada servicio, con los profesionales de los informes de demo/web
AGENDAS = [
    ["AGDIG01", "Aparato digestivo", "DIG", "PRIMERA", "Dra. Sanz Oria", 20, "08:30", "14:30", 18],
    ["AGDIG02", "Aparato digestivo", "DIG", "SUCESIVA", "Dr. L. Marchena Ruiz", 15, "08:30", "14:30", 24],
    ["AGCAR01", "Cardiología", "CAR", "PRIMERA", "Dr. A. Peñalver Soto", 20, "08:00", "14:00", 18],
    ["AGCAR02", "Cardiología", "CAR", "SUCESIVA", "Dr. A. Peñalver Soto", 15, "08:00", "14:00", 24],
    ["AGTRA01", "Traumatología", "TRA", "PRIMERA", "Dr. G. Rendueles Pina", 20, "08:30", "14:30", 18],
    ["AGTRA02", "Traumatología", "TRA", "SUCESIVA", "Dr. G. Rendueles Pina", 15, "08:30", "14:30", 24],
    ["AGOFT01", "Oftalmología", "OFT", "PRIMERA", "Dr. S. Reina Cobo", 20, "08:00", "14:00", 18],
    ["AGOFT02", "Oftalmología", "OFT", "SUCESIVA", "Dra. V. Loriente Prats", 15, "08:00", "14:00", 24],
    ["AGDER01", "Dermatología", "DER", "PRIMERA", "Dra. E. Montalvo Ruiz", 20, "08:30", "14:30", 18],
    ["AGDER02", "Dermatología", "DER", "SUCESIVA", "Dra. E. Montalvo Ruiz", 15, "08:30", "14:30", 24],
    ["AGNRL01", "Neurología", "NRL", "PRIMERA", "Dr. J. Arteaga Liñán", 20, "08:00", "14:00", 18],
    ["AGNRL02", "Neurología", "NRL", "SUCESIVA", "Dr. J. Arteaga Liñán", 15, "08:00", "14:00", 24],
]


def ts(d: datetime) -> str:
    return d.strftime("%Y-%m-%d %H:%M")


def dt(texto: str) -> datetime:
    return datetime.strptime(texto, "%Y-%m-%d %H:%M" if " " in texto else "%Y-%m-%d")


class Base:
    def __init__(self):
        self.t = {tabla: [] for tabla in CABECERAS}
        self.t["AGENDAS"] = [list(a) for a in AGENDAS]
        self.n: dict[str, int] = {}

    def id(self, prefijo: str, ancho: int = 7) -> str:
        self.n[prefijo] = self.n.get(prefijo, 0) + 1
        return f"{prefijo}{self.n[prefijo]:0{ancho}d}"

    def paciente(self, pid, edad, sexo, canal, apoyo):
        self.t["PACIENTES"].append([pid, edad, sexo, canal, apoyo])
        self.canal = {**getattr(self, "canal", {}), pid: canal}

    def derivacion(self, did, pid, cuando, esp, prioridad, motivo, observaciones, estado="CITADA"):
        self.t["DERIVACIONES_AP"].append([did, pid, cuando, esp, prioridad, motivo, observaciones, estado])
        return did

    def _hueco(self, esp, tipo, cuando: datetime, estado="OCUPADO"):
        h = self.id("H")
        agenda = f"AG{esp}0{1 if tipo == 'P' else 2}"
        fin = cuando + timedelta(minutes=20 if tipo == "P" else 15)
        self.t["HUECOS_AGENDA"].append([h, agenda, cuando.strftime("%Y-%m-%d"), cuando.strftime("%H:%M"),
                                        fin.strftime("%H:%M"), estado, "PRIMERA" if tipo == "P" else "SUCESIVA"])
        return h

    def _aviso(self, cid, pid, motivo, cuando: datetime, falla=False, canal=None, respuesta=None):
        if cuando > HOY:
            return
        if respuesta is None:
            respuesta = "" if falla else {"CONFIRMACION_CITA": "CONFIRMA", "AVISO_CAMBIO_CITA": "ACEPTA"}.get(motivo, "")
        self.t["COMUNICACIONES"].append([self.id("CO"), cid, pid, "", canal or self.canal[pid], motivo, ts(cuando),
                                         respuesta, "FALLIDO" if falla else "ENTREGADO"])

    def _tarea(self, cid, perfil, tipo, cuando: datetime, minutos, automatizable, incidencia=""):
        self.t["TAREAS_ADMINISTRATIVAS"].append([self.id("TA"), cid, perfil, tipo, ts(cuando), minutos,
                                                 "SI" if automatizable else "NO", incidencia])

    def cita(self, pid, esp, cuando, tipo="S", estado="REALIZADA", der="", prioridad="ORDINARIA",
             asignada=None, cambios=(), anulada=None, fallan=(), espera=18, atencion=14, nota=""):
        """Una cita con todo lo que la rodea. `cambios`: [(cuándo se cambia, a qué
        fecha, quién, motivo)]; `anulada`: (cuándo, quién, motivo); `fallan`: los
        avisos que no le llegan ("CONFIRMACION_CITA", "RECORDATORIO", "AVISO_CAMBIO_CITA")."""
        cid = self.id("C")
        cuando = dt(cuando)
        asignada = dt(asignada) if asignada else cuando - timedelta(days=34, hours=-2)
        h = self._hueco(esp, tipo, cuando)
        movs = [(asignada, "CREACION", "DERIVACION_AP" if der else "SERVICIO", "Asignación inicial", "", "CITADA", "", h)]
        ocupa = [h, asignada]
        self._aviso(cid, pid, "CONFIRMACION_CITA", asignada + timedelta(minutes=7), "CONFIRMACION_CITA" in fallan)
        for cambio, nueva, quien, motivo in cambios:
            cambio, nueva = dt(cambio), dt(nueva)
            h2 = self._hueco(esp, tipo, nueva)
            movs.append((cambio, "REPROGRAMACION", quien, motivo, "CITADA", "CITADA", h, h2))
            self.t["OCUPACIONES_HUECO"].append([self.id("OC"), ocupa[0], cid, ts(ocupa[1]), ts(cambio), "REPROGRAMACION", "NO"])
            self._aviso(cid, pid, "AVISO_CAMBIO_CITA", cambio + timedelta(minutes=12), "AVISO_CAMBIO_CITA" in fallan)
            self._tarea(cid, "ADMINISTRATIVO", "REPROGRAMAR_CITA", cambio, 7, True)
            ocupa, h, cuando = [h2, cambio], h2, nueva
        fin, motivo_fin = "", ""
        if anulada:
            quita, quien, motivo = dt(anulada[0]), anulada[1], anulada[2]
            movs.append((quita, "ANULACION", quien, motivo, "CITADA", "ANULADA", h, ""))
            self._aviso(cid, pid, "AVISO_CAMBIO_CITA", quita + timedelta(minutes=20), "AVISO_CAMBIO_CITA" in fallan,
                        respuesta="" if "AVISO_CAMBIO_CITA" in fallan else "ENTERADO")
            estado, fin, motivo_fin = "ANULADA", ts(quita), "ANULACION"
        elif cuando <= HOY:
            recordatorio = (cuando - timedelta(days=2)).replace(hour=10, minute=0)
            self._aviso(cid, pid, "RECORDATORIO", recordatorio, "RECORDATORIO" in fallan)
            if "RECORDATORIO" in fallan:
                self._tarea(cid, "ADMINISTRATIVO", "LLAMADA_RECORDATORIO", recordatorio + timedelta(hours=3), 4, True,
                            "No contesta")
            if estado == "REALIZADA":
                movs.append((cuando, "ATENCION", "SISTEMA", "Cita atendida", "CITADA", "REALIZADA", h, h))
                llegada = cuando - timedelta(minutes=16)
                entrada = cuando + timedelta(minutes=espera)
                self.t["CIRCUITO_CONSULTA"].append([self.id("CC"), cid, ts(llegada), ts(llegada + timedelta(minutes=4)),
                                                    ts(entrada), ts(entrada + timedelta(minutes=atencion)), espera, atencion])
                fin, motivo_fin = ts(cuando + timedelta(minutes=espera + atencion)), "ATENDIDA"
            else:                                              # NO_REALIZADA: no se presenta
                movs.append((cuando, "CIERRE", "SISTEMA", "Paciente no presentado", "CITADA", "NO_REALIZADA", h, h))
                fin, motivo_fin = ts(cuando + timedelta(hours=6)), "NO_PRESENTADO"
        else:
            estado = "CITADA"
            recordatorio = (cuando - timedelta(days=2)).replace(hour=10, minute=0)
            self._aviso(cid, pid, "RECORDATORIO", recordatorio, "RECORDATORIO" in fallan)
        self.t["OCUPACIONES_HUECO"].append([self.id("OC"), ocupa[0], cid, ts(ocupa[1]), fin, motivo_fin,
                                            "SI" if not fin else "NO"])
        self.t["CITAS"].append([cid, pid, der, h, ts(asignada), tipo, estado, prioridad, estado,
                                1 if cambios else 0, 1 if anulada else 0, nota])
        for m in movs:
            self.t["MOVIMIENTOS_CITAS"].append([self.id("M"), cid, ts(m[0]), *m[1:]])
        return cid

    def prueba(self, pid, origen, tipo, solicitada, programada, realizada="", estado="REALIZADA",
               resultado=True, revision=""):
        self.t["PRUEBAS"].append([self.id("PR"), pid, origen, revision, tipo, solicitada, programada, realizada,
                                  estado, 1 if resultado and estado == "REALIZADA" else 0])

    def escribir(self, carpeta: Path):
        carpeta.mkdir(parents=True, exist_ok=True)
        for tabla, cabecera in CABECERAS.items():
            with open(carpeta / f"{tabla}.csv", "w", newline="", encoding="utf-8") as f:
                w = csv.writer(f)
                w.writerow(cabecera)
                w.writerows(self.t[tabla])


# =====================================================================
#  María Alonso Vega, 58 años: dispepsia que no responde al omeprazol
# =====================================================================
#  Lo que esconde su historia (demo/web/historial-datos.js): un adenoma de
#  colon en 2015 con control a los 5 años que se suspendió por la COVID y nadie
#  volvió a citar; la derivación de hoy dice «no IQ previas, no AINEs, NAMC»
#  y nada de eso es cierto. Cuidaba a su madre con demencia: faltó a una
#  revisión de Oftalmología.

DERIVACION_MARIA = """MOTIVO: dispepsia. estudio

mujer 58a, dolor epigastrico de 5 meses, posprandial, al principio dia si dia no
y ahora casi diario. plenitud precoz. no disfagia no vomitos. peso estable
(68 kg). ritmo intestinal normal, no rectorragias.
AP: HTA (enalapril). no IQ previas. no AINEs de forma habitual. no fumadora.
NAMC.
ha hecho 2 tandas de omeprazol 20mg/24h 8 sem cada una, mejora algo y al dejarlo
vuelve.
EF: abd blando depresible, molestias a la palpacion en epigastrio, no masas ni
megalias, murphy -
analitica 12/09: hemograma normal Hb 13.4 / bioq con perfil hepatico normal
GPT 24 GGT 31. Ag H.pylori heces NEGATIVO 18/09. SOH negativa 18/09 (todo en HCE)
dispepsia no investigada que no responde a IBP, ruego valoracion y estudio.
gracias"""


def maria(b: Base):
    pid = "P0000417"
    b.paciente(pid, 58, "F", "SMS", "NINGUNA")

    # 2015: sangre oculta positiva en el cribado → colonoscopia, adenoma
    d = b.derivacion("DER-2015-00312", pid, "2015-02-20 11:42", "DIG", "ORDINARIA",
                     "Cribado de cáncer colorrectal: sangre oculta en heces positiva",
                     "programa de cribado CCR: SOH + (inmunoquimica). asintomatica, no cambio de ritmo, "
                     "no rectorragia. sin AF de CCR. solicito colonoscopia. gracias")
    c = b.cita(pid, "DIG", "2015-03-04 10:20", "P", der=d, asignada="2015-02-23 09:15")
    b.prueba(pid, c, "ENDOSCOPIA", "2015-03-04 10:40", "2015-03-18 09:00", "2015-03-18 09:00")
    c = b.cita(pid, "DIG", "2015-04-15 12:00", asignada="2015-03-18 12:30",
               nota="Resultado de AP: adenoma tubular con displasia de bajo grado")
    # el control a los 5 años: se programó en 2015 y nunca se hizo
    b.prueba(pid, c, "ENDOSCOPIA", "2015-04-15 12:10", "2020-04-20 09:00", "", "PROGRAMADA", False)
    b.cita(pid, "DIG", "2020-04-27 11:30", asignada="2020-01-14 10:05",
           anulada=("2020-03-16 08:40", "SERVICIO", "Suspensión de la actividad programada (COVID-19)"))

    # 2023: ojo seco en Oftalmología; la revisión de 2024 no la hace
    d = b.derivacion("DER-2023-02117", pid, "2023-04-11 17:05", "OFT", "ORDINARIA",
                     "Ojo seco y cansancio visual",
                     "ojo seco bilateral de meses, con lagrimas artificiales sin mejoria. trabaja muchas "
                     "horas con pantallas. ruego valoracion")
    b.cita(pid, "OFT", "2023-05-30 09:40", "P", der=d, asignada="2023-04-20 08:50",
           cambios=[("2023-05-22 13:10", "2023-06-08 10:20", "SERVICIO", "Ausencia del profesional")])
    b.cita(pid, "OFT", "2024-06-12 16:40", estado="NO_REALIZADA", asignada="2023-06-08 10:45",
           nota="")

    # las de Urgencias y Primaria, con su informe (D05, D09, D12, D15, D16): sin cita de consultas
    b.prueba(pid, "", "ECG", "2020-06-03 10:05", "2020-06-03 10:15", "2020-06-03 10:15")
    b.prueba(pid, "", "ECOGRAFIA", "2020-06-03 10:30", "2020-06-03 12:40", "2020-06-03 12:40")
    b.prueba(pid, "", "ANALITICA", "2022-04-05 09:10", "2022-04-19 08:20", "2022-04-19 08:20")
    b.prueba(pid, "", "ECG", "2025-02-02 19:20", "2025-02-02 19:25", "2025-02-02 19:25")
    b.prueba(pid, "", "ANALITICA", "2025-02-02 19:22", "2025-02-02 19:40", "2025-02-02 22:45")
    b.prueba(pid, "", "RADIOGRAFIA", "2025-02-02 19:23", "2025-02-02 20:10", "2025-02-02 20:10")
    b.prueba(pid, "", "ANALITICA", "2026-07-21 12:30", "2026-09-12 08:10", "2026-09-12 08:10")
    b.prueba(pid, "", "ANALITICA", "2026-07-21 12:31", "2026-09-18 09:00", "2026-09-18 09:00")

    # hoy: la derivación preferente por la dispepsia; con ella, su médica pide una
    # ecografía (pendiente) y una calprotectina, cuyo resultado llega después
    d = b.derivacion("DER-2026-04871", pid, "2026-10-12 13:26", "DIG", "PREFERENTE",
                     "Dispepsia de 5 meses que no responde a IBP", DERIVACION_MARIA)
    b.prueba(pid, "", "ECOGRAFIA", "2026-10-12 13:30", "2026-11-06 10:40", "", "PROGRAMADA", False)
    b.prueba(pid, "", "ANALITICA", "2026-10-12 13:31", "2026-10-16 08:30", "2026-10-20 13:05")
    b.cita(pid, "DIG", "2026-10-24 16:40", "P", der=d, prioridad="PREFERENTE", asignada="2026-10-14 10:12")


# =====================================================================
#  Ramón Esteve Gálvez, 72 años: anemia ferropénica a estudio
# =====================================================================
#  Lo que esconde (demo/web/historial-datos-2.js): una angiodisplasia gástrica
#  que sangró en 2019 («no antecedentes digestivos», dice la derivación);
#  anticoagulado por fibrilación auricular, con Adiro sin indicación; alergia
#  al contraste yodado; la colonoscopia de control de 2026 sigue sin hacer.
#  Oye mal por teléfono: las llamadas de aviso no le llegan y falta a citas.

DERIVACION_RAMON = """MOTIVO: anemia ferropenica a estudio

varon de 72a con anemia microcitica Hb 9.6, ferritina 8, SOH +. astenia.
no antecedentes digestivos de interes. no dolor abdominal. no rectorragia.
tto: acenocumarol, bisoprolol, omeprazol, paracetamol.
analitica 28/09 en HCE.
derivo para valoracion y completar estudio, probable origen digestivo.
rgo valoracion. gracias"""


def ramon(b: Base):
    pid = "P0002856"
    b.paciente(pid, 72, "M", "LLAMADA", "AUDITIVA")

    # 2012: pirosis → gastroscopia, hernia de hiato y esofagitis A
    d = b.derivacion("DER-2012-00845", pid, "2012-03-20 12:15", "DIG", "ORDINARIA", "Pirosis y epigastralgia",
                     "pirosis y epigastralgia de meses, peor por la noche. omeprazol 20 con mejoria parcial. "
                     "fumador hasta 2005. solicito valoracion y gastroscopia si procede")
    c = b.cita(pid, "DIG", "2012-04-10 11:00", "P", der=d, asignada="2012-03-26 09:30")
    b.prueba(pid, c, "ENDOSCOPIA", "2012-04-10 11:15", "2012-05-06 08:30", "2012-05-06 08:30")
    b.cita(pid, "DIG", "2012-06-05 12:40", asignada="2012-05-07 10:00")

    # 2016: anemia leve → colonoscopia (pólipo hiperplásico, control a los 10 años) y TAC sin contraste
    d = b.derivacion("DER-2016-01934", pid, "2016-07-12 10:48", "DIG", "ORDINARIA",
                     "Anemia leve y cambio del ritmo intestinal",
                     "Hb 12.1, ferritina 22. cambio de ritmo de 3 meses, alterna estreñimiento. "
                     "SOH positiva. ruego colonoscopia")
    c = b.cita(pid, "DIG", "2016-08-30 10:00", "P", der=d, asignada="2016-07-19 08:20")
    b.prueba(pid, c, "ENDOSCOPIA", "2016-08-30 10:20", "2016-09-22 09:00", "2016-09-22 09:00")
    b.prueba(pid, c, "TAC", "2016-08-30 10:22", "2016-10-14 12:30", "2016-10-14 12:30")
    c = b.cita(pid, "DIG", "2016-11-08 11:20", asignada="2016-10-17 09:40",
               nota="Durante el TAC: urticaria con el contraste yodado; se suspende el contraste")
    # el control a los 10 años: está pedido, sin hacer
    b.prueba(pid, c, "ENDOSCOPIA", "2016-11-08 11:35", "2026-11-12 09:00", "", "PROGRAMADA", False)

    # 2018: palpitaciones → fibrilación auricular permanente; revisión cada año en Cardiología
    d = b.derivacion("DER-2018-00377", pid, "2018-01-15 18:02", "CAR", "PREFERENTE",
                     "Palpitaciones y disnea de esfuerzo",
                     "palpitaciones de semanas y disnea de moderados esfuerzos. ECG en el centro: ritmo "
                     "irregular a 100 lpm, probable FA. inicio bisoprolol. ruego valoracion preferente")
    c = b.cita(pid, "CAR", "2018-02-06 09:20", "P", der=d, prioridad="PREFERENTE", asignada="2018-01-22 11:00")
    b.prueba(pid, c, "ECG", "2018-02-06 09:30", "2018-02-06 09:45", "2018-02-06 09:45")
    b.prueba(pid, c, "ECOGRAFIA", "2018-02-06 09:35", "2018-02-27 10:10", "2018-02-27 10:10")
    b.cita(pid, "CAR", "2018-03-03 10:40", asignada="2018-02-06 09:50",
           nota="FA permanente: se inicia acenocumarol")
    b.cita(pid, "CAR", "2019-03-12 11:00", asignada="2018-03-03 11:00")
    # 2019: sangrado por angiodisplasia gástrica (Urgencias), gastroscopia urgente con argón
    c = b.cita(pid, "DIG", "2019-12-17 12:20", asignada="2019-11-30 13:00",
               nota="Revisión tras hemorragia digestiva alta por angiodisplasia")
    b.prueba(pid, c, "ENDOSCOPIA", "2019-11-27 18:10", "2019-11-27 20:30", "2019-11-27 20:30")
    b.prueba(pid, c, "ANALITICA", "2019-12-17 12:40", "2020-01-10 08:15", "2020-01-10 08:15")
    b.cita(pid, "CAR", "2020-03-10 10:20", asignada="2019-03-12 11:20",
           cambios=[("2020-03-16 09:05", "2020-06-16 10:00", "SERVICIO",
                     "Suspensión de la actividad programada (COVID-19)")])
    b.cita(pid, "CAR", "2021-03-09 09:40", asignada="2020-06-16 10:20")

    # 2021: gonartrosis → prótesis de rodilla derecha
    d = b.derivacion("DER-2021-00590", pid, "2021-01-28 12:30", "TRA", "ORDINARIA", "Gonartrosis derecha avanzada",
                     "dolor mecanico de rodilla dcha de años, limita la marcha a 200 m. Rx: gonartrosis "
                     "tricompartimental. paracetamol, no AINEs (anticoagulado). valorar IQ")
    c = b.cita(pid, "TRA", "2021-03-02 12:00", "P", der=d, asignada="2021-02-04 10:30")
    b.prueba(pid, c, "RADIOGRAFIA", "2021-03-02 12:15", "2021-03-02 12:40", "2021-03-02 12:40")
    b.cita(pid, "TRA", "2021-07-06 11:20", asignada="2021-06-10 12:00", nota="Revisión tras artroplastia")
    b.cita(pid, "TRA", "2022-01-11 10:40", asignada="2021-07-06 11:40")

    # las llamadas no le llegan: falta a Cardiología y hay que volver a citarle
    b.cita(pid, "CAR", "2022-03-15 10:00", estado="NO_REALIZADA", asignada="2021-03-09 10:00", fallan={"RECORDATORIO"})
    b.cita(pid, "CAR", "2022-05-10 09:20", asignada="2022-03-16 12:10")
    b.cita(pid, "CAR", "2023-03-14 10:20", asignada="2022-05-10 09:40")
    b.cita(pid, "CAR", "2024-03-12 11:00", asignada="2023-03-14 10:40")

    # 2023: Nefrología, con analítica y ecografía renal (E08)
    b.prueba(pid, "", "ANALITICA", "2023-03-30 10:00", "2023-04-12 08:15", "2023-04-12 08:15")
    b.prueba(pid, "", "ECOGRAFIA", "2023-03-30 10:02", "2023-04-21 11:30", "2023-04-21 11:30")

    # 2024: catarata → lista de espera; las revisiones se mueven
    d = b.derivacion("DER-2024-03311", pid, "2024-07-10 09:55", "OFT", "ORDINARIA", "Disminución de la agudeza visual",
                     "refiere ver borroso de lejos desde hace un año, peor el OD. ruego valoracion. "
                     "anticoagulado con sintrom")
    b.cita(pid, "OFT", "2024-09-19 12:40", "P", der=d, asignada="2024-07-18 08:30",
           nota="Catarata bilateral: lista de espera de facoemulsificación OD")
    # el preoperatorio de la catarata (E15, E16): la hemoglobina ya baja
    b.prueba(pid, "", "ANALITICA", "2024-12-18 12:00", "2025-01-15 08:20", "2025-01-15 08:20")
    b.prueba(pid, "", "ECG", "2024-12-18 12:01", "2025-01-15 08:40", "2025-01-15 08:40")
    b.cita(pid, "CAR", "2025-03-11 10:00", estado="NO_REALIZADA", asignada="2024-03-12 11:20",
           fallan={"RECORDATORIO"})
    b.cita(pid, "CAR", "2025-04-22 09:40", asignada="2025-03-12 09:15")
    b.cita(pid, "OFT", "2025-02-11 11:00", asignada="2024-09-19 13:00",
           cambios=[("2025-02-03 12:20", "2025-03-25 10:40", "SERVICIO", "Reorganización de agenda")])
    # el último año: la anemia de marzo (E11); Primaria pide una ecografía abdominal
    # que no se hace, porque la llamada para recordársela no le llega
    b.cita(pid, "CAR", "2026-03-10 10:20", estado="NO_REALIZADA", asignada="2025-04-22 10:00",
           fallan={"RECORDATORIO"})
    b.prueba(pid, "", "ANALITICA", "2026-02-24 09:30", "2026-03-12 08:10", "2026-03-12 08:10")
    b.prueba(pid, "", "ECOGRAFIA", "2026-03-20 11:15", "2026-05-26 12:20", "", "NO_REALIZADA", False)
    b._aviso("", pid, "RECORDATORIO", dt("2026-05-24 10:00"), falla=True)
    b._tarea("", "ADMINISTRATIVO", "LLAMADA_RECORDATORIO", dt("2026-05-24 13:00"), 4, True, "No contesta")
    c = b.cita(pid, "CAR", "2026-05-05 09:00", asignada="2026-03-11 12:30")
    b.prueba(pid, c, "ECG", "2026-05-05 09:10", "2026-05-05 09:25", "2026-05-05 09:25")
    b.cita(pid, "OFT", "2026-05-19 12:00", asignada="2025-03-25 11:00",
           cambios=[("2026-05-12 13:40", "2026-06-23 11:20", "SERVICIO", "Ausencia del profesional")],
           fallan={"AVISO_CAMBIO_CITA"})

    b.prueba(pid, "", "ANALITICA", "2026-09-15 10:10", "2026-09-28 08:20", "2026-09-28 08:20")

    # hoy: la derivación preferente por la anemia; la confirmación por teléfono no le llega
    d = b.derivacion("DER-2026-04902", pid, "2026-10-14 12:51", "DIG", "PREFERENTE",
                     "Anemia ferropénica a estudio", DERIVACION_RAMON)
    c = b.cita(pid, "DIG", "2026-10-24 17:10", "P", der=d, prioridad="PREFERENTE", asignada="2026-10-15 09:30",
               fallan={"CONFIRMACION_CITA"})
    b._tarea(c, "ADMINISTRATIVO", "LLAMADA_CONFIRMACION", dt("2026-10-16 11:00"), 6, False, "Contesta su hijo")
    b._aviso(c, pid, "CONFIRMACION_CITA", dt("2026-10-16 11:15"))


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--carpeta", default=str(RAIZ / "data"))
    args = ap.parse_args()
    carpeta = Path(args.carpeta)
    if (carpeta / "PACIENTES.csv").exists() and not (carpeta / MARCA).exists():
        sys.exit(f"En {carpeta} ya hay CSV que no son de esta base (¿los del reto?): no los piso.")

    b = Base()
    maria(b)
    ramon(b)
    b.t["DOCUMENTOS_HCE"] = informes()
    b.escribir(carpeta)
    (carpeta / MARCA).write_text(
        "Base inventada por mvp/base_inventada.py: dos pacientes, ningún dato es real.\n"
        "Si llegan los CSV del reto, se copian encima y se borra este fichero.\n", encoding="utf-8")
    print(f"Base inventada en {carpeta}:")
    print("  " + ", ".join(f"{t.lower()} {len(f)}" for t, f in b.t.items()))


if __name__ == "__main__":
    main()
