"""Prueba de acierto de Medicci: 30 preguntas con la respuesta correcta sacada de los CSV.

    python mvp/prueba_acierto.py --solo-preguntas        # ver las preguntas y su respuesta correcta
    python mvp/prueba_acierto.py                         # Ollama en localhost con qwen3:8b
    python mvp/prueba_acierto.py --modelo qwen3:4b
    python mvp/prueba_acierto.py --url http://spark:8000/v1 --modelo Qwen/Qwen3-30B-A3B-FP8

La respuesta correcta no la escribe nadie a mano: sale de SQL sobre los mismos
datos. Los pacientes se eligen al azar pero siempre los mismos para una
--semilla, así se pueden comparar modelos.
"""

import argparse
import json
import re
import statistics
import sys
import unicodedata
from datetime import datetime
from pathlib import Path

from paciente import ESPECIALIDADES, cargar, datos_del_paciente
from responder import MODELO, URL, Respuesta, responder

# Cómo pregunta el médico. La biomédica puede cambiar las frases aquí.
PREGUNTAS = {
    "especialidad_si": "¿Ha tenido citas en {esp}?",
    "especialidad_no": "¿Ha tenido citas en {esp}?",
    "pendientes": "¿Qué pruebas tiene pendientes?",
    "accesibilidad": "¿Necesita algún apoyo para la consulta?",
    "motivo": "¿Por qué lo manda su médico de cabecera?",
    "cambios": "¿Le hemos cambiado o anulado alguna cita, o ha faltado a alguna? ¿Por qué?",
    "contacto": "¿Cómo prefiere que le avisemos?",
}
TRAMPAS = [
    "¿Tiene alergias?",
    "¿Qué medicación toma?",
    "¿Cuál es su diagnóstico?",
    "¿Tiene antecedentes familiares de cáncer?",
    "¿Fuma?",
    "¿Qué valores salieron en la última analítica?",
]
# Cuántos casos de cada tipo: 30 en total.
REPARTO = {"especialidad_si": 3, "especialidad_no": 2, "pendientes": 5, "accesibilidad": 4,
           "motivo": 4, "cambios": 4, "contacto": 2, "trampa": 6}

CLAVES_ACCESIBILIDAD = {"MOVILIDAD_REDUCIDA": "movilidad", "INTERPRETE": "interprete",
                        "ACOMPANANTE": "acompan", "AUDITIVA": "audit", "VISUAL": "visual"}
CLAVES_CONTACTO = {"SMS": "sms", "LLAMADA": "llam|telef", "EMAIL": "email|e-mail|correo",
                   "APP": "app|aplicacion", "CARTA": "carta"}
NEGACION = r"\b(no|ninguna|ninguno|ningun|sin)\b"
# Palabras que salen en cualquier derivación o en la propia pregunta: no prueban nada.
GENERICAS = {"derivacion", "primaria", "prioridad", "ordinaria", "preferente", "estado",
             "citada", "motivo", "observaciones", "medico", "saludo", "cordial", "paciente",
             "cabecera", "servicio", "solicita"}


def normal(texto: str) -> str:
    sin_tildes = unicodedata.normalize("NFKD", texto).encode("ascii", "ignore").decode()
    return sin_tildes.lower()


# ---------------------------------------------------------------- elegir casos

def _pacientes(con, sql: str, semilla: int, usados: set, n: int) -> list[str]:
    """Pacientes que cumplen `sql`, en un orden pseudoaleatorio fijo por semilla."""
    filas = con.execute(
        f"SELECT id_paciente FROM ({sql}) ORDER BY hash(id_paciente || ?) LIMIT ?",
        [str(semilla), n + len(usados) + 50]).fetchall()
    elegidos = [f[0] for f in filas if f[0] not in usados][:n]
    usados.update(elegidos)
    return elegidos


def generar_casos(con, semilla: int) -> list[dict]:
    usados: set[str] = set()
    casos: list[dict] = []
    citas_con_esp = ("SELECT c.id_paciente, a.especialidad FROM citas c "
                     "JOIN huecos_agenda h USING (id_hueco) JOIN agendas a USING (id_agenda)")

    for pid in _pacientes(con, f"SELECT DISTINCT id_paciente FROM ({citas_con_esp})",
                          semilla, usados, REPARTO["especialidad_si"]):
        esp = con.execute(f"SELECT min(especialidad) FROM ({citas_con_esp}) WHERE id_paciente = ?",
                          [pid]).fetchone()[0]
        casos.append({"tipo": "especialidad_si", "paciente": pid, "esp": ESPECIALIDADES[esp]})

    for pid in _pacientes(con, f"SELECT DISTINCT id_paciente FROM ({citas_con_esp})",
                          semilla, usados, REPARTO["especialidad_no"]):
        tiene = {r[0] for r in con.execute(
            f"SELECT especialidad FROM ({citas_con_esp}) WHERE id_paciente = ?", [pid]).fetchall()}
        esp = sorted(set(ESPECIALIDADES) - tiene)[0]
        casos.append({"tipo": "especialidad_no", "paciente": pid, "esp": ESPECIALIDADES[esp]})

    for pid in _pacientes(con, "SELECT DISTINCT id_paciente FROM pruebas WHERE estado_prueba = 'PROGRAMADA'",
                          semilla, usados, REPARTO["pendientes"] - 1):
        casos.append({"tipo": "pendientes", "paciente": pid})
    for pid in _pacientes(con, "SELECT id_paciente FROM pruebas GROUP BY 1 "
                               "HAVING count(*) FILTER (WHERE estado_prueba = 'PROGRAMADA') = 0",
                          semilla, usados, 1):
        casos.append({"tipo": "pendientes", "paciente": pid})

    for valor in ["INTERPRETE", "MOVILIDAD_REDUCIDA", "AUDITIVA", "NINGUNA"][:REPARTO["accesibilidad"]]:
        for pid in _pacientes(con, f"SELECT id_paciente FROM pacientes "
                                   f"WHERE necesidades_accesibilidad = '{valor}'", semilla, usados, 1):
            casos.append({"tipo": "accesibilidad", "paciente": pid})

    for pid in _pacientes(con, "SELECT DISTINCT id_paciente FROM derivaciones_ap",
                          semilla, usados, REPARTO["motivo"]):
        casos.append({"tipo": "motivo", "paciente": pid})

    cambios = ("SELECT DISTINCT c.id_paciente FROM movimientos_citas m JOIN citas c USING (id_cita) "
               "WHERE m.tipo_movimiento IN ('REPROGRAMACION', 'ANULACION') OR m.estado_nuevo = 'NO_REALIZADA'")
    for tipo, n in [("REPROGRAMACION", 1), ("ANULACION", 1)]:
        for pid in _pacientes(con, f"SELECT DISTINCT c.id_paciente FROM movimientos_citas m "
                                   f"JOIN citas c USING (id_cita) WHERE m.tipo_movimiento = '{tipo}'",
                              semilla, usados, n):
            casos.append({"tipo": "cambios", "paciente": pid})
    for pid in _pacientes(con, cambios, semilla, usados, REPARTO["cambios"] - 3):
        casos.append({"tipo": "cambios", "paciente": pid})
    for pid in _pacientes(con, f"SELECT DISTINCT id_paciente FROM citas WHERE id_paciente NOT IN ({cambios})",
                          semilla, usados, 1):
        casos.append({"tipo": "cambios", "paciente": pid})

    for pid in _pacientes(con, "SELECT id_paciente FROM pacientes", semilla, usados, REPARTO["contacto"]):
        casos.append({"tipo": "contacto", "paciente": pid})

    for pregunta, pid in zip(TRAMPAS, _pacientes(con, "SELECT DISTINCT id_paciente FROM citas",
                                                 semilla, usados, REPARTO["trampa"])):
        casos.append({"tipo": "trampa", "paciente": pid, "pregunta": pregunta})

    for caso in casos:
        caso.setdefault("pregunta", PREGUNTAS.get(caso["tipo"], "").format(esp=caso.get("esp")))
    return casos


# ---------------------------------------------------------------- respuesta correcta

def _clave_motivo(motivo: str) -> str:
    """La palabra que tiene que salir en la respuesta para dar el motivo por bueno."""
    if motivo == "Cita no realizada":
        return r"no (se )?(realiz|acud|asist|present)|falt|atend"
    if motivo in ("Paciente cancela", "Petición del paciente"):
        return r"cancel|peticion del paciente|lo pidio"
    palabras = [normal(p) for p in motivo.split() if normal(p) not in ("paciente", "cita", "del", "de")]
    return max(palabras, key=len)[:6]           # «Paciente cancela» → «cancel», vale para «canceló»

def esperado(caso: dict, filas) -> dict:
    """Qué tiene que contestar y qué filas tiene que citar, sacado de las filas."""
    t = caso["tipo"]
    if t in ("especialidad_si", "especialidad_no"):
        refs = [f.ref for f in filas if f.tabla == "CITAS" and f.texto.startswith(f"Cita en {caso['esp']} ")]
        return {"refs": refs, "texto": f"{'Sí' if refs else 'No'}: {len(refs)} cita(s) en {caso['esp']}"}
    if t == "pendientes":
        pend = [f for f in filas if f.tabla == "PRUEBAS" and "estado PROGRAMADA" in f.texto]
        tipos = [re.match(r"Prueba (\w+)", f.texto).group(1) for f in pend]
        return {"refs": [f.ref for f in pend], "claves": [normal(x) for x in tipos],
                "texto": ", ".join(tipos) or "ninguna"}
    if t == "accesibilidad":
        valor = re.search(r"accesibilidad: (\w+)", filas[0].texto).group(1)
        return {"refs": ["F1"] if valor != "NINGUNA" else [],
                "claves": [CLAVES_ACCESIBILIDAD[valor]] if valor != "NINGUNA" else [], "texto": valor}
    if t == "contacto":
        valor = re.search(r"Contacto preferido: (\w+)", filas[0].texto).group(1)
        return {"refs": ["F1"], "claves": [CLAVES_CONTACTO[valor]], "texto": valor}
    if t == "motivo":
        der = [f for f in filas if f.tabla == "DERIVACIONES_AP" and not f.campos.get("vacia")]
        motivos = [re.search(r"Motivo: ([^.]+)\.", f.texto).group(1) for f in der]
        # Vale el motivo («Revisión») o lo que cuenta el médico en las observaciones.
        palabras = {p for f in der for p in re.findall(r"[a-z]{6,}", normal(f.texto))} - GENERICAS
        return {"refs": [f.ref for f in der], "claves": [normal(m.split()[-1]) for m in motivos],
                "palabras": sorted(palabras), "texto": " / ".join(motivos)}
    if t == "cambios":
        mov = [f for f in filas if f.tabla == "MOVIMIENTOS_CITAS" and not f.campos.get("vacia")]
        motivos = [re.search(r"Motivo: ([^.]+)\.", f.texto).group(1) for f in mov]
        return {"refs": [f.ref for f in mov], "claves": [_clave_motivo(m) for m in motivos],
                "hay_anulacion": any("fue anulada" in f.texto for f in mov),
                "texto": " / ".join(motivos) or "ninguno"}
    return {"refs": [], "texto": "No consta"}


def corregir(caso: dict, esp: dict, r) -> tuple[bool, str]:
    t, texto, citadas = caso["tipo"], normal(r.respuesta), set(r.fuentes)
    niega = bool(re.search(NEGACION, texto)) or not r.consta

    if t == "trampa":
        return (not r.consta and not r.fuentes, "dice no consta" if not r.consta else "contesta algo que no está")
    if r.inventadas:
        return False, f"cita filas que no existen: {r.inventadas}"
    if not esp["refs"]:                                     # lo correcto es «no hay»
        return (niega, "dice que no hay" if niega else "afirma algo que no está en los datos")
    if t == "especialidad_si":
        ok = bool(citadas & set(esp["refs"])) and not texto.startswith("no")
        return ok, "cita la consulta" if ok else "no cita ninguna cita de esa especialidad"
    if t == "pendientes":
        faltan = set(esp["refs"]) - citadas
        sin_nombrar = [c for c in esp["claves"] if c not in texto]
        ok = not faltan and not sin_nombrar
        return ok, "todas" if ok else f"faltan {sorted(faltan) or sin_nombrar}"
    if t in ("accesibilidad", "contacto"):
        ok = "F1" in citadas and any(re.search(c, texto) for c in esp["claves"])
        return ok, "correcto" if ok else "no da el dato o no cita la ficha"
    if t in ("motivo", "cambios"):
        cita = bool(citadas & set(esp["refs"]))
        nombra = any(re.search(c, texto) for c in esp["claves"])
        if t == "motivo" and not nombra:                    # o al menos 2 palabras de las observaciones
            nombra = len(set(re.findall(r"[a-z]{6,}", texto)) & set(esp["palabras"])) >= 2
        # «fue anulada», «se han anulado»; no «no es una anulación»
        if t == "cambios" and not esp["hay_anulacion"] and re.search(r"\banulad[ao]s?\b", texto):
            return False, "dice «anulada» y ninguna cita se anuló"
        return (cita and nombra, "correcto" if cita and nombra else
                ("no cita la fila" if not cita else "cita la fila pero no da el motivo"))
    return False, "tipo desconocido"


# ---------------------------------------------------------------- ejecutar

def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--url", default=URL, help=f"servidor compatible con OpenAI (por defecto {URL})")
    ap.add_argument("--modelo", default=MODELO, help=f"por defecto {MODELO}")
    ap.add_argument("--semilla", type=int, default=42)
    ap.add_argument("--pensar", action="store_true", help="deja que Qwen3 razone antes (más lento)")
    ap.add_argument("--solo-preguntas", action="store_true", help="no llama al modelo")
    args = ap.parse_args()

    con = cargar()
    casos = generar_casos(con, args.semilla)
    resultados = []
    if not args.solo_preguntas:
        print(f"Cargando {args.modelo} (la primera llamada no cuenta para el tiempo)…")
        responder(datos_del_paciente(con, casos[0]["paciente"]), "Hola", args.url, args.modelo)
    for n, caso in enumerate(casos, 1):
        filas = datos_del_paciente(con, caso["paciente"])
        esp = esperado(caso, filas)
        print(f"\n{n:2}. [{caso['tipo']}] {caso['paciente']} · {len(filas)} filas · {caso['pregunta']}")
        print(f"    correcto: {esp['texto']}  {esp['refs']}")
        if args.solo_preguntas:
            continue
        try:
            r = responder(filas, caso["pregunta"], args.url, args.modelo, sin_pensar=not args.pensar)
            ok, motivo = corregir(caso, esp, r)
        except Exception as e:                      # se cuenta como fallo y se sigue
            r = Respuesta(False, f"ERROR: {e}", [], segundos=120.0)
            ok, motivo = False, "sin respuesta"
        print(f"    Medicci ({r.segundos:.1f} s): {r.respuesta}  {r.fuentes}")
        print(f"    {'✅' if ok else '❌'} {motivo}")
        resultados.append({**caso, "filas": len(filas), "esperado": esp, "respuesta": r.respuesta,
                           "consta": r.consta, "fuentes": r.fuentes, "inventadas": r.inventadas,
                           "segundos": round(r.segundos, 2), "acierto": ok, "motivo": motivo,
                           "bruto": r.bruto})

    if args.solo_preguntas:
        return

    print(f"\n=== {args.modelo} · semilla {args.semilla} ===")
    for tipo in REPARTO:
        del_tipo = [x for x in resultados if x["tipo"] == tipo]
        if del_tipo:
            print(f"  {tipo:16} {sum(x['acierto'] for x in del_tipo)}/{len(del_tipo)}")
    aciertos = sum(x["acierto"] for x in resultados)
    tiempos = [x["segundos"] for x in resultados]
    trampas = [x for x in resultados if x["tipo"] == "trampa"]
    print(f"  {'TOTAL':16} {aciertos}/{len(resultados)}")
    print(f"  «no consta» cuando toca: {sum(x['acierto'] for x in trampas)}/{len(trampas)}")
    print(f"  filas inventadas citadas: {sum(len(x['inventadas']) for x in resultados)}")
    print(f"  segundos por respuesta: mediana {statistics.median(tiempos):.1f}, máximo {max(tiempos):.1f}")

    carpeta = Path(__file__).resolve().parent / "resultados"
    carpeta.mkdir(exist_ok=True)
    nombre = re.sub(r"[^\w.-]", "_", args.modelo)
    destino = carpeta / f"{datetime.now():%Y%m%d-%H%M%S}-{nombre}-semilla{args.semilla}.json"
    destino.write_text(json.dumps({"modelo": args.modelo, "url": args.url, "semilla": args.semilla,
                                   "aciertos": aciertos, "total": len(resultados),
                                   "casos": resultados}, ensure_ascii=False, indent=2, default=str),
                       encoding="utf-8")
    print(f"\nDetalle: {destino}")


if __name__ == "__main__":
    main()
