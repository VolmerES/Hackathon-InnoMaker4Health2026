"""La mañana de un médico con Medicci, de principio a fin, grabada en vídeo.

Abre HCIS a pantalla completa en un Chrome aparte (?simulacion=1, que pone
los rótulos y enseña el arranque del ordenador), abre la burbuja nativa
cuando «arranca Windows» y lo graba todo con ffmpeg. La burbuja va a nombre
de med.demo01, sigue al paciente por el título de HCIS y hace lo que toca en
cada paso (web/simulacion.js).

Se corre en Windows, con la demo ya en marcha (arrancar.bat):

    python mvp/simular.py                        # deja la historia como al principio y graba
    python mvp/simular.py --sin-video            # solo la simulación, para ensayar
    python mvp/simular.py --salida C:/ruta/video.mp4

Antes vuelve a mandar la historia por HL7 (--sin-limpiar para no hacerlo):
el resultado que llega en directo cambia a María, y sin limpiar la segunda
vez ya no tendría la prueba pendiente.
"""

import argparse
import ctypes
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
API = "http://localhost:8000"
MEDICO = "med.demo01"
CHROME = Path(os.environ.get("PROGRAMFILES", r"C:\Program Files")) / "Google/Chrome/Application/chrome.exe"
BURBUJA = RAIZ / "medicci-rs/target/release/medicci.exe"


def pide(ruta: str) -> dict:
    with urllib.request.urlopen(API + ruta, timeout=10) as r:
        return json.load(r)


def pantalla() -> tuple[int, int]:
    """El tamaño de verdad del monitor principal, en píxeles (sin el escalado)."""
    ctypes.windll.user32.SetProcessDPIAware()
    return ctypes.windll.user32.GetSystemMetrics(0), ctypes.windll.user32.GetSystemMetrics(1)


def paso(accion: str) -> int:
    peticion = urllib.request.Request(API + "/api/simulacion/paso", data=json.dumps({"accion": accion}).encode(),
                                      headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(peticion, timeout=10) as r:
        return json.load(r)["n"]


def aparta_ventana() -> int:
    """Minimiza la ventana de delante (la consola desde la que se lanza) para
    que se vea HCIS: Windows no deja que un programa de fondo se ponga delante."""
    h = ctypes.windll.user32.GetForegroundWindow()
    ctypes.windll.user32.ShowWindow(h, 6)            # SW_MINIMIZE
    return h


def al_frente(prefijo: str) -> bool:
    """Pone delante la ventana cuyo título empieza por `prefijo` (HCIS a
    pantalla completa), para que tape también la barra de tareas. Windows solo
    deja hacerlo si parece que se ha pulsado una tecla: se pulsa y suelta Alt."""
    u = ctypes.windll.user32
    encontrada = []

    @ctypes.WINFUNCTYPE(ctypes.c_bool, ctypes.c_void_p, ctypes.c_void_p)
    def una(h, _):
        texto = ctypes.create_unicode_buffer(256)
        u.GetWindowTextW(ctypes.c_void_p(h), texto, 256)
        if u.IsWindowVisible(ctypes.c_void_p(h)) and texto.value.startswith(prefijo):
            encontrada.append(h)
            return False
        return True

    u.EnumWindows(una, None)
    if not encontrada:
        return False
    u.keybd_event(0x12, 0, 0, 0)                     # Alt
    u.keybd_event(0x12, 0, 2, 0)
    return bool(u.SetForegroundWindow(ctypes.c_void_p(encontrada[0])))


def ffmpeg() -> str | None:
    enlaces = Path(os.environ.get("LOCALAPPDATA", "")) / "Microsoft/WinGet/Links/ffmpeg.exe"
    return shutil.which("ffmpeg") or (str(enlaces) if enlaces.exists() else None)


def cierra_burbuja() -> None:
    subprocess.run(["taskkill", "/IM", "medicci.exe", "/F"], capture_output=True)
    time.sleep(1)


def abre_burbuja(*args: str, medico: str | None = None) -> subprocess.Popen:
    entorno = dict(os.environ)
    if medico:
        entorno.update(USERNAME=medico, USER=medico)
    return subprocess.Popen([str(BURBUJA), "--servidor", API, *args], cwd=BURBUJA.parent.parent.parent,
                            env=entorno)


def limpia() -> None:
    """La copia que llega por HL7, otra vez desde cero (unos 90 s)."""
    print("Dejando la historia como al principio (HL7)…", flush=True)
    subprocess.run(["docker", "restart", "medicci-servidor"], check=True, capture_output=True)
    for _ in range(60):
        try:
            pide("/api/estado")
            break
        except OSError:
            time.sleep(1)
    entorno = dict(os.environ, MEDICCI_DATOS="hl7")
    r = subprocess.run(["docker", "compose", "-f", str(RAIZ / "mvp/docker-compose.yml"), "run", "--rm",
                        "--no-deps", "emisor", "--comprobar"], env=entorno, capture_output=True,
                       text=True, encoding="utf-8")
    print("   " + (r.stdout.strip().splitlines() or ["(sin salida)"])[-1], flush=True)


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--salida", type=Path, default=Path.home() / "Desktop/Medicci-video/medicci-manana-del-medico.mp4")
    ap.add_argument("--sin-video", action="store_true")
    ap.add_argument("--sin-limpiar", action="store_true")
    args = ap.parse_args()

    # que Windows no apague la pantalla ni se bloquee mientras graba: pasó, y
    # el vídeo salió con la pantalla de bloqueo (ES_CONTINUOUS | SYSTEM | DISPLAY)
    ctypes.windll.kernel32.SetThreadExecutionState(0x80000000 | 0x1 | 0x2)
    if not args.sin_limpiar:
        limpia()
    cierra_burbuja()

    # HCIS a pantalla completa, delante de todo; la página espera a «empezar»
    perfil = tempfile.mkdtemp(prefix="medicci-simulacion-")
    chrome = subprocess.Popen([str(CHROME), f"--user-data-dir={perfil}", "--kiosk", "--no-first-run",
                               "--no-default-browser-check", "--disable-features=Translate",
                               "--window-position=0,0", f"{API}/?simulacion=1"])
    time.sleep(4)
    consola = aparta_ventana()
    time.sleep(1)
    if not al_frente("HCIS"):
        print("   (no he podido poner HCIS delante: puede verse la barra de tareas)", flush=True)
    time.sleep(1)

    grabando = None
    if not args.sin_video:
        if not (f := ffmpeg()):
            sys.exit("No encuentro ffmpeg (winget install Gyan.FFmpeg) o use --sin-video.")
        ancho, alto = pantalla()
        args.salida.parent.mkdir(parents=True, exist_ok=True)
        grabando = subprocess.Popen(
            [f, "-y", "-loglevel", "error", "-f", "gdigrab", "-framerate", "30", "-offset_x", "0", "-offset_y", "0",
             "-video_size", f"{ancho}x{alto}", "-draw_mouse", "0", "-i", "desktop",
             "-c:v", "libx264", "-preset", "veryfast", "-crf", "20", "-pix_fmt", "yuv420p",
             "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2", str(args.salida)],
            stdin=subprocess.PIPE)
        print(f"Grabando {ancho}×{alto} en {args.salida}", flush=True)
        time.sleep(1)

    inicio_n = vistos = paso("empezar")
    print("Simulación en marcha…", flush=True)
    burbuja = None
    limite = time.monotonic() + 600
    try:
        while time.monotonic() < limite:
            s = pide("/api/simulacion")
            if s["n"] != vistos:
                vistos = s["n"]
                print(f"   paso {s['n'] - inicio_n}: {s['accion']} {s['texto']}", flush=True)
                if s["accion"] == "windows" and burbuja is None:
                    # como si arrancara con Windows: con el usuario del médico
                    burbuja = abre_burbuja("--simulacion", medico=MEDICO)
                if s["accion"] == "fin":
                    time.sleep(2)
                    break
            time.sleep(0.3)
    finally:
        if grabando:
            grabando.communicate(b"q", timeout=30)
            print(f"Vídeo: {args.salida}", flush=True)
        chrome.terminate()
        subprocess.run(["taskkill", "/PID", str(chrome.pid), "/T", "/F"], capture_output=True)
        if burbuja:
            burbuja.terminate()
        shutil.rmtree(perfil, ignore_errors=True)
        ctypes.windll.user32.ShowWindow(consola, 9)      # SW_RESTORE: la consola, como estaba
        abre_burbuja()                     # la de siempre, a nombre del usuario de Windows


if __name__ == "__main__":
    main()
