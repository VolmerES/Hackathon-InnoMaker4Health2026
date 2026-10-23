"""Conexión DuckDB con una vista por cada CSV de data/."""

from pathlib import Path

import duckdb

DATA = Path(__file__).resolve().parent.parent / "data"

TABLAS = [
    "PACIENTES", "DERIVACIONES_AP", "AGENDAS", "HUECOS_AGENDA", "CITAS",
    "MOVIMIENTOS_CITAS", "OCUPACIONES_HUECO", "COMUNICACIONES",
    "TAREAS_ADMINISTRATIVAS", "CIRCUITO_CONSULTA", "PRUEBAS",
]


def conectar() -> duckdb.DuckDBPyConnection:
    con = duckdb.connect()
    for t in TABLAS:
        ruta = (DATA / f"{t}.csv").as_posix()
        con.execute(
            f"CREATE VIEW {t.lower()} AS "
            f"SELECT * FROM read_csv('{ruta}', header=true, sample_size=-1)"
        )
    return con
