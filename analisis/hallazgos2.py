"""Segunda tanda: cuándo se envía el recordatorio y por qué fallan las ofertas de adelanto."""

from hallazgos import con, tabla

tabla("Antelación del recordatorio (días antes de la cita)", """
    SELECT date_diff('day', r.fecha_envio::DATE, h.fecha) dias_antes, count(*) n
    FROM comunicaciones r JOIN citas c USING (id_cita) JOIN huecos_agenda h ON h.id_hueco = c.id_hueco
    WHERE r.motivo = 'RECORDATORIO' GROUP BY ALL ORDER BY n DESC LIMIT 8
""")
tabla("Sin respuesta al recordatorio: tasa de falta por canal del recordatorio", """
    SELECT r.canal, r.resultado, count(*) n, round(avg(b.falta) * 100, 1) pct_falta
    FROM base b JOIN comunicaciones r USING (id_cita)
    WHERE r.motivo = 'RECORDATORIO' AND r.respuesta_paciente = 'SIN_RESPUESTA'
    GROUP BY ALL ORDER BY ALL
""")
tabla("Citas no realizadas según la respuesta al recordatorio (reparto)", """
    SELECT coalesce(r.respuesta_paciente || ' / ' || r.resultado, '(sin recordatorio)') grupo,
           sum(b.falta) faltas, round(sum(b.falta) * 100.0 / sum(sum(b.falta)) OVER (), 1) pct_de_las_faltas
    FROM base b LEFT JOIN (SELECT * FROM comunicaciones WHERE motivo = 'RECORDATORIO') r USING (id_cita)
    GROUP BY ALL ORDER BY faltas DESC
""")
tabla("Ofertas de adelanto: antelación y canal", """
    SELECT CASE WHEN d < 2 THEN 'a <2 días' WHEN d < 7 THEN 'b 2-6' ELSE 'c 7+' END antelacion,
           canal, respuesta_paciente, count(*) n
    FROM (SELECT *, date_diff('day', o.fecha_envio::DATE, h.fecha) d
          FROM comunicaciones o JOIN huecos_agenda h ON h.id_hueco = o.id_hueco_ofertado
          WHERE o.motivo = 'OFERTA_ADELANTO')
    GROUP BY ALL ORDER BY ALL
""")
tabla("Ofertas de adelanto: ¿el hueco ofertado acabó ocupado?", """
    SELECT h.estado_hueco, count(*) n
    FROM comunicaciones o JOIN huecos_agenda h ON h.id_hueco = o.id_hueco_ofertado
    WHERE o.motivo = 'OFERTA_ADELANTO' GROUP BY ALL
""")
tabla("Horas de consulta perdidas y tareas ligadas a las faltas", """
    SELECT count(*) citas_no_realizadas,
           round(count(*) * 20 / 60.0) horas_de_consulta_perdidas,
           (SELECT round(sum(duracion_minutos) / 60) FROM tareas_administrativas
            WHERE tipo_tarea = 'GESTION_NO_REALIZADA') horas_gestion_no_realizada,
           (SELECT round(sum(duracion_minutos) / 60) FROM tareas_administrativas
            WHERE tipo_tarea = 'RECONTACTO_PACIENTE') horas_recontacto
    FROM base WHERE falta = 1
""")
tabla("Rango de fechas de las citas analizadas", "SELECT min(fecha_cita), max(fecha_cita) FROM base")
