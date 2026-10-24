"""Las cinco cosas más importantes de Medicci, cada una en un GIF corto con su
rótulo, para enseñarlas sin la demo en marcha.

    python mvp/gifs.py                       # con la demo en marcha; deja los GIF en el Escritorio
    python mvp/gifs.py --salida C:/ruta

Como simular.py: HCIS a pantalla completa (?simulacion=libre, que obedece a
/api/simulacion) y la burbuja (--simulacion) a nombre de med.demo01. Graba la
pantalla tal como se ve (ddagrab: con las transparencias de verdad) y pasa
cada grabación a GIF.
"""

import argparse
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from simular import (API, CHROME, MEDICO, abre_burbuja, al_frente, aparta_ventana, cierra_burbuja, ffmpeg,
                     limpia, pantalla, paso, pide)

ESCENAS = [
    ("1-sigue-al-paciente", [
        ("rotulo", "1 · Sigue al paciente|El médico abre a María en HCIS, como siempre.", 3),
        ("paciente", "P0129798", 3),
        ("rotulo", "1 · Sigue al paciente|Medicci lo ve y ya tiene su resumen: los avisos, cada uno con su fuente.", 0.5),
        ("abrir", "", 7),
    ]),
    ("2-pregunta-con-fuente", [
        ("rotulo", "2 · Pregunta y fuente|Pregunta como lo diría en voz alta…", 1),
        ("preguntar", "¿Qué pruebas tiene pendientes?", 7),
        ("rotulo", "2 · Pregunta y fuente|…contesta una IA local, en este ordenador, y cita la fila de la que sale.", 5),
    ]),
    ("3-resumen-ampliado", [
        ("rotulo", "3 · Resumen ampliado|Todo lo de María en una ventana: avisos, historia, pruebas y lo que no consta.", 1),
        ("ampliar", "", 7),
        ("cerrar_ampliar", "", 1.5),
    ]),
    ("4-resultado-por-hl7", [
        ("rotulo", "4 · En directo por HL7|Ya en consulta, HCIS manda el resultado de la radiografía…", 3),
        ("evento", "P0129798", 2),
        ("rotulo", "4 · En directo por HL7|…y Medicci lo enseña en el momento, marcado como «Nuevo».", 6),
    ]),
    ("5-registro-de-accesos", [
        ("plegar", "", 1),
        ("ir", f"/registro.html?simulacion=libre&medico={MEDICO}", 3),
        ("rotulo", "5 · Registro de accesos|Todo lo que ha consultado el médico queda a su nombre…", 4.5),
        ("rotulo", "5 · Registro de accesos|…encadenado: si alguien borra o cambia una línea, se nota.", 4.5),
    ]),
]


def a_gif(f: str, video: Path, gif: Path) -> None:
    filtro = ("fps=12,scale=1280:-1:flags=lanczos,split[a][b];"
              "[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer:bayer_scale=4")
    subprocess.run([f, "-y", "-loglevel", "error", "-i", str(video), "-vf", filtro, str(gif)], check=True)


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--salida", type=Path, default=Path.home() / "Desktop/Medicci-gifs")
    ap.add_argument("--sin-limpiar", action="store_true")
    args = ap.parse_args()
    if not (f := ffmpeg()):
        sys.exit("No encuentro ffmpeg (winget install Gyan.FFmpeg).")
    args.salida.mkdir(parents=True, exist_ok=True)
    import ctypes
    ctypes.windll.kernel32.SetThreadExecutionState(0x80000000 | 0x1 | 0x2)   # sin bloquear la pantalla
    if not args.sin_limpiar:
        limpia()
    cierra_burbuja()

    perfil = tempfile.mkdtemp(prefix="medicci-simulacion-")
    tmp = Path(tempfile.mkdtemp(prefix="medicci-gifs-"))
    chrome = subprocess.Popen([str(CHROME), f"--user-data-dir={perfil}", "--kiosk", "--no-first-run",
                               "--no-default-browser-check", "--disable-features=Translate",
                               "--window-position=0,0", f"{API}/?simulacion=libre"])
    consola = burbuja = None
    try:
        time.sleep(4)
        consola = aparta_ventana()
        time.sleep(1)
        al_frente("· simulación")
        burbuja = abre_burbuja("--simulacion", medico=MEDICO)
        time.sleep(5)
        ancho, alto = pantalla()
        for nombre, pasos in ESCENAS:
            video = tmp / f"{nombre}.mp4"
            graba = subprocess.Popen(
                [f, "-y", "-loglevel", "error", "-f", "lavfi", "-i", "ddagrab=output_idx=0:framerate=24",
                 "-vf", f"hwdownload,format=bgra,crop={ancho}:{alto}:0:0,format=yuv420p",
                 "-c:v", "libx264", "-preset", "veryfast", "-crf", "18", str(video)], stdin=subprocess.PIPE)
            time.sleep(1)
            for accion, texto, espera in pasos:
                if accion == "evento":
                    pide("/api/simulacion/evento", {"nhc": texto, "tipo": "resultado"})
                else:
                    paso(accion, texto)
                time.sleep(espera)
            graba.communicate(b"q", timeout=30)
            a_gif(f, video, args.salida / f"{nombre}.gif")
            print(f"   {nombre}.gif", flush=True)
            paso("sin_rotulo")
            time.sleep(1)
    finally:
        chrome.terminate()
        subprocess.run(["taskkill", "/PID", str(chrome.pid), "/T", "/F"], capture_output=True)
        if burbuja:
            burbuja.terminate()
        shutil.rmtree(perfil, ignore_errors=True)
        shutil.rmtree(tmp, ignore_errors=True)
        if consola:
            ctypes.windll.user32.ShowWindow(consola, 9)
        abre_burbuja()
    print(f"GIF en {args.salida}", flush=True)


if __name__ == "__main__":
    main()
