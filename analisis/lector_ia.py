"""Prueba del lector de derivaciones con un modelo local (Ollama).

Lee una muestra de DERIVACIONES_AP, extrae datos estructurados del texto libre
del médico de Primaria y compara el tipo de derivación con `motivo_derivacion`.

Uso: python analisis/lector_ia.py [n_por_motivo] [modelo]
"""

import json
import sys
import time
import urllib.request
from collections import Counter

from datos import conectar

N = int(sys.argv[1]) if len(sys.argv) > 1 else 20
MODELO = sys.argv[2] if len(sys.argv) > 2 else "qwen3.5:2b"
OLLAMA = "http://localhost:11434/api/chat"

TIPOS = {
    "Valoración especializada": "NUEVA",
    "Revisión": "REVISION",
    "Reanudar seguimiento": "REANUDAR_SEGUIMIENTO",
}

ESQUEMA = {
    "type": "object",
    "properties": {
        "tipo": {"type": "string", "enum": list(TIPOS.values())},
        "edad": {"type": ["integer", "null"]},
        "sexo": {"type": ["string", "null"], "enum": ["F", "M", None]},
        "antecedentes": {"type": "array", "items": {"type": "string"}},
        "meses_evolucion": {"type": ["integer", "null"]},
        "pruebas_previas_en_ap": {"type": "boolean"},
        "datos_que_faltan": {"type": "array", "items": {"type": "string"}},
        "resumen": {"type": "string"},
    },
    "required": ["tipo", "edad", "sexo", "antecedentes", "meses_evolucion",
                 "pruebas_previas_en_ap", "datos_que_faltan", "resumen"],
}

INSTRUCCIONES = """Eres un asistente administrativo de consultas externas de un hospital.
Lees la derivación que envía el médico de Atención Primaria y extraes datos. No diagnosticas ni recomiendas tratamientos.

tipo:
- NUEVA: problema nuevo que se deriva para valoración.
- REVISION: paciente ya valorado antes por el servicio que ha empeorado.
- REANUDAR_SEGUIMIENTO: paciente en seguimiento que perdió su cita de revisión y pide una nueva.
edad, sexo, meses_evolucion: solo si aparecen en el texto; si no, null.
antecedentes: siglas o enfermedades previas que cite (por ejemplo HTA, DL, hipotiroidismo).
pruebas_previas_en_ap: true si dice que ya se hicieron pruebas en Primaria.
datos_que_faltan: lo que falta para que el especialista valore bien (edad, tiempo de evolución, pruebas hechas, motivo concreto...).
resumen: una frase para el especialista."""


def leer(texto: str) -> dict:
    cuerpo = {
        "model": MODELO,
        "messages": [
            {"role": "system", "content": INSTRUCCIONES},
            {"role": "user", "content": texto},
        ],
        "format": ESQUEMA,
        "think": False,
        "stream": False,
        "options": {"temperature": 0},
    }
    req = urllib.request.Request(OLLAMA, json.dumps(cuerpo).encode(), {"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as r:
        return json.loads(json.loads(r.read())["message"]["content"])


con = conectar()
muestra = con.execute(f"""
    SELECT id_derivacion, motivo_derivacion, observaciones_medico FROM (
        SELECT *, row_number() OVER (PARTITION BY motivo_derivacion ORDER BY hash(id_derivacion)) r
        FROM derivaciones_ap) WHERE r <= {N}
""").fetchall()

aciertos, confusion, tiempos = 0, Counter(), []
leer("Prueba de arranque.")  # carga el modelo en memoria antes de medir
for i, (id_d, motivo, texto) in enumerate(muestra):
    t0 = time.perf_counter()
    res = leer(texto)
    tiempos.append(time.perf_counter() - t0)
    esperado = TIPOS[motivo]
    aciertos += res["tipo"] == esperado
    confusion[(esperado, res["tipo"])] += 1
    if i < 3 or res["tipo"] != esperado:
        print(f"\n{id_d} · esperado {esperado} · leído {res['tipo']}\n  {texto}\n  → {json.dumps(res, ensure_ascii=False)}")

print(f"\nModelo {MODELO}: {aciertos}/{len(muestra)} aciertos en el tipo "
      f"({aciertos / len(muestra):.0%}); {sum(tiempos) / len(tiempos):.1f} s de media por derivación")
for (e, l), n in sorted(confusion.items()):
    print(f"  {e:22} → {l:22} {n}")
