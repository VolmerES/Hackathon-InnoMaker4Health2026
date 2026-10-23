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
"""

import argparse
import json
import sys
import threading
import time
import urllib.request
import uuid
from datetime import datetime
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from alertas import alertas, tarjeta
from paciente import cargar, datos_del_paciente
from responder import MODELO, URL, responder

AQUI = Path(__file__).resolve().parent
WEB = AQUI / "web"
REGISTRO = AQUI / "resultados" / "registro.jsonl"

# La agenda necesita nombres y los CSV no los traen: son inventados para la demo.
NOMBRES = {"F": ["María Alonso", "Carmen Ruiz", "Lucía Sanz", "Ana Gil", "Pilar Mora",
                 "Rosa Navarro", "Elena Prieto", "Teresa Vidal"],
           "M": ["José Pardo", "Antonio Vega", "Manuel Ortiz", "Javier Lozano", "Luis Cano",
                 "Pedro Ibáñez", "Ramón Soler", "Andrés Molina"]}


class Medici:
    def __init__(self):
        print("Cargando los datos…", flush=True)
        self.con = cargar()
        self.cerrojo = threading.Lock()       # DuckDB: una consulta a la vez en esta conexión
        self.sesiones: dict[str, str] = {}
        self.agenda = self._agenda()

    def filas(self, nhc: str):
        with self.cerrojo:
            return datos_del_paciente(self.con, nhc)

    def _agenda(self) -> list[dict]:
        """8 pacientes con tarjetas variadas: tres con 3 cosas, dos con 2, dos con 1 y uno sin nada.

        La primera es «María», la del guion: mujer, con una necesidad de
        accesibilidad, una prueba pendiente y una derivación de Primaria.
        """
        with self.cerrojo:
            maria = [r[0] for r in self.con.execute(
                "SELECT p.id_paciente FROM pacientes p WHERE p.sexo = 'F' "
                "AND p.necesidades_accesibilidad <> 'NINGUNA' "
                "AND EXISTS (SELECT 1 FROM pruebas r WHERE r.id_paciente = p.id_paciente AND r.estado_prueba = 'PROGRAMADA') "
                "AND EXISTS (SELECT 1 FROM derivaciones_ap d WHERE d.id_paciente = p.id_paciente) "
                "ORDER BY hash(p.id_paciente || 'maria') LIMIT 200").fetchall()]
            candidatos = [r[0] for r in self.con.execute(
                "SELECT DISTINCT id_paciente FROM citas ORDER BY hash(id_paciente || 'agenda') LIMIT 400"
            ).fetchall()]
        faltan = {3: 3, 2: 2, 1: 2, 0: 1}
        elegidos = []
        for nhc in maria:
            filas = self.filas(nhc)
            if len(alertas(filas)) >= 3 and len(filas) <= 20:
                faltan[3] -= 1
                elegidos.append((9, nhc, filas[0].campos))      # 9: siempre la primera
                break
        for nhc in candidatos:
            filas = self.filas(nhc)
            n = min(len(alertas(filas)), 3)
            if faltan.get(n) and len(filas) <= 25 and nhc not in {e[1] for e in elegidos}:
                faltan[n] -= 1
                elegidos.append((n, nhc, filas[0].campos))
            if not any(faltan.values()):
                break
        elegidos.sort(key=lambda e: (-e[0], e[1]))
        orden = [elegidos[i] for i in (0, 3, 5, 1, 7, 4, 2, 6) if i < len(elegidos)]
        usados = {"F": 0, "M": 0}
        agenda = []
        for i, (_, nhc, ficha) in enumerate(orden):
            sexo = self.con.execute("SELECT sexo FROM pacientes WHERE id_paciente = ?", [nhc]).fetchone()[0]
            lista = NOMBRES.get(sexo, NOMBRES["F"])
            nombre = lista[usados.get(sexo, 0) % len(lista)]
            usados[sexo] = usados.get(sexo, 0) + 1
            agenda.append({"nhc": nhc, "hora": f"{9 + (i * 20) // 60:02d}:{(i * 20) % 60:02d}",
                           "nombre": nombre, "edad": ficha["edad"], "sexo": sexo})
        return agenda

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

    def log_message(self, formato, *args):
        if "/api/" in self.path:
            sys.stderr.write(f"{self.command} {self.path}\n")

    def _json(self, codigo: int, cuerpo) -> None:
        datos = json.dumps(cuerpo, ensure_ascii=False).encode()
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
        if partes[1:3] == ["hcis", "paciente"] and len(partes) == 4:
            filas = m.filas(partes[3])
            if not filas:
                return self._json(404, {"error": "Paciente desconocido"})
            return self._json(200, {"filas": [{"ref": f.ref, "tabla": f.tabla, "texto": f.texto}
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
    args = ap.parse_args()

    Peticion.medici = Medici()
    Peticion.url_modelo, Peticion.modelo = args.url, args.modelo
    servidor = ThreadingHTTPServer((args.host, args.puerto), Peticion)
    print(f"Medici listo: http://localhost:{args.puerto}  (modelo {args.modelo} en {args.url})", flush=True)
    servidor.serve_forever()


if __name__ == "__main__":
    main()
