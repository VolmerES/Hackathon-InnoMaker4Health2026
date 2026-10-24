"""El registro de accesos: qué médico ha mirado qué paciente, cuándo y qué vio.

Cada línea es un JSON con el médico, el paciente, la sesión y la acción (abrir,
tarjeta, resumen, pregunta…). Solo se añaden líneas, y cada una lleva la huella
de la anterior: si alguien borra o cambia una línea, la cadena se rompe y
--comprobar lo dice.

    python mvp/registro.py                         # todo
    python mvp/registro.py --medico med.demo01     # qué ha consultado este médico
    python mvp/registro.py --paciente P0129798     # quién ha mirado a este paciente
    python mvp/registro.py --comprobar             # que no se ha tocado
"""

import argparse
import hashlib
import json
import sys
import threading
from datetime import datetime, timezone
from pathlib import Path

RUTA = Path(__file__).resolve().parent / "resultados" / "registro.jsonl"
INICIO = "0" * 64


def _huella(anterior: str, linea: dict) -> str:
    datos = json.dumps(linea, ensure_ascii=False, sort_keys=True, default=str)
    return hashlib.sha256((anterior + datos).encode()).hexdigest()


def leer(ruta: Path = RUTA) -> list[dict]:
    if not ruta.exists():
        return []
    with ruta.open(encoding="utf-8") as f:
        return [json.loads(l) for l in f if l.strip()]


class Registro:
    def __init__(self, ruta: Path = RUTA):
        self.ruta = ruta
        self.cerrojo = threading.Lock()
        lineas = leer(ruta)
        self.ultima = lineas[-1].get("huella", INICIO) if lineas else INICIO

    def apunta(self, **datos) -> None:
        with self.cerrojo:
            # en UTC y con la zona escrita: el registro vale igual en cualquier servidor
            linea = {"hora": datetime.now(timezone.utc).isoformat(timespec="seconds"), **datos}
            linea["huella"] = _huella(self.ultima, linea)
            self.ruta.parent.mkdir(exist_ok=True)
            with self.ruta.open("a", encoding="utf-8") as f:
                f.write(json.dumps(linea, ensure_ascii=False, default=str) + "\n")
            self.ultima = linea["huella"]


def comprobar(lineas: list[dict]) -> list[str]:
    """Las líneas cuya huella no cuadra con la anterior: borradas, cambiadas o metidas."""
    fallos, anterior = [], INICIO
    for i, l in enumerate(lineas, 1):
        if "huella" not in l:                        # líneas de antes de la cadena
            continue
        datos = {k: v for k, v in l.items() if k != "huella"}
        if _huella(anterior, datos) != l["huella"]:
            fallos.append(f"línea {i} ({l.get('hora')}, {l.get('medico')}, {l.get('accion')})")
        anterior = l["huella"]
    return fallos


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--medico")
    ap.add_argument("--paciente")
    ap.add_argument("--comprobar", action="store_true")
    ap.add_argument("--ruta", type=Path, default=RUTA)
    args = ap.parse_args()

    lineas = leer(args.ruta)
    if args.comprobar:
        fallos = comprobar(lineas)
        con = sum("huella" in l for l in lineas)
        print(f"{con} líneas encadenadas: " + ("ninguna tocada." if not fallos else f"{len(fallos)} no cuadran:"))
        for f in fallos:
            print("   " + f)
        sys.exit(1 if fallos else 0)

    for l in lineas:
        if args.medico and l.get("medico") != args.medico:
            continue
        if args.paciente and l.get("paciente") != args.paciente:
            continue
        que = {"pregunta": l.get("pregunta"), "tarjeta": "; ".join(l.get("alertas") or []),
               "resumen": f"{l.get('filas', '')} filas"}.get(l.get("accion"), l.get("detalle") or "")
        print(f"{l.get('hora')}  {l.get('medico') or '¿?':14} {l.get('paciente') or '':10} "
              f"{l.get('accion'):16} {que or ''}")


if __name__ == "__main__":
    main()
