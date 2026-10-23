"""Cifras para decidir entre las ideas candidatas (docs/CONTEXTO.md)."""

from datos import conectar

con = conectar()


def tabla(titulo: str, sql: str) -> None:
    res = con.execute(sql)
    cols = [d[0] for d in res.description]
    filas = res.fetchall()
    print(f"\n## {titulo}")
    print(" | ".join(cols))
    for f in filas:
        print(" | ".join(f"{x:,.1f}" if isinstance(x, float) else str(x) for x in f))


# --- Tareas administrativas: dónde se va el tiempo ---------------------------
tabla("Minutos de tareas administrativas por tipo", """
    SELECT tipo_tarea, perfil_profesional, count(*) n,
           round(sum(duracion_minutos)) minutos,
           round(avg(duracion_minutos), 1) media,
           round(avg(automatizable) * 100) pct_automatizable
    FROM tareas_administrativas GROUP BY ALL ORDER BY minutos DESC
""")
tabla("Total y automatizable (horas)", """
    SELECT round(sum(duracion_minutos) / 60) horas_totales,
           round(sum(duracion_minutos) FILTER (automatizable = 1) / 60) horas_automatizables,
           min(fecha_hora_inicio), max(fecha_hora_inicio)
    FROM tareas_administrativas
""")

# --- Citas no realizadas -----------------------------------------------------
tabla("Estado final de las citas (sin urgentes)", """
    SELECT tipo_visita, estado_actual, count(*) n
    FROM citas WHERE estado_actual <> 'URGENTE' GROUP BY ALL ORDER BY ALL
""")

# Base: citas programadas que llegaron a su día (realizada o no realizada)
con.execute("""
    CREATE TEMP TABLE base AS
    SELECT c.*, p.edad, p.sexo, p.preferencias_contacto, p.necesidades_accesibilidad,
           h.fecha AS fecha_cita, a.especialidad,
           (c.estado_actual = 'NO_REALIZADA')::INT AS falta
    FROM citas c
    JOIN pacientes p USING (id_paciente)
    JOIN huecos_agenda h USING (id_hueco)
    JOIN agendas a USING (id_agenda)
    WHERE c.estado_actual IN ('REALIZADA', 'NO_REALIZADA')
""")
tabla("Tasa de no realizadas por tipo de visita", """
    SELECT tipo_visita, count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM base GROUP BY ALL ORDER BY ALL
""")
tabla("Tasa de no realizadas por respuesta al recordatorio", """
    SELECT coalesce(r.respuesta_paciente, '(sin recordatorio)') respuesta,
           coalesce(r.resultado, '-') resultado, count(*) n,
           round(avg(b.falta) * 100, 1) pct_falta
    FROM base b
    LEFT JOIN (SELECT * FROM comunicaciones WHERE motivo = 'RECORDATORIO') r USING (id_cita)
    GROUP BY ALL ORDER BY n DESC
""")
tabla("¿El recordatorio fue por el canal preferido?", """
    SELECT (r.canal = b.preferencias_contacto) canal_preferido, count(*) n,
           round(avg(b.falta) * 100, 1) pct_falta
    FROM base b JOIN comunicaciones r USING (id_cita)
    WHERE r.motivo = 'RECORDATORIO' GROUP BY ALL
""")
tabla("Tasa de no realizadas por accesibilidad", """
    SELECT necesidades_accesibilidad, count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM base GROUP BY ALL ORDER BY pct_falta DESC
""")
tabla("Tasa de no realizadas por canal preferido", """
    SELECT preferencias_contacto, count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM base GROUP BY ALL ORDER BY pct_falta DESC
""")
tabla("Tasa de no realizadas por edad", """
    SELECT CASE WHEN edad < 40 THEN '18-39' WHEN edad < 65 THEN '40-64'
                WHEN edad < 80 THEN '65-79' ELSE '80+' END tramo,
           count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM base GROUP BY ALL ORDER BY ALL
""")
tabla("Tasa de no realizadas por demora (días desde la asignación)", """
    SELECT CASE WHEN d < 15 THEN 'a <15' WHEN d < 30 THEN 'b 15-29' WHEN d < 60 THEN 'c 30-59'
                WHEN d < 120 THEN 'd 60-119' ELSE 'e 120+' END demora,
           count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM (SELECT *, date_diff('day', fecha_asignacion::DATE, fecha_cita) d FROM base)
    GROUP BY ALL ORDER BY ALL
""")
tabla("Tasa de no realizadas por reprogramada", """
    SELECT reprogramada_ficticia, count(*) n, round(avg(falta) * 100, 1) pct_falta
    FROM base GROUP BY ALL ORDER BY ALL
""")

# --- Huecos liberados: ¿se vuelven a usar? -----------------------------------
tabla("Huecos con más de una ocupación en su historia", """
    SELECT count(*) huecos FROM (
        SELECT id_hueco FROM ocupaciones_hueco GROUP BY 1 HAVING count(*) > 1)
""")
tabla("Huecos liberados: estado actual y antelación con la que se liberaron", """
    SELECT o.motivo_fin, h.estado_hueco,
           CASE WHEN d < 0 THEN 'a ya pasado' WHEN d < 2 THEN 'b <2 días'
                WHEN d < 7 THEN 'c 2-6 días' WHEN d < 30 THEN 'd 7-29 días'
                ELSE 'e 30+ días' END antelacion,
           count(*) n
    FROM (SELECT *, date_diff('day', fin_ocupacion::DATE, h2.fecha) d
          FROM ocupaciones_hueco JOIN huecos_agenda h2 USING (id_hueco)
          WHERE ocupacion_activa = 0) o
    JOIN huecos_agenda h USING (id_hueco)
    GROUP BY ALL ORDER BY ALL
""")
tabla("Ofertas de adelanto: respuesta", """
    SELECT motivo, respuesta_paciente, count(*) n
    FROM comunicaciones WHERE id_hueco_ofertado IS NOT NULL GROUP BY ALL ORDER BY ALL
""")

# --- Oferta y demanda por especialidad ---------------------------------------
tabla("Huecos por estado y especialidad", """
    SELECT a.especialidad, h.tipo_hueco,
           count(*) FILTER (h.estado_hueco = 'LIBRE') libres,
           count(*) FILTER (h.estado_hueco = 'OCUPADO') ocupados,
           count(*) FILTER (h.estado_hueco = 'BLOQUEADO') bloqueados
    FROM huecos_agenda h JOIN agendas a USING (id_agenda)
    GROUP BY ALL ORDER BY ALL
""")
tabla("Demora de primeras consultas por especialidad y prioridad (días)", """
    SELECT d.especialidad_destino, d.prioridad, count(*) n,
           round(median(date_diff('day', d.fecha_solicitud::DATE, h.fecha))) mediana,
           round(quantile_cont(date_diff('day', d.fecha_solicitud::DATE, h.fecha), 0.9)) p90
    FROM derivaciones_ap d JOIN citas c USING (id_derivacion) JOIN huecos_agenda h USING (id_hueco)
    GROUP BY ALL ORDER BY ALL
""")
tabla("Derivaciones sin cita", """
    SELECT especialidad_destino, estado_solicitud, count(*) n
    FROM derivaciones_ap WHERE estado_solicitud <> 'CITADA' GROUP BY ALL ORDER BY ALL
""")

# --- Pruebas y revisiones ----------------------------------------------------
tabla("Pruebas con revisión enlazada", """
    SELECT (id_cita_revision IS NOT NULL) con_revision, estado_prueba,
           resultado_disponible, count(*) n
    FROM pruebas GROUP BY ALL ORDER BY ALL
""")
tabla("Revisiones enlazadas: ¿había resultado el día de la revisión?", """
    SELECT c.estado_actual,
           CASE WHEN p.fecha_realizacion IS NULL THEN 'prueba sin hacer'
                WHEN p.fecha_realizacion::DATE > h.fecha THEN 'prueba después de la revisión'
                ELSE 'prueba antes' END situacion,
           count(*) n
    FROM pruebas p JOIN citas c ON c.id_cita = p.id_cita_revision
    JOIN huecos_agenda h ON h.id_hueco = c.id_hueco
    GROUP BY ALL ORDER BY ALL
""")

# --- Circuito el día de la consulta ------------------------------------------
tabla("Circuito de consulta", """
    SELECT round(avg(tiempo_espera), 1) espera_media, round(avg(tiempo_atencion), 1) atencion_media,
           round(avg(date_diff('minute', hora_llegada, hora_admision)), 1) llegada_a_admision
    FROM circuito_consulta
""")
