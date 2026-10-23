"""Perfil de los CSV: filas, tipos, nulos y valores de las columnas categóricas."""

from datos import TABLAS, conectar

MAX_CATEGORIAS = 25

con = conectar()
for t in TABLAS:
    v = t.lower()
    filas = con.execute(f"SELECT count(*) FROM {v}").fetchone()[0]
    print(f"\n### {t}: {filas:,} filas")
    for col, tipo, *_ in con.execute(f"DESCRIBE {v}").fetchall():
        n_dist, n_nulos = con.execute(
            f'SELECT count(DISTINCT "{col}"), count(*) - count("{col}") FROM {v}'
        ).fetchone()
        linea = f"- {col} ({tipo}): {n_dist:,} distintos, {n_nulos:,} nulos"
        if n_dist <= MAX_CATEGORIAS and not col.startswith("id_"):
            valores = con.execute(
                f'SELECT "{col}", count(*) n FROM {v} GROUP BY 1 ORDER BY n DESC'
            ).fetchall()
            linea += "\n    " + " · ".join(f"{a}={b:,}" for a, b in valores)
        elif tipo in ("TIMESTAMP", "DATE", "BIGINT", "DOUBLE"):
            mn, mx = con.execute(f'SELECT min("{col}"), max("{col}") FROM {v}').fetchone()
            linea += f" · rango {mn} → {mx}"
        print(linea)
