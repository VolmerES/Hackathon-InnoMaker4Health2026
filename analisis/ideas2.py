"""Ronda 2 de ideas: problemas fuera de las faltas (pruebas, derivaciones, reprogramaciones)."""

from datos import conectar

con = conectar()


def tabla(titulo, sql):
    res = con.execute(sql)
    print(f"\n## {titulo}\n" + " | ".join(d[0] for d in res.description))
    for f in res.fetchall():
        print(" | ".join(str(x) for x in f))


tabla("Pruebas según el tipo de visita que las pide", """
    SELECT a.especialidad, c.tipo_visita, count(*) pruebas
    FROM pruebas p JOIN citas c ON c.id_cita = p.id_cita_origen
    JOIN huecos_agenda h ON h.id_hueco = c.id_hueco JOIN agendas a USING (id_agenda)
    GROUP BY ALL ORDER BY ALL
""")
tabla("Pruebas más pedidas en primeras consultas, por especialidad", """
    SELECT a.especialidad, p.tipo_prueba, count(*) n
    FROM pruebas p JOIN citas c ON c.id_cita = p.id_cita_origen
    JOIN huecos_agenda h ON h.id_hueco = c.id_hueco JOIN agendas a USING (id_agenda)
    WHERE c.tipo_visita = 'P' GROUP BY 1, 2 QUALIFY row_number() OVER (PARTITION BY a.especialidad ORDER BY n DESC) <= 3
    ORDER BY a.especialidad, n DESC
""")
tabla("Primeras realizadas: ¿cuántas piden prueba?", """
    SELECT count(*) primeras, count(DISTINCT p.id_cita_origen) con_prueba
    FROM citas c LEFT JOIN pruebas p ON p.id_cita_origen = c.id_cita
    WHERE c.tipo_visita = 'P' AND c.estado_actual = 'REALIZADA'
""")
tabla("Reprogramaciones por motivo y origen", """
    SELECT motivo, origen_movimiento, count(*) n
    FROM movimientos_citas WHERE tipo_movimiento = 'REPROGRAMACION' GROUP BY ALL ORDER BY n DESC
""")
tabla("Derivaciones por motivo", """
    SELECT motivo_derivacion, estado_solicitud, count(*) n
    FROM derivaciones_ap GROUP BY ALL ORDER BY ALL
""")
tabla("Ejemplos de observaciones por motivo", """
    SELECT motivo_derivacion, observaciones_medico FROM (
        SELECT *, row_number() OVER (PARTITION BY motivo_derivacion ORDER BY id_derivacion) r
        FROM derivaciones_ap) WHERE r <= 3
""")
tabla("Incidencias por tipo de tarea", """
    SELECT tipo_tarea, incidencia_asociada, count(*) n, round(sum(duracion_minutos) / 60) horas
    FROM tareas_administrativas WHERE incidencia_asociada <> 'NINGUNA' GROUP BY ALL ORDER BY n DESC
""")
tabla("Derivaciones rechazadas y en revisión: ejemplos", """
    SELECT estado_solicitud, especialidad_destino, observaciones_medico FROM (
        SELECT *, row_number() OVER (PARTITION BY estado_solicitud ORDER BY id_derivacion) r
        FROM derivaciones_ap WHERE estado_solicitud IN ('RECHAZADA', 'EN_REVISION')) WHERE r <= 3
""")
