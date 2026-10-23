"""La agenda del día de la demo: 8 pacientes con tarjetas variadas.

La usan el servidor (para enseñarla) y el emisor HL7 (para saber qué pacientes
mandar). Con los mismos datos sale siempre la misma agenda.
"""

from alertas import alertas

# Los CSV no traen nombres: son inventados para la demo.
NOMBRES = {"F": ["María Alonso", "Carmen Ruiz", "Lucía Sanz", "Ana Gil", "Pilar Mora",
                 "Rosa Navarro", "Elena Prieto", "Teresa Vidal"],
           "M": ["José Pardo", "Antonio Vega", "Manuel Ortiz", "Javier Lozano", "Luis Cano",
                 "Pedro Ibáñez", "Ramón Soler", "Andrés Molina"]}


def elegir_agenda(con, filas_de) -> list[dict]:
    """Tres pacientes con 3 cosas, dos con 2, dos con 1 y uno sin nada.

    La primera es «María», la del guion: mujer, con una necesidad de
    accesibilidad, una prueba pendiente y una derivación de Primaria.
    `filas_de(nhc)` devuelve las filas del paciente (paciente.datos_del_paciente).
    """
    maria = [r[0] for r in con.execute(
        "SELECT p.id_paciente FROM pacientes p WHERE p.sexo = 'F' "
        "AND p.necesidades_accesibilidad <> 'NINGUNA' "
        "AND EXISTS (SELECT 1 FROM pruebas r WHERE r.id_paciente = p.id_paciente AND r.estado_prueba = 'PROGRAMADA') "
        "AND EXISTS (SELECT 1 FROM derivaciones_ap d WHERE d.id_paciente = p.id_paciente) "
        "ORDER BY hash(p.id_paciente || 'maria') LIMIT 200").fetchall()]
    candidatos = [r[0] for r in con.execute(
        "SELECT DISTINCT id_paciente FROM citas ORDER BY hash(id_paciente || 'agenda') LIMIT 400").fetchall()]

    faltan = {3: 3, 2: 2, 1: 2, 0: 1}
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

    elegidos.sort(key=lambda e: (-e[0], e[1]))
    orden = [elegidos[i] for i in (0, 3, 5, 1, 7, 4, 2, 6) if i < len(elegidos)]
    usados = {"F": 0, "M": 0}
    agenda = []
    for i, (_, nhc, ficha) in enumerate(orden):
        sexo = con.execute("SELECT sexo FROM pacientes WHERE id_paciente = ?", [nhc]).fetchone()[0]
        lista = NOMBRES.get(sexo, NOMBRES["F"])
        nombre = lista[usados.get(sexo, 0) % len(lista)]
        usados[sexo] = usados.get(sexo, 0) + 1
        agenda.append({"nhc": nhc, "hora": f"{9 + (i * 20) // 60:02d}:{(i * 20) % 60:02d}",
                       "nombre": nombre, "edad": ficha["edad"], "sexo": sexo})
    return agenda
