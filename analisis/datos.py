"""Conexión DuckDB con una vista por cada CSV de data/ (u otra carpeta con MEDICCI_CSV)."""

import os
from pathlib import Path

import duckdb

DATA = Path(os.environ.get("MEDICCI_CSV") or Path(__file__).resolve().parent.parent / "data")

TABLAS = [
    "PACIENTES", "DERIVACIONES_AP", "AGENDAS", "HUECOS_AGENDA", "CITAS",
    "MOVIMIENTOS_CITAS", "OCUPACIONES_HUECO", "COMUNICACIONES",
    "TAREAS_ADMINISTRATIVAS", "CIRCUITO_CONSULTA", "PRUEBAS",
]


def conectar(extra: tuple[Path, ...] = ()) -> duckdb.DuckDBPyConnection:
    """Con `extra`, otras carpetas con los mismos CSV se suman a las de DATA."""
    con = duckdb.connect()
    for t in TABLAS:
        rutas = [(d / f"{t}.csv").as_posix() for d in (DATA, *extra)]
        con.execute(
            f"CREATE VIEW {t.lower()} AS "
            f"SELECT * FROM read_csv({rutas}, header=true, sample_size=-1, union_by_name=true)"
        )
    return con
