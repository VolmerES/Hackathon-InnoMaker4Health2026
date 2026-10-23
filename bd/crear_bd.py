#!/usr/bin/env python3
"""Crea bd/jano.db desde cero: esquema, vistas y datos de la demo.

Usa el módulo sqlite3 de Python, así que no hace falta el cliente sqlite3
instalado. Funciona igual en Ubuntu y en Windows:

    python3 bd/crear_bd.py
"""
import sqlite3
from pathlib import Path

BD = Path(__file__).resolve().parent
FICHEROS = ["01_cita.sql", "02_jano.sql", "03_vistas.sql", "04_datos.sql"]


def main():
    destino = BD / "jano.db"
    if destino.exists():
        destino.unlink()
    cx = sqlite3.connect(str(destino))
    cx.execute("PRAGMA foreign_keys = ON")
    for nombre in FICHEROS:
        cx.executescript((BD / "sql" / nombre).read_text(encoding="utf-8"))
    cx.commit()

    print("bd/jano.db creada:")
    for tabla in ("pacientes", "citas", "documentos_hce", "hechos", "ficha_campos"):
        n = cx.execute("SELECT COUNT(*) FROM %s" % tabla).fetchone()[0]
        print("  %-16s %d" % (tabla, n))
    cx.close()


if __name__ == "__main__":
    main()
