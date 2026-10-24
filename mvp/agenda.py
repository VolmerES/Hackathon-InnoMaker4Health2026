"""La agenda del día de la demo: 14 pacientes del reto con tarjetas variadas y,
si están cargados, los 6 crónicos generados con más historia.

La usan el servidor (para enseñarla) y el emisor HL7 (para saber qué pacientes
mandar). Con los mismos datos sale siempre la misma agenda, y también con solo
los pacientes de la agenda (la copia que se llena por HL7): cada paciente se
elige por una regla que no depende de los demás.
"""

from alertas import alertas

# Los CSV no traen nombres: son inventados para la demo.
NOMBRES = {"F": ["María Alonso", "Carmen Ruiz", "Lucía Sanz", "Ana Gil", "Pilar Mora",
                 "Rosa Navarro", "Elena Prieto", "Teresa Vidal", "Isabel Romero", "Marta Delgado",
                 "Dolores Castro", "Julia Herrero"],
           "M": ["José Pardo", "Antonio Vega", "Manuel Ortiz", "Javier Lozano", "Luis Cano",
                 "Pedro Ibáñez", "Ramón Soler", "Andrés Molina", "Francisco Rubio", "Miguel Santos",
                 "Alberto Gallego", "Rafael Méndez"]}
# Los de la base inventada (mvp/base_inventada.py), con el nombre de sus informes
NOMBRES_FIJOS = {"P0000417": "María Alonso Vega", "P0002856": "Ramón Esteve Gálvez"}
POCOS = 10            # con tan pocos pacientes (la base inventada), entran todos
# cuántos del reto con 3, 2, 1 y 0 cosas en la tarjeta (María cuenta en las de 3)
CUPOS = {3: 5, 2: 4, 1: 3, 0: 2}
CRONICOS = 6          # los generados con más citas
CADA = 3              # un crónico cada tantos huecos


def elegir_agenda(con, filas_de) -> list[dict]:
    """La primera es «María», la del guion: mujer, con una necesidad de
    accesibilidad, una prueba pendiente y una derivación de Primaria.
    `filas_de(nhc)` devuelve las filas del paciente (paciente.datos_del_paciente).
    """
    todos = [r[0] for r in con.execute(
        "SELECT DISTINCT id_paciente FROM citas WHERE id_paciente NOT LIKE 'G%' ORDER BY 1").fetchall()]
    if len(todos) <= POCOS:
        # una base pequeña, de historias largas: todos por orden de NHC, sin cupos ni tope de filas
        orden = [(None, nhc, filas_de(nhc)[0].campos) for nhc in todos]
        return _con_nombre(con, orden)
    maria = [r[0] for r in con.execute(
        "SELECT p.id_paciente FROM pacientes p WHERE p.sexo = 'F' AND p.id_paciente NOT LIKE 'G%' "
        "AND p.necesidades_accesibilidad <> 'NINGUNA' "
        "AND EXISTS (SELECT 1 FROM pruebas r WHERE r.id_paciente = p.id_paciente AND r.estado_prueba = 'PROGRAMADA') "
        "AND EXISTS (SELECT 1 FROM derivaciones_ap d WHERE d.id_paciente = p.id_paciente) "
        "ORDER BY hash(p.id_paciente || 'maria') LIMIT 200").fetchall()]
    candidatos = [r[0] for r in con.execute(
        "SELECT DISTINCT id_paciente FROM citas WHERE id_paciente NOT LIKE 'G%' "
        "ORDER BY hash(id_paciente || 'agenda') LIMIT 600").fetchall()]

    faltan = dict(CUPOS)
    elegidos = []
    for nhc in maria:
        filas = filas_de(nhc)
        if len(alertas(filas)) >= 3 and len(filas) <= 20:
            faltan[3] -= 1
            elegidos.append((9, nhc, filas[0].campos))      # 9: siempre la primera
            break
    for nhc in candidatos:
        if nhc in {e[1] for e in elegidos}:
            continue
        filas = filas_de(nhc)
        n = min(len(alertas(filas)), 3)
        if faltan.get(n) and len(filas) <= 25:
            faltan[n] -= 1
            elegidos.append((n, nhc, filas[0].campos))
        if not any(faltan.values()):
            break

    # alternando: una con muchas cosas, otra con pocas…
    elegidos.sort(key=lambda e: (-e[0], e[1]))
    mitad = (len(elegidos) + 1) // 2
    orden = [e for par in zip(elegidos[:mitad], elegidos[mitad:][::-1]) for e in par]
    orden += elegidos[len(orden):mitad]

    # los crónicos de años: los que más citas tienen
    cronicos = [(None, r[0], filas_de(r[0])[0].campos) for r in con.execute(
        "SELECT id_paciente FROM citas WHERE id_paciente LIKE 'G%' GROUP BY 1 "
        "ORDER BY count(*) DESC, 1 LIMIT ?", [CRONICOS]).fetchall()]
    for i, c in enumerate(cronicos):
        orden.insert(min(CADA * (i + 1) - 1, len(orden)), c)

    return _con_nombre(con, orden)


def _con_nombre(con, orden) -> list[dict]:
    """La agenda: cada paciente con su hora (cada 20 minutos desde las 9) y un nombre."""
    usados = {"F": 0, "M": 0}
    agenda = []
    for i, (_, nhc, ficha) in enumerate(orden):
        sexo = con.execute("SELECT sexo FROM pacientes WHERE id_paciente = ?", [nhc]).fetchone()[0]
        lista = NOMBRES.get(sexo, NOMBRES["F"])
        nombre = NOMBRES_FIJOS.get(nhc) or lista[usados.get(sexo, 0) % len(lista)]
        usados[sexo] = usados.get(sexo, 0) + 1
        agenda.append({"nhc": nhc, "hora": f"{9 + (i * 20) // 60:02d}:{(i * 20) % 60:02d}",
                       "nombre": nombre, "edad": ficha["edad"], "sexo": sexo,
                       "cronico": nhc.startswith("G")})
    return agenda
