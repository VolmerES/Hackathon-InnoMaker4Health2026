"""Dictado: de la voz del médico al texto de la pregunta, con Whisper en local.

La burbuja graba solo mientras el médico mantiene pulsado el micrófono y manda
el audio aquí. Se transcribe en memoria y no se guarda nunca: ni en disco ni en
el registro, donde queda solo el texto. El texto vuelve a la burbuja, el médico
lo revisa y la pregunta sigue el camino de siempre: el modelo no oye nada.

Usa faster-whisper (Whisper con CTranslate2, licencia MIT). Se elige en
medicci.conf o con variables de entorno:

    MEDICCI_VOZ_MODELO       small (por defecto), medium, large-v3-turbo… o la ruta
                             de un modelo ya descargado, para un servidor sin internet
    MEDICCI_VOZ_DISPOSITIVO  cpu (por defecto) o cuda
    MEDICCI_VOZ_CALCULO      int8 (por defecto en cpu) o float16 (en cuda)

Por defecto va en el procesador: la VRAM se queda para el modelo de lenguaje.
La primera vez descarga el modelo de Whisper (small, unos 500 MB).
"""

import io
import threading
import time

from config import valor

MODELO = valor("MEDICCI_VOZ_MODELO", "small")
DISPOSITIVO = valor("MEDICCI_VOZ_DISPOSITIVO", "cpu")
CALCULO = valor("MEDICCI_VOZ_CALCULO", "int8" if DISPOSITIVO == "cpu" else "float16")
MAX_BYTES = 8 * 1024 * 1024          # un minuto de audio de sobra

_modelo = None
_cerrojo = threading.Lock()          # un modelo cargado y una transcripción a la vez


def disponible() -> bool:
    try:
        import faster_whisper  # noqa: F401
    except ImportError:
        return False
    return True


def transcribir(audio: bytes) -> tuple[str, float]:
    """El texto dictado y los segundos que ha tardado. El audio no sale de la memoria."""
    from faster_whisper import WhisperModel
    global _modelo
    with _cerrojo:
        if _modelo is None:
            print(f"Cargando Whisper {MODELO} en {DISPOSITIVO} ({CALCULO})…", flush=True)
            _modelo = WhisperModel(MODELO, device=DISPOSITIVO, compute_type=CALCULO)
        inicio = time.perf_counter()
        # vad_filter quita los silencios del principio y del final, donde Whisper
        # tiende a inventarse frases
        segmentos, _ = _modelo.transcribe(io.BytesIO(audio), language="es", vad_filter=True)
        texto = " ".join(s.text.strip() for s in segmentos).strip()
    return texto, time.perf_counter() - inicio
