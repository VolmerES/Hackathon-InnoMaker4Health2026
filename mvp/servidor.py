"""Servidor de la demo: la réplica de HCIS, la pestaña Medici y su API.

    python mvp/servidor.py                 # http://localhost:8000
    python mvp/servidor.py --puerto 8080

Solo usa la biblioteca estándar (y duckdb). En producción sería FastAPI detrás
de Caddy; la API es la misma.

API de Medici (lo que llamaría la pestaña desde HCIS):
    POST /api/sesion                    {nhc}    → abre sesión atada a ese paciente
    GET  /api/sesion/<id>/tarjeta                → «3 cosas antes de que entre»
    POST /api/sesion/<id>/pregunta      {texto}  → respuesta con sus fuentes
    GET  /api/estado                             → si el modelo local responde

Lo que hace de HCIS en la demo (en el hospital es el propio HCIS):
    GET  /api/hcis/agenda                        → la agenda del día
    GET  /api/hcis/paciente/<nhc>                → lo que HCIS enseña del paciente

Con --datos hl7 arranca con la copia vacía y un receptor MLLP (puerto 2575):
los datos llegan como mensajes HL7 (mvp/hl7_emisor.py hace de HCIS).
    GET  /api/hl7/ultimos                        → los últimos mensajes recibidos
"""

import argparse
import json
import os
import sys
import threading
import time
import urllib.request
import uuid
from datetime import datetime
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from agenda import elegir_agenda
from alertas import tarjeta
from paciente import cargar, cargar_vacia, datos_del_paciente
from responder import MODELO, URL, responder

AQUI = Path(__file__).resolve().parent
WEB = AQUI / "web"
FUENTES = AQUI.parent / "medicci-rs" / "fuentes"     # Noto Sans, las mismas que la burbuja
REGISTRO = AQUI / "resultados" / "registro.jsonl"


class Medici:
    def __init__(self, datos: str = "csv"):
        print("Cargando los datos…" if datos == "csv" else "Copia vacía: esperando mensajes HL7…", flush=True)
        self.con = cargar() if datos == "csv" else cargar_vacia()
        self.cerrojo = threading.Lock()       # DuckDB: una consulta a la vez en esta conexión
        self.sesiones: dict[str, str] = {}
        self.hl7_recibidos: list[dict] = []   # los últimos mensajes, para enseñarlos en la demo
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
        if len(self._agenda) == 8 and time.monotonic() - self._ultimo_hl7 > 3:
            self._agenda_fija = True
        return self._agenda

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

    def registrar(self, **datos):
        REGISTRO.parent.mkdir(exist_ok=True)
        with REGISTRO.open("a", encoding="utf-8") as f:
            f.write(json.dumps({"hora": datetime.now().isoformat(timespec="seconds"), **datos},
                               ensure_ascii=False) + "\n")


class Peticion(SimpleHTTPRequestHandler):
    medici: Medici = None
    url_modelo = URL
    modelo = MODELO

    def __init__(self, *a, **kw):
        super().__init__(*a, directory=str(WEB), **kw)

    def translate_path(self, path):
        ruta = path.split("?")[0]
        if ruta.startswith("/fuentes/"):
            return str(FUENTES / Path(ruta).name)
        return super().translate_path(path)

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
        nhc = self.medici.sesiones.get(partes[2]) if len(partes) > 3 else None
        if not nhc:
            self._json(404, {"error": "Sesión desconocida: vuelva a abrir al paciente."})
        return nhc

    def do_GET(self):
        partes = self.path.split("?")[0].strip("/").split("/")
        if partes[0] != "api":
            return super().do_GET()
        m = self.medici
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
            m.registrar(sesion=partes[2], paciente=nhc, accion="tarjeta",
                        alertas=[a["tipo"] for a in t["alertas"]])
            return self._json(200, t)
        self._json(404, {"error": "No existe"})

    def do_POST(self):
        partes = self.path.strip("/").split("/")
        m = self.medici
        if partes[:2] == ["api", "sesion"] and len(partes) == 2:
            nhc = str(self._cuerpo().get("nhc", "")).strip()
            filas = m.filas(nhc) if nhc else []
            if not filas:
                return self._json(404, {"error": f"No hay datos del paciente {nhc or '(vacío)'}."})
            sesion = uuid.uuid4().hex[:12]
            m.sesiones[sesion] = nhc                  # la sesión queda atada a este paciente
            m.registrar(sesion=sesion, paciente=nhc, accion="abrir")
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
            m.registrar(sesion=partes[2], paciente=nhc, accion="pregunta", pregunta=texto,
                        fuentes=r.fuentes, segundos=round(time.perf_counter() - inicio, 2))
            return self._json(200, {"respuesta": r.respuesta, "consta": r.consta,
                                    "fuentes": m.fuentes(filas, r.fuentes), "segundos": round(r.segundos, 1)})
        self._json(404, {"error": "No existe"})


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--puerto", type=int, default=8000)
    ap.add_argument("--url", default=URL, help="servidor del modelo, compatible con OpenAI")
    ap.add_argument("--modelo", default=MODELO)
    ap.add_argument("--datos", choices=["csv", "hl7"], default=os.environ.get("MEDICI_DATOS", "csv"),
                    help="csv: lee los CSV; hl7: copia vacía que se llena con mensajes HL7")
    ap.add_argument("--hl7-puerto", type=int, default=2575)
    args = ap.parse_args()

    Peticion.medici = Medici(args.datos)
    if args.datos == "hl7":
        from hl7 import escuchar
        receptor = escuchar(args.host, args.hl7_puerto, Peticion.medici.recibir_hl7)
        threading.Thread(target=receptor.serve_forever, daemon=True).start()
        print(f"Receptor HL7 (MLLP) escuchando en el puerto {args.hl7_puerto}", flush=True)
    Peticion.url_modelo, Peticion.modelo = args.url, args.modelo
    servidor = ThreadingHTTPServer((args.host, args.puerto), Peticion)
    print(f"Medici listo: http://localhost:{args.puerto}  (modelo {args.modelo} en {args.url})", flush=True)
    servidor.serve_forever()


if __name__ == "__main__":
    main()
