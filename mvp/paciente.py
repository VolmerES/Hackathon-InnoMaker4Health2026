"""Las 6 consultas fijas: todo lo que hay de un paciente, en filas numeradas.

Cada fila lleva una referencia ([F1], [D1], [C1], [M1], [P1], [A1]) que el
modelo tiene que citar. El servidor comprueba después que existe.
"""

import sys
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "analisis"))
from datos import conectar  # noqa: E402

ESPECIALIDADES = {
    "OFT": "Oftalmología",
    "TRA": "Traumatología",
    "DER": "Dermatología",
    "CAR": "Cardiología",
    "NRL": "Neurología",
    "DIG": "Aparato digestivo",
}
TIPO_VISITA = {"P": "primera", "S": "sucesiva"}


@dataclass
class Fila:
    ref: str      # [C3] sin corchetes: "C3"
    tabla: str    # de qué tabla sale
    id: str       # id original en el CSV
    texto: str    # lo que ve el modelo
    campos: dict = field(default_factory=dict)   # los datos en bruto, para las alertas


TABLAS_MEDICI = ["pacientes", "derivaciones_ap", "citas", "huecos_agenda", "agendas",
                 "movimientos_citas", "pruebas", "comunicaciones"]


def cargar():
    """Copia en memoria las tablas que usa Medici: leer los CSV en cada consulta es lento."""
    con = conectar()
    for t in TABLAS_MEDICI:
        con.execute(f"CREATE TABLE {t}_m AS SELECT * FROM {t}")
        con.execute(f"DROP VIEW {t}")
        con.execute(f"ALTER TABLE {t}_m RENAME TO {t}")
    return con


def cargar_vacia():
    """La copia de Medici vacía, para llenarla con mensajes HL7.

    Los avisos al paciente no van por HL7 (vienen de la plataforma de SMS):
    esos sí se cargan de su CSV.
    """
    con = conectar()
    for t in TABLAS_MEDICI:
        filtro = "" if t == "comunicaciones" else " LIMIT 0"
        con.execute(f"CREATE TABLE {t}_m AS SELECT * FROM {t}{filtro}")
        con.execute(f"DROP VIEW {t}")
        con.execute(f"ALTER TABLE {t}_m RENAME TO {t}")
    return con


def _fecha(valor) -> str:
    return valor.strftime("%d/%m/%Y") if valor else "sin fecha"


def _numerar(prefijo: str, tabla: str, filas: list[tuple[str, str, dict]]) -> list[Fila]:
    return [Fila(f"{prefijo}{i}", tabla, id_, texto, campos)
            for i, (id_, texto, campos) in enumerate(filas, 1)]


def datos_del_paciente(con, id_paciente: str) -> list[Fila]:
    """Las 6 consultas, siempre filtradas por el paciente que pone el servidor."""
    filas: list[Fila] = []

    p = con.execute(
        "SELECT edad, sexo, preferencias_contacto, necesidades_accesibilidad "
        "FROM pacientes WHERE id_paciente = ?", [id_paciente]).fetchone()
    if p:
        filas += _numerar("F", "PACIENTES", [(id_paciente,
            f"Ficha: {p[0]} años, sexo {p[1]}. Contacto preferido: {p[2]}. "
            f"Necesidades de accesibilidad: {p[3]}.",
            {"edad": p[0], "contacto": p[2], "accesibilidad": p[3]})])

    der = con.execute(
        "SELECT id_derivacion, fecha_solicitud, especialidad_destino, prioridad, "
        "motivo_derivacion, observaciones_medico, estado_solicitud "
        "FROM derivaciones_ap WHERE id_paciente = ? ORDER BY fecha_solicitud",
        [id_paciente]).fetchall()
    filas += _numerar("D", "DERIVACIONES_AP", [(d[0],
        f"Derivación de Primaria el {_fecha(d[1])} a {ESPECIALIDADES.get(d[2], d[2])}, "
        f"prioridad {d[3]}, estado {d[6]}. Motivo: {d[4]}. Observaciones del médico: {d[5]}",
        {"fecha": d[1], "especialidad": d[2], "prioridad": d[3], "motivo": d[4], "estado": d[6],
         "observaciones": d[5]})
        for d in der])

    citas = con.execute(
        "SELECT c.id_cita, h.fecha, h.hora_inicio, a.especialidad, c.tipo_visita, "
        "c.estado_actual, c.prioridad "
        "FROM citas c LEFT JOIN huecos_agenda h USING (id_hueco) "
        "LEFT JOIN agendas a USING (id_agenda) "
        "WHERE c.id_paciente = ? ORDER BY h.fecha, h.hora_inicio", [id_paciente]).fetchall()
    filas_citas = _numerar("C", "CITAS", [(c[0],
        f"Cita en {ESPECIALIDADES.get(c[3], c[3] or 'servicio desconocido')} el {_fecha(c[1])}"
        f"{' a las ' + c[2].strftime('%H:%M') if c[2] else ''}, visita "
        f"{TIPO_VISITA.get(c[4], c[4])}, estado {c[5]}, prioridad {c[6]}.",
        {"fecha": c[1], "especialidad": c[3], "estado": c[5], "hora": c[2], "tipo_visita": c[4],
         "prioridad": c[6]})
        for c in citas])
    filas += filas_citas
    ref_cita = {f.id: f.ref for f in filas_citas}

    mov = con.execute(
        "SELECT m.id_movimiento, m.id_cita, m.fecha_hora_movimiento, m.tipo_movimiento, "
        "m.origen_movimiento, m.motivo "
        "FROM movimientos_citas m JOIN citas c USING (id_cita) "
        "WHERE c.id_paciente = ? AND (m.tipo_movimiento IN ('REPROGRAMACION', 'ANULACION') "
        "OR m.estado_nuevo = 'NO_REALIZADA') ORDER BY m.fecha_hora_movimiento",
        [id_paciente]).fetchall()
    # Escrito para que no se confundan: «no realizada» no es «anulada».
    que_paso = {
        "REPROGRAMACION": "fue reprogramada (cambiada de fecha)",
        "ANULACION": "fue anulada",
    }
    filas += _numerar("M", "MOVIMIENTOS_CITAS", [(m[0],
        f"La cita [{ref_cita.get(m[1], m[1])}] "
        + (f"{que_paso[m[3]]} el {_fecha(m[2])}, a petición de {m[4]}. Motivo: {m[5]}."
           if m[3] in que_paso else
           f"quedó como NO REALIZADA el {_fecha(m[2])}: se cerró sin atender al paciente, "
           f"no es una anulación. Motivo: {m[5]}."),
        {"tipo": m[3] if m[3] in que_paso else "NO_REALIZADA", "fecha": m[2],
         "cita": ref_cita.get(m[1]), "motivo": m[5]})
        for m in mov])

    pruebas = con.execute(
        "SELECT id_prueba, tipo_prueba, fecha_solicitud, fecha_programada, "
        "fecha_realizacion, estado_prueba, resultado_disponible "
        "FROM pruebas WHERE id_paciente = ? ORDER BY fecha_solicitud", [id_paciente]).fetchall()
    filas += _numerar("P", "PRUEBAS", [(p[0],
        f"Prueba {p[1]}: solicitada el {_fecha(p[2])}, programada el {_fecha(p[3])}, "
        f"realizada {'el ' + _fecha(p[4]) if p[4] else 'no'}, estado {p[5]}, "
        f"resultado disponible: {'sí' if p[6] else 'no'} (el valor del resultado no está en estos datos).",
        {"tipo": p[1], "programada": p[3], "realizada": p[4], "estado": p[5], "resultado": bool(p[6])})
        for p in pruebas])

    avisos = con.execute(
        "SELECT id_comunicacion, id_cita, canal, motivo, fecha_envio, resultado, respuesta_paciente "
        "FROM comunicaciones WHERE id_paciente = ? ORDER BY fecha_envio", [id_paciente]).fetchall()
    filas += _numerar("A", "COMUNICACIONES", [(a[0],
        f"Aviso {a[3]} por {a[2]} el {_fecha(a[4])} sobre la cita [{ref_cita.get(a[1], a[1])}]: "
        f"{a[5]}, respuesta del paciente: {a[6] or 'ninguna'}.",
        {"canal": a[2], "fecha": a[4], "resultado": a[5], "motivo": a[3], "respuesta": a[6],
         "cita": ref_cita.get(a[1])})
        for a in avisos])

    # Una tabla vacía también es un dato: si no se dice, el modelo rellena el hueco
    # con lo que tiene (preguntado por pruebas, contestaba con citas).
    vacias = {
        "DERIVACIONES_AP": ("D", "No hay derivaciones de Primaria registradas para este paciente."),
        "CITAS": ("C", "No hay citas registradas para este paciente."),
        "MOVIMIENTOS_CITAS": ("M", "No hay citas reprogramadas, anuladas ni cerradas sin atender."),
        "PRUEBAS": ("P", "No hay pruebas registradas para este paciente: ni pedidas, ni pendientes, ni hechas."),
        "COMUNICACIONES": ("A", "No hay avisos enviados a este paciente."),
    }
    hay = {f.tabla for f in filas}
    for tabla, (prefijo, texto) in vacias.items():
        if filas and tabla not in hay:
            filas.append(Fila(f"{prefijo}0", tabla, "", texto, {"vacia": True}))
    return filas


def formatear(filas: list[Fila]) -> str:
    return "\n".join(f"[{f.ref}] {f.texto}" for f in filas)


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    print(formatear(datos_del_paciente(cargar(), sys.argv[1] if len(sys.argv) > 1 else "P0009693")))
