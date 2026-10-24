"""Una sola llamada al modelo local: ordena las filas y contesta citándolas.

Habla con cualquier servidor compatible con OpenAI (Ollama, vLLM, llama.cpp)
usando solo la biblioteca estándar. Con MEDICCI_URL=claude, en un equipo sin
GPU, la llamada la hace el CLI de Claude (`claude -p`) con la sesión de quien
arranca el servidor.
"""

import json
import os
import re
import shutil
import subprocess
import time
import urllib.error
import urllib.request
from dataclasses import dataclass, field

from config import valor
from paciente import Fila, formatear
from seleccion import seleccionar

# Adonde diga el hospital o el pliego: medicci.conf o variables de entorno.
URL = valor("MEDICCI_URL", "http://localhost:11434/v1")
MODELO = valor("MEDICCI_MODELO", "qwen3:8b")
CLAVE = valor("MEDICCI_CLAVE", "local")      # Ollama no pide clave; otros servidores, sí

SISTEMA = """Eres Medicci, el asistente del médico en consultas externas.
Respondes preguntas sobre UN paciente usando SOLO las filas de datos que se te dan.
Cada fila empieza con su referencia entre corchetes, por ejemplo [C2].

Reglas:
1. Usa solo esas filas. No uses conocimiento general ni supongas nada: no deduzcas
   diagnósticos, causas ni fechas que no estén escritos tal cual en una fila.
2. Pon en "fuentes" las referencias de todas las filas en las que te basas.
3. Si la pregunta trata de algo que no está en ninguna fila (alergias, diagnósticos,
   medicación, antecedentes, hábitos o el valor de un resultado, si no hay informes
   que lo digan), responde "consta": false, "respuesta": "No consta en los datos
   disponibles." y "fuentes": [].
   Si solo aparece en el texto que escribió el médico de Primaria (las
   observaciones de una derivación), cítalo entre comillas diciendo quién lo
   escribió, y no lo conviertas en un sí o un no: «Su médico de cabecera escribió
   "…" en la derivación del 12/10/2026.»
4. Si preguntan si hay algo que las filas sí recogen (pruebas pendientes, cambios
   de cita, citas en un servicio…) y no lo hay, dilo directamente, por ejemplo
   «No tiene pruebas pendientes», sin inventar.
5. Responde en español con UNA frase corta (como mucho 25 palabras) que dé justo lo
   que el médico busca, por ejemplo «Sí: una ecografía abdominal, programada para
   el 3 de marzo y sin hacer.» No repitas estados técnicos, fechas de
   solicitud ni lo que no se pregunta: el médico ve el texto completo en las fuentes.
6. Lo que salga de un informe clínico (las filas [I…]), dilo como lo dice el informe
   y di cuál es y de qué fecha: «El alta de Urgencias del 4/5/2022 recoge una
   neumonía basal derecha.» Si dos filas se contradicen (la derivación
   dice una cosa y un informe otra), di las dos, sin elegir.
7. Las pruebas vienen por tipo (ENDOSCOPIA, ANALITICA…). Si preguntan por una más
   concreta (colonoscopia, gastroscopia…), di la que consta con su tipo, sin decir
   que es esa: «Consta una endoscopia programada para el 12/11/2026; no dice de qué tipo.»

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
    usadas: list = field(default_factory=list)              # las filas que vio el modelo


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


def es_cli(url: str) -> bool:
    """MEDICCI_URL=claude: el modelo es el CLI de Claude, no un servidor."""
    return url.strip().lower() == "claude"


def _llamar_cli(mensajes: list[dict], modelo: str) -> str:
    """La misma llamada con `claude -p`: sin herramientas, sin MCP, sin los
    ajustes del usuario y con el mismo esquema JSON. Las filas salen del equipo
    hacia Anthropic: solo para probar con datos sintéticos."""
    claude = shutil.which("claude")
    if not claude:
        raise RuntimeError("No encuentro el CLI de Claude (claude) en el PATH.")
    # el modelo de Ollama (qwen3:8b) no le vale: por defecto, el más rápido
    if not re.match(r"(haiku|sonnet|opus|claude-)", modelo or ""):
        modelo = "haiku"
    orden = [claude, "-p", "--model", modelo, "--tools", "", "--strict-mcp-config", "--setting-sources", "",
             "--no-session-persistence", "--output-format", "json",
             "--system-prompt", mensajes[0]["content"], "--json-schema", json.dumps(ESQUEMA)]
    # si el servidor se arranca desde una sesión de Claude Code, que no se crea una subsesión
    entorno = {k: v for k, v in os.environ.items() if k != "CLAUDECODE" and not k.startswith("CLAUDE_CODE_")}
    r = subprocess.run(orden, input=mensajes[1]["content"].replace(" /no_think", ""),
                       capture_output=True, text=True, encoding="utf-8", timeout=120, env=entorno)
    try:
        datos = json.loads(r.stdout)
    except json.JSONDecodeError:
        raise RuntimeError(f"claude -p no ha contestado: {(r.stderr or r.stdout)[-200:]}")
    if datos.get("is_error"):
        raise RuntimeError(f"claude -p: {datos.get('result')}")
    salida = datos.get("structured_output")
    return json.dumps(salida, ensure_ascii=False) if salida else datos.get("result") or ""


def _leer_json(texto: str) -> dict:
    texto = re.sub(r"<think>.*?</think>", "", texto, flags=re.S)
    trozo = re.search(r"\{.*\}", texto, flags=re.S)
    return json.loads(trozo.group(0)) if trozo else {}


def responder(filas: list[Fila], pregunta: str, url: str = URL, modelo: str = MODELO,
              sin_pensar: bool = True) -> Respuesta:
    # un crónico no cabe entero en un modelo local pequeño: lo que hace falta.
    # Al CLI de Claude le cabe toda la historia.
    filas = seleccionar(filas, pregunta, **({"contexto": 200_000} if es_cli(url) else {}))
    usuario = f"Datos del paciente:\n{formatear(filas)}\n\nPregunta: {pregunta}"
    if sin_pensar:
        usuario += " /no_think"          # Qwen3: contesta sin razonar en voz alta (más rápido)
    mensajes = [{"role": "system", "content": SISTEMA}, {"role": "user", "content": usuario}]

    inicio = time.perf_counter()
    if es_cli(url):
        bruto = _llamar_cli(mensajes, modelo)
    else:
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
    return Respuesta(consta, texto, fuentes, inventadas, segundos, bruto, filas)
