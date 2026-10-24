"""Prueba de la tarjeta «3 cosas antes de que entre».

Las alertas no las escribe la IA, así que aquí no se mide acierto de un modelo.
Se comprueban dos cosas sobre una muestra de pacientes:

1. **Que cada alerta es cierta**, contrastándola con una consulta SQL distinta
   de la que la generó, directamente sobre las tablas.
2. **Que la tarjeta sirve**: a cuántos pacientes les sale algo y qué sale.

    python mvp/prueba_alertas.py               # 2.000 pacientes al azar
    python mvp/prueba_alertas.py --n 500 --semilla 7
"""

import argparse
import re
import sys
from collections import Counter
from datetime import datetime

from alertas import MAXIMO, VENTANA, alertas
from paciente import cargar, datos_del_paciente


def comprobar(con, pid: str, todas) -> list[str]:
    """Contrasta cada alerta con SQL directo. Devuelve los fallos.

    Con la misma regla que la tarjeta: lo ya pasado, solo el último año de
    seguimiento, contado desde lo último que hay del paciente; lo pendiente,
    siempre.
    """
    fallos = []
    ahora = datetime.now()
    uno = lambda sql, *extra: con.execute(sql, [pid, *extra]).fetchone()[0]  # noqa: E731
    por_tipo = {a.tipo: a for a in todas}

    acc = uno("SELECT necesidades_accesibilidad FROM pacientes WHERE id_paciente = ?")
    if (acc != "NINGUNA") != ("accesibilidad" in por_tipo):
        fallos.append(f"accesibilidad: en la tabla {acc}")

    referencia = con.execute(
        "SELECT max(t) FROM ("
        " SELECT m.fecha_hora_movimiento AS t FROM movimientos_citas m JOIN citas c USING (id_cita) WHERE c.id_paciente = $1"
        " UNION ALL SELECT fecha_envio FROM comunicaciones WHERE id_paciente = $1"
        " UNION ALL SELECT fecha_realizacion FROM pruebas WHERE id_paciente = $1"
        " UNION ALL SELECT fecha_solicitud FROM derivaciones_ap WHERE id_paciente = $1"
        " UNION ALL SELECT CAST(h.fecha AS TIMESTAMP) FROM citas c JOIN huecos_agenda h USING (id_hueco)"
        "  WHERE c.id_paciente = $1) WHERE t <= $2", [pid, ahora]).fetchone()[0] or ahora
    desde = referencia - VENTANA

    cuentas = {
        "citas_no_realizadas": uno("SELECT count(*) FROM movimientos_citas m JOIN citas c USING (id_cita) "
                                   "WHERE c.id_paciente = ? AND m.estado_nuevo = 'NO_REALIZADA' "
                                   "AND m.tipo_movimiento NOT IN ('REPROGRAMACION', 'ANULACION') "
                                   "AND m.fecha_hora_movimiento >= ?", desde),
        "prueba_pendiente": uno("SELECT count(*) FROM pruebas WHERE id_paciente = ? AND estado_prueba = 'PROGRAMADA'"),
        "prueba_no_realizada": uno("SELECT count(*) FROM pruebas WHERE id_paciente = ? "
                                   "AND estado_prueba = 'NO_REALIZADA' AND fecha_programada >= ?", desde),
        "avisos_fallidos": uno("SELECT count(*) FROM comunicaciones WHERE id_paciente = ? "
                               "AND resultado = 'FALLIDO' AND fecha_envio >= ?", desde),
        "resultado_disponible": uno("SELECT count(*) FROM pruebas WHERE id_paciente = ? AND estado_prueba = 'REALIZADA' "
                                    "AND resultado_disponible = 1 AND fecha_realizacion >= ?", desde),
        "citas_cambiadas": uno("SELECT count(*) FROM movimientos_citas m JOIN citas c USING (id_cita) "
                               "WHERE c.id_paciente = ? AND m.tipo_movimiento IN ('REPROGRAMACION', 'ANULACION') "
                               "AND m.fecha_hora_movimiento >= ?", desde),
        "derivacion_prioritaria": uno("SELECT count(*) FROM derivaciones_ap WHERE id_paciente = ? "
                                      "AND prioridad <> 'ORDINARIA' AND estado_solicitud <> 'RECHAZADA' "
                                      "AND fecha_solicitud >= ?", desde),
    }
    for tipo, n in cuentas.items():
        a = por_tipo.get(tipo)
        if tipo == "derivacion_prioritaria":           # una alerta por derivación
            if n != sum(x.tipo == tipo for x in todas):
                fallos.append(f"{tipo}: SQL {n}, alertas {sum(x.tipo == tipo for x in todas)}")
            continue
        if (n > 0) != (a is not None):
            fallos.append(f"{tipo}: SQL {n}, alerta {'sí' if a else 'no'}")
        elif a and a.total != n:
            fallos.append(f"{tipo}: SQL {n}, alerta {a.total}")
        elif a and n > 1 and str(n) not in re.findall(r"\d+", a.texto) and tipo in (
                "citas_no_realizadas", "avisos_fallidos"):
            fallos.append(f"{tipo}: el texto no dice {n}: {a.texto}")
    return fallos


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--n", type=int, default=2000)
    ap.add_argument("--semilla", type=int, default=42)
    args = ap.parse_args()

    con = cargar()
    pacientes = [r[0] for r in con.execute(
        "SELECT id_paciente FROM pacientes ORDER BY hash(id_paciente || ?) LIMIT ?",
        [str(args.semilla), args.n]).fetchall()]

    con_fallos, en_tarjeta, cuantas = [], Counter(), Counter()
    for pid in pacientes:
        filas = datos_del_paciente(con, pid)
        todas = alertas(filas)
        validas = {f.ref for f in filas}
        fallos = comprobar(con, pid, todas)
        fallos += [f"fuente inexistente {r}" for a in todas for r in a.fuentes if r not in validas]
        if fallos:
            con_fallos.append((pid, fallos))
        cuantas[min(len(todas), MAXIMO)] += 1
        en_tarjeta.update(a.tipo for a in todas[:MAXIMO])

    n = len(pacientes)
    print(f"Pacientes: {n} (semilla {args.semilla})")
    print(f"Alertas que no cuadran con SQL directo: {len(con_fallos)} pacientes")
    for pid, fallos in con_fallos[:10]:
        print(f"   {pid}: {fallos}")
    print("Cuántas cosas salen en la tarjeta:")
    for k in range(MAXIMO + 1):
        print(f"   {k}{'+' if k == MAXIMO else ''}: {cuantas[k]:5} ({100 * cuantas[k] / n:4.1f} %)")
    print("Qué sale en la tarjeta (pacientes con esa alerta entre las 3 primeras):")
    for tipo, c in en_tarjeta.most_common():
        print(f"   {tipo:24} {c:5} ({100 * c / n:4.1f} %)")


if __name__ == "__main__":
    main()
