"""Una sola llamada al modelo local: ordena las filas y contesta citándolas.

Habla con cualquier servidor compatible con OpenAI (Ollama, vLLM, llama.cpp)
usando solo la biblioteca estándar.
"""

import json
import os
import re
import time
import urllib.error
import urllib.request
from dataclasses import dataclass, field

from paciente import Fila, formatear

from config import valor

# Adonde diga el hospital o el pliego: medici.conf o variables de entorno.
URL = valor("MEDICI_URL", "http://localhost:11434/v1")
MODELO = valor("MEDICI_MODELO", "qwen3:8b")
CLAVE = valor("MEDICI_CLAVE", "local")      # Ollama no pide clave; otros servidores, sí

SISTEMA = """Eres Medici, el asistente del médico en consultas externas.
Respondes preguntas sobre UN paciente usando SOLO las filas de datos que se te dan.
Cada fila empieza con su referencia entre corchetes, por ejemplo [C2].

Reglas:
1. Usa solo esas filas. No uses conocimiento general ni supongas nada.
2. Pon en "fuentes" las referencias de todas las filas en las que te basas.
3. Si la pregunta trata de algo que estas filas no recogen (alergias, diagnósticos,
   medicación, antecedentes, hábitos o el valor de un resultado), responde
   "consta": false, "respuesta": "No consta en los datos disponibles." y "fuentes": [].
4. Si preguntan si hay algo que las filas sí recogen (pruebas pendientes, cambios
   de cita, citas en un servicio…) y no lo hay, dilo directamente, por ejemplo
   «No tiene pruebas pendientes», sin inventar.
5. Responde en español, en 1 a 3 frases.

Devuelve solo JSON: {"consta": true o false, "respuesta": "...", "fuentes": ["C1", ...]}"""

ESQUEMA = {
    "type": "object",
    "properties": {
        "consta": {"type": "boolean"},
        "respuesta": {"type": "string"},
        "fuentes": {"type": "array", "items": {"type": "string"}},
    },
    "required": ["consta", "respuesta", "fuentes"],
}


@dataclass
class Respuesta:
    consta: bool
    respuesta: str
    fuentes: list[str]                                      # solo las que existen
    inventadas: list[str] = field(default_factory=list)     # citadas pero inexistentes
    segundos: float = 0.0
    bruto: str = ""


def _llamar(mensajes: list[dict], formato: dict | None, url: str, modelo: str,
            sin_pensar: bool = True) -> str:
    # max_tokens: un tope para que una respuesta atascada no bloquee la consulta
    cuerpo = {"model": modelo, "messages": mensajes, "temperature": 0, "max_tokens": 400}
    if sin_pensar:
        # Ollama 0.40 ignora «/no_think»: Qwen3 razona por dentro, gasta los 400
        # tokens y devuelve la respuesta vacía. Así se apaga de verdad.
        cuerpo["reasoning_effort"] = "none"
    if formato:
        cuerpo["response_format"] = formato
    peticion = urllib.request.Request(
        f"{url.rstrip('/')}/chat/completions",
        data=json.dumps(cuerpo).encode(),
        headers={"Content-Type": "application/json", "Authorization": f"Bearer {CLAVE}"},
    )
    with urllib.request.urlopen(peticion, timeout=120) as r:
        return json.load(r)["choices"][0]["message"]["content"] or ""


def _leer_json(texto: str) -> dict:
    texto = re.sub(r"<think>.*?</think>", "", texto, flags=re.S)
    trozo = re.search(r"\{.*\}", texto, flags=re.S)
    return json.loads(trozo.group(0)) if trozo else {}


def responder(filas: list[Fila], pregunta: str, url: str = URL, modelo: str = MODELO,
              sin_pensar: bool = True) -> Respuesta:
    usuario = f"Datos del paciente:\n{formatear(filas)}\n\nPregunta: {pregunta}"
    if sin_pensar:
        usuario += " /no_think"          # Qwen3: contesta sin razonar en voz alta (más rápido)
    mensajes = [{"role": "system", "content": SISTEMA}, {"role": "user", "content": usuario}]

    inicio = time.perf_counter()
    try:
        bruto = _llamar(mensajes, {"type": "json_schema",
                                   "json_schema": {"name": "respuesta", "schema": ESQUEMA}},
                        url, modelo, sin_pensar)
    except urllib.error.HTTPError:
        bruto = _llamar(mensajes, {"type": "json_object"}, url, modelo, sin_pensar)   # servidores más viejos
    segundos = time.perf_counter() - inicio

    try:
        datos = _leer_json(bruto)
    except json.JSONDecodeError:
        datos = {}
    validas = {f.ref for f in filas}
    citadas = [str(r).strip("[] ") for r in datos.get("fuentes", [])]
    fuentes = [r for r in dict.fromkeys(citadas) if r in validas]
    inventadas = [r for r in citadas if r not in validas]

    consta = bool(datos.get("consta", False)) and bool(fuentes)
    texto = datos.get("respuesta") or "No consta en los datos disponibles."
    if not consta and not fuentes:
        texto = texto if re.search(r"no (consta|aparece)", texto, re.I) else "No consta en los datos disponibles."
    return Respuesta(consta, texto, fuentes, inventadas, segundos, bruto)
