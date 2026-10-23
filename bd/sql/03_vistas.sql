-- =====================================================================
-- Jano / Medici · 03 · Vistas
-- Lo que consulta la pantalla y lo que mide el piloto.
-- =====================================================================

-- Toda la cita de hoy en una fila: paciente, derivación, hueco y ficha.
CREATE VIEW v_cita_completa AS
SELECT
  c.id_cita,
  c.fecha_asignacion,
  c.tipo_visita,
  c.estado_actual,
  c.prioridad,
  c.servicio,
  c.episodio,
  p.id_paciente,
  p.nhc,
  p.edad,
  p.sexo,
  p.preferencias_contacto,
  p.necesidades_accesibilidad,
  d.id_derivacion,
  d.fecha_solicitud,
  d.motivo_derivacion,
  d.prioridad          AS prioridad_ap,
  d.centro_origen,
  d.medico_solicitante,
  a.profesional_ficticio,
  h.fecha              AS fecha_hueco,
  h.hora_inicio,
  f.id_ficha,
  f.generada_en        AS ficha_generada_en,
  f.campos_con_dato,
  f.campos_sin_dato
FROM citas c
JOIN pacientes p          ON p.id_paciente = c.id_paciente
LEFT JOIN derivaciones_ap d ON d.id_derivacion = c.id_derivacion
LEFT JOIN huecos_agenda h ON h.id_hueco = c.id_hueco
LEFT JOIN agendas a       ON a.id_agenda = h.id_agenda
LEFT JOIN fichas_consulta f ON f.id_cita = c.id_cita;

-- La ficha de una cita, campo a campo, con la frase que la sostiene.
CREATE VIEW v_ficha_con_fuente AS
SELECT
  fc.id_ficha,
  f.id_cita,
  fc.bloque,
  fc.seccion,
  fc.etiqueta,
  fc.valor,
  fc.detalle,
  fc.destacado,
  h.cita_literal,
  h.confianza,
  h.consta,
  doc.id_documento,
  doc.fecha_documento,
  doc.tipo_documento,
  doc.servicio AS servicio_documento
FROM ficha_campos fc
JOIN fichas_consulta f    ON f.id_ficha = fc.id_ficha
LEFT JOIN hechos h        ON h.id_hecho = fc.id_hecho
LEFT JOIN documentos_hce doc ON doc.id_documento = h.id_documento;

-- Lo que la historia no tiene: los huecos de cada paciente.
CREATE VIEW v_no_consta AS
SELECT
  h.id_paciente,
  h.categoria,
  h.afirmacion,
  l.fecha_lectura
FROM hechos h
JOIN lecturas l ON l.id_lectura = h.id_lectura
WHERE h.consta = 'N';

-- Contradicciones entre la derivación y la historia: lo que más pesa en la demo.
CREATE VIEW v_contradicciones AS
SELECT
  h.id_paciente,
  d.id_derivacion,
  h.afirmacion,
  h.cita_literal,
  doc.id_documento,
  doc.fecha_documento,
  doc.servicio
FROM hechos h
JOIN documentos_hce doc ON doc.id_documento = h.id_documento
JOIN derivaciones_ap d  ON d.id_paciente = h.id_paciente
WHERE h.categoria LIKE '%contradiccion%';

-- Indicadores del piloto en Digestivo.
CREATE VIEW v_indicadores_piloto AS
SELECT
  c.servicio,
  COUNT(*)                                                           AS citas,
  SUM(CASE WHEN c.tipo_visita = 'Primera'  THEN 1 ELSE 0 END)        AS primeras,
  SUM(CASE WHEN c.tipo_visita = 'Sucesiva' THEN 1 ELSE 0 END)        AS sucesivas,
  SUM(CASE WHEN cc.resuelta_en_una_visita = 'S' THEN 1 ELSE 0 END)   AS resueltas_una_visita,
  SUM(CASE WHEN pr.adelantada_por_jano    = 'S' THEN 1 ELSE 0 END)   AS pruebas_adelantadas,
  SUM(CASE WHEN pr.evitada_por_duplicidad = 'S' THEN 1 ELSE 0 END)   AS duplicadas_evitadas
FROM citas c
LEFT JOIN circuito_consulta cc ON cc.id_cita = c.id_cita
LEFT JOIN pruebas pr           ON pr.id_cita_origen = c.id_cita
GROUP BY c.servicio;

-- Acierto del lector: cuántas veces el facultativo corrige la ruta propuesta.
CREATE VIEW v_acierto_lector AS
SELECT
  v.ruta_propuesta,
  COUNT(*)                                                AS validadas,
  SUM(CASE WHEN v.corregida = 'S' THEN 1 ELSE 0 END)      AS corregidas,
  AVG(v.segundos_validacion)                              AS segundos_medios
FROM validaciones_derivacion v
GROUP BY v.ruta_propuesta;
