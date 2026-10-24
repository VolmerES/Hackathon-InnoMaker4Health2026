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
# No están en los CSV del reto: los informes clínicos de la base inventada
# (mvp/base_inventada.py). Si no hay fichero, no hay vista.
OPCIONALES = ["DOCUMENTOS_HCE"]


def conectar(extra: tuple[Path, ...] = ()) -> duckdb.DuckDBPyConnection:
    """Con `extra`, otras carpetas con los mismos CSV se suman a las de DATA."""
    con = duckdb.connect()
    for t in TABLAS:
        rutas = [(d / f"{t}.csv").as_posix() for d in (DATA, *extra)]
        con.execute(
            f"CREATE VIEW {t.lower()} AS "
            f"SELECT * FROM read_csv({rutas}, header=true, sample_size=-1, union_by_name=true)"
        )
    for t in OPCIONALES:
        rutas = [(d / f"{t}.csv").as_posix() for d in (DATA, *extra) if (d / f"{t}.csv").exists()]
        if rutas:
            con.execute(f"CREATE VIEW {t.lower()} AS SELECT * FROM read_csv({rutas}, header=true, "
                        f"sample_size=-1, union_by_name=true)")
    return con
