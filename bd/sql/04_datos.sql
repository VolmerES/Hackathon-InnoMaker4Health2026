-- Generado por bd/carga/generar_datos.js · datos sintéticos de la demo
BEGIN TRANSACTION;
INSERT INTO pacientes (id_paciente, nhc, cipa, fecha_nacimiento, edad, sexo, preferencias_contacto, necesidades_accesibilidad, telefono, centro_salud, medico_ap) VALUES ('PAC-00417', 'PAC-00417', 'ALVG680204XXX', '1968-02-04', 58, 'Mujer', 'SMS', 'Horario de tarde (cuidadora a cargo)', '6XX XXX XXX', 'C.S. Ensanche de Vallecas', 'Dra. C. Herrero Nieto');
INSERT INTO derivaciones_ap (id_derivacion, id_paciente, fecha_solicitud, especialidad_destino, prioridad, motivo_derivacion, observaciones_medico, centro_origen, medico_solicitante, estado_solicitud) VALUES ('DER-2026-04871', 'PAC-00417', '2026-10-12 09:00:00', 'APARATO DIGESTIVO', 'Normal', 'Dispepsia de 5 meses que no responde a IBP (derivación DER-2026-04871)', 'MOTIVO: dispepsia. estudio

mujer 58a, dolor epigastrico de 5 meses, posprandial, al principio dia si dia no
y ahora casi diario. plenitud precoz. no disfagia no vomitos. peso estable
(68 kg). ritmo intestinal normal, no rectorragias.
AP: HTA (enalapril). no IQ previas. no AINEs de forma habitual. no fumadora.
NAMC.
ha hecho 2 tandas de omeprazol 20mg/24h 8 sem cada una, mejora algo y al dejarlo
vuelve.
EF: abd blando depresible, molestias a la palpacion en epigastrio, no masas ni
megalias, murphy -
analitica 12/09: hemograma normal Hb 13.4 / bioq con perfil hepatico normal
GPT 24 GGT 31. Ag H.pylori heces NEGATIVO 18/09. SOH negativa 18/09 (todo en HCE)
dispepsia no investigada que no responde a IBP, ruego valoracion y estudio.
gracias', 'C.S. Ensanche de Vallecas', 'Dra. C. Herrero Nieto', 'Validada');
INSERT INTO citas (id_cita, id_paciente, id_derivacion, fecha_asignacion, tipo_visita, estado_actual, prioridad, servicio, episodio, motivo_cita) VALUES ('CITA-2026-00417', 'PAC-00417', 'DER-2026-04871', '2026-10-23', 'Primera', 'Planificada', 'Normal', 'Aparato Digestivo', 'CEX-2026-118442', 'Dispepsia de 5 meses que no responde a IBP (derivación DER-2026-04871)');
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D01', 'PAC-00417', '2015-03-18', 'Informe de endoscopia', 'Aparato Digestivo · Endoscopias', 'Dr. L. Marchena Ruiz', 'HCIS', 'COLONOSCOPIA COMPLETA. Indicacion: SOH + en cribado poblacional.
Preparacion Boston 8/9. Se progresa hasta ciego (valvula ileocecal y orificio
apendicular identificados). Retirada lenta.
Hallazgos: polipo sesil de 9 mm en sigma, Paris 0-Is. Resecado con asa de
diatermia, recuperado. Resto de mucosa colonica sin lesiones. Hemorroides
internas grado I.
AP (anatomia patologica, informe 02/04/2015): adenoma tubular con displasia de
bajo grado, margenes libres.
RECOMENDACION: control endoscopico a los 5 anos (2020).', 71);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D02', 'PAC-00417', '2016-02-09', 'Nota de consulta', 'Atención Primaria', 'Dr. M. Pardo Sierra', 'HCIS', 'Acude por cifras altas de TA tomadas en farmacia. AMPA 2 semanas: media
148/92. Asintomatica. No cefalea. No disnea.
Peso 71 kg, talla 162 cm, IMC 27.
Analitica: funcion renal normal, Cr 0.8, K 4.2, col total 221, LDL 142.
ECG: ritmo sinusal, sin alteraciones.
DX: HTA esencial grado 1. Inicio enalapril 10 mg/24h. Dieta hiposodica.
Control en 2 meses.', 60);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D03', 'PAC-00417', '2017-05-21', 'Informe de alta de Urgencias', 'Urgencias Hospitalarias', 'Dra. N. Olmeda Paz', 'HCIS', 'MOTIVO: dolor lumbar izq de inicio brusco, irradiado a fosa iliaca, con
nauseas. No fiebre. No sdme miccional.
EF: puñopercusion renal izq +. Abdomen blando, no peritonismo.
Analitica: leucos 9.100, Cr 0.9, PCR 4. Sedimento: microhematuria.
ECO ABDOMINAL (21/05/2017): litiasis de 4 mm en ureter proximal izquierdo con
minima ectasia pielica. Higado, vesicula y via biliar normales. Pancreas
normal. Riñon derecho normal.
Tratamiento en urgencias: dexketoprofeno iv + hidratacion, cede el dolor.
ALTA. Dco: colico renal izquierdo no complicado. Tto: dexketoprofeno 25 mg
c/8h si dolor, abundante liquido, filtrar orina. Control por su MAP.', 94);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D04', 'PAC-00417', '2018-11-14', 'Nota de consulta', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'Lumbalgia mecanica de 10 dias, sin irradiacion, no sdme cauda. Trabaja de
auxiliar de geriatria, moviliza pacientes.
EF: contractura paravertebral lumbar, Lasegue -.
Tto: ibuprofeno 600 c/8h 5 dias + paracetamol de rescate. Calor local.
Se le explica que si lo necesita puede repetir el ibuprofeno en episodios
puntuales, no mas de 5 dias seguidos.', 55);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D05', 'PAC-00417', '2020-06-03', 'Informe de alta de Urgencias', 'Urgencias Hospitalarias', 'Dr. J. Belmonte Arce', 'HCIS', 'MOTIVO: epigastralgia de 3 dias, ardor, nauseas sin vomitos. Peor con el
estomago vacio. Tomo ibuprofeno la semana pasada por dolor de espalda.
EF: BEG, afebril, TA 139/85. Abdomen blando, molestias en epigastrio sin
defensa. Murphy -. Blumberg -.
ECG normal. Analitica: Hb 13.1, leucos 7.800, amilasa 54, bilirrubina 0.6,
GOT 21 GPT 19, Cr 0.8, PCR 3.
ECO ABDOMINAL (03/06/2020): higado de tamaño y ecoestructura normales, sin
LOEs. Vesicula alitiasica, pared fina. Via biliar no dilatada. Pancreas sin
alteraciones valorables. Bazo y riñones normales. No liquido libre.
ALTA. Dco: gastritis / dispepsia probablemente por AINE. Tto: omeprazol 20 mg
antes del desayuno 4 semanas, evitar AINEs. Si no mejora, valorar por su MAP.', 114);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D06', 'PAC-00417', '2021-01-27', 'Hoja de quirófano y alta', 'Ginecología', 'Dra. P. Aguirre Lastra', 'HCIS', 'INGRESO 26/01/2021. ALTA 28/01/2021.
Dco: utero miomatoso con menometrorragias y anemia secundaria, refractario a
tto medico.
IQ 27/01/2021: HISTERECTOMIA TOTAL LAPAROSCOPICA con doble anexectomia.
Hallazgos: utero de 11 cm con miomas intramurales. Sin complicaciones
intraoperatorias. Sangrado estimado 150 ml.
Postoperatorio: ALERGIA A METAMIZOL. En la primera dosis de metamizol iv
presenta exantema generalizado con prurito y edema palpebral, sin compromiso
respiratorio. Se suspende, se pauta dexclorfeniramina + metilprednisolona y
cede en 4 h. SE REGISTRA COMO ALERGIA EN LA HISTORIA. Analgesia posterior con
paracetamol iv.
Alta con buen estado general, tolerando dieta, herida sin signos de infeccion.
Tto al alta: paracetamol 1 g c/8h, enoxaparina 40 mg/24h 7 dias, hierro oral
3 meses. NO ADMINISTRAR METAMIZOL (Nolotil) NI DERIVADOS PIRAZOLONICOS.', 120);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D07', 'PAC-00417', '2021-03-12', 'Nota de consulta', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'Revision post IQ ginecologica. Buena evolucion, herida cerrada. Hb de control
12.4 (previa 10.1). Sigue con hierro oral.
Se actualiza hoja de alergias: METAMIZOL (exantema y edema palpebral en el
postoperatorio, enero 2021).
Se recuerda no tomar Nolotil ni compuestos con metamizol.', 42);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D08', 'PAC-00417', '2021-10-05', 'Nota de enfermería', 'Atención Primaria · Enfermería', 'E. Vilches (enfermera)', 'HCIS', 'Control de HTA. TA 132/80, FC 72. Peso 69.8 kg. Adherencia buena al enalapril.
Se revisa tecnica de AMPA. Consejo dietetico. Proxima revision en 6 meses.
Vacuna gripe administrada.', 29);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D09', 'PAC-00417', '2022-04-19', 'Analítica', 'Laboratorio · Atención Primaria', '—', 'HCIS', 'ANALITICA DE CONTROL (ayunas).
Hemograma: Hb 13.2, Hto 40%, VCM 88, leucos 6.900, plaquetas 251.000.
Bioquimica: glucosa 98, HbA1c 5.4%, Cr 0.82, FG >90, Na 140, K 4.1,
col total 214, LDL 138, HDL 56, TG 101, GOT 22, GPT 20, GGT 28, FA 61,
bilirrubina total 0.5, TSH 1.9, ferritina 48, vitamina D 18 (baja).
Sangre oculta en heces (cribado): NEGATIVA.', 62);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D10', 'PAC-00417', '2023-06-08', 'Informe de consulta', 'Oftalmología', 'Dr. S. Reina Cobo', 'HCIS', 'Revision por ojo seco y cansancio visual. AV 0.9 ambos ojos con correccion.
PIO 15/16. Polo anterior: menisco lagrimal bajo. Fondo de ojo sin
retinopatia hipertensiva. Dco: sdme de ojo seco leve. Tto: lagrimas
artificiales sin conservantes 4-6 veces/dia. Alta a su MAP.', 43);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D11', 'PAC-00417', '2024-11-14', 'Nota de consulta', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'Acude por insomnio de conciliacion y ansiedad. Cuida de su madre con demencia
en domicilio desde hace 2 anos, duerme 4-5 h. No animo bajo mantenido, no
ideacion. GAD-7: 12.
Se ofrece derivacion a psicologia (lista larga) y se pauta lorazepam 1 mg al
acostarse maximo 4 semanas. Medidas de higiene del sueño.
Refiere seguir tomando ibuprofeno 600 "cuando le duele la espalda", unos 3-4
dias al mes. Se insiste en que lo limite y en que tome paracetamol primero.', 80);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D12', 'PAC-00417', '2025-02-02', 'Informe de alta de Urgencias', 'Urgencias Hospitalarias', 'Dra. R. Quiles Amat', 'HCIS', 'MOTIVO: dolor centrotoracico opresivo de 20 min en reposo, sin irradiacion,
con sensacion de angustia. Ya cedido a su llegada.
EF: BEG, TA 144/88, FC 78, SatO2 98%. AC ritmico sin soplos. AP normal.
ECG: RS a 76 lpm, sin alteraciones de la repolarizacion. Troponina ultrasensible
seriada (0 y 3 h): normal, sin curva. Rx torax: sin hallazgos.
ALTA. Dco: dolor toracico atipico, probablemente de origen ansioso.
No cardiopatia isquemica demostrada. Se recomienda control por su MAP.
Nota: la paciente pregunta por su dolor de estomago, se le indica que lo
comente con su medica de familia.', 97);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D13', 'PAC-00417', '2026-05-18', 'Nota de consulta', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'Desde hace unas 3 semanas molestia epigastrica despues de comer, con
sensacion de llenarse enseguida. No disfagia, no vomitos, no perdida de peso.
Ritmo intestinal normal, no rectorragia. Duerme algo mejor.
EF: abdomen blando, molestia a la palpacion en epigastrio, no masas.
Dco: dispepsia funcional / posible dispepsia por AINE (sigue tomando
ibuprofeno a demanda).
Tto: omeprazol 20 mg/24h antes del desayuno 8 semanas. Suspender ibuprofeno,
paracetamol si dolor. Revision al acabar.', 72);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D14', 'PAC-00417', '2026-07-21', 'Nota de consulta', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'Revision. Con el omeprazol mejoro bastante, al dejarlo hace 2 semanas han
vuelto las molestias igual que antes. Sigue sin disfagia, sin vomitos, peso
estable (68 kg). Dice haber dejado el ibuprofeno casi del todo.
Se repite tanda de omeprazol 20 mg/24h otras 8 semanas y se solicita analitica
completa y test de H. pylori en heces. Si no mejora, derivar a Digestivo.', 63);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D15', 'PAC-00417', '2026-09-12', 'Analítica', 'Laboratorio · Atención Primaria', '—', 'HCIS', 'ANALITICA (ayunas).
Hemograma: Hb 13.4, Hto 41%, VCM 89, leucos 7.100, plaquetas 243.000.
Bioquimica: glucosa 94, HbA1c 5.5%, Cr 0.79, FG >90, Na 141, K 4.3,
proteinas totales 7.1, albumina 4.4, col total 209, LDL 131, HDL 58, TG 94,
GOT 23, GPT 24, GGT 31, FA 58, bilirrubina total 0.5, LDH 180,
amilasa 49, lipasa 32, ferritina 52, hierro 84, TSH 2.1, PCR 1.8,
Ac antitransglutaminasa IgA: negativos. IgA total normal.', 72);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D16', 'PAC-00417', '2026-09-18', 'Resultado de pruebas', 'Laboratorio · Atención Primaria', '—', 'HCIS', 'Antigeno de Helicobacter pylori en heces: NEGATIVO.
Sangre oculta en heces (inmunoquimica): NEGATIVA.
Coprocultivo: no se aislan enteropatogenos.
Parasitos en heces (3 muestras): negativo.', 24);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D17', 'PAC-00417', '2026-10-06', 'Nota de enfermería', 'Atención Primaria · Enfermería', 'E. Vilches (enfermera)', 'HCIS', 'Control. TA 136/84, FC 70. Peso 68 kg, talla 162 cm, IMC 25.9.
Refiere que las molestias de estomago siguen igual y que esta preocupada.
Comenta que su madre fallecio en julio y que ahora duerme mejor.
Toma: enalapril 10 mg/24h, omeprazol 20 mg/24h (el que le dieron en julio, lo
ha seguido tomando), paracetamol a demanda, lagrimas artificiales.
No toma lorazepam desde hace mas de un ano.', 68);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('D18', 'PAC-00417', '2026-10-12', 'Derivación a consulta externa', 'Atención Primaria', 'Dra. C. Herrero Nieto', 'HCIS', 'MOTIVO: dispepsia. estudio

mujer 58a, dolor epigastrico de 5 meses, posprandial, al principio dia si dia no
y ahora casi diario. plenitud precoz. no disfagia no vomitos. peso estable
(68 kg). ritmo intestinal normal, no rectorragias.
AP: HTA (enalapril). no IQ previas. no AINEs de forma habitual. no fumadora.
NAMC.
ha hecho 2 tandas de omeprazol 20mg/24h 8 sem cada una, mejora algo y al dejarlo
vuelve.
EF: abd blando depresible, molestias a la palpacion en epigastrio, no masas ni
megalias, murphy -
analitica 12/09: hemograma normal Hb 13.4 / bioq con perfil hepatico normal
GPT 24 GGT 31. Ag H.pylori heces NEGATIVO 18/09. SOH negativa 18/09 (todo en HCE)
dispepsia no investigada que no responde a IBP, ruego valoracion y estudio.
gracias', 123);
INSERT INTO lecturas (id_lectura, id_paciente, id_cita, fecha_lectura, modelo, version_modelo, temperatura, documentos_leidos, milisegundos, ejecutada_en) VALUES ('LEC-P1-001', 'PAC-00417', 'CITA-2026-00417', '2026-10-23 03:10:00', 'lector-extractivo', 'demo-1', 0, 18, 900, 'servidor del hospital');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H001', 'LEC-P1-001', 'PAC-00417', 'D13', 'resumen', 'Mujer de 58 años con dispepsia epigástrica posprandial desde mayo de 2026, con plenitud precoz, sin disfagia, sin vómitos y con peso estable.', 'molestia epigastrica despues de comer, con
sensacion de llenarse enseguida', 26, 100, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H002', 'LEC-P1-001', 'PAC-00417', 'D14', 'resumen', 'Dos tandas de omeprazol de 8 semanas: mejora mientras lo toma y recae al suspenderlo.', 'Con el omeprazol mejoro bastante, al dejarlo hace 2 semanas han
vuelto las molestias igual que antes', 10, 110, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H003', 'LEC-P1-001', 'PAC-00417', 'D15', 'resumen', 'Estudio de Primaria ya completo y sin hallazgos: analítica con perfil hepático, celiaquía, H. pylori y sangre oculta.', 'Ac antitransglutaminasa IgA: negativos', 363, 401, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H004', 'LEC-P1-001', 'PAC-00417', 'D14', 'resumen|alarma', 'No hay datos de alarma registrados en ninguna nota: sin disfagia, sin vómitos, sin pérdida de peso (68 kg estables desde julio) y sin anemia (Hb 13,4).', 'Sigue sin disfagia, sin vomitos, peso
estable (68 kg)', 112, 165, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H005', 'LEC-P1-001', 'PAC-00417', 'D06', 'contradiccion|alergias', 'ALERGIA A METAMIZOL documentada en 2021. La derivación de hoy dice «NAMC» (sin alergias conocidas).', 'NO ADMINISTRAR METAMIZOL (Nolotil) NI DERIVADOS PIRAZOLONICOS.', 797, 859, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H006', 'LEC-P1-001', 'PAC-00417', 'D06', 'contradiccion|cirugias', 'Sí tiene cirugía previa: histerectomía total laparoscópica con doble anexectomía en enero de 2021. La derivación dice «no IQ previas».', 'IQ 27/01/2021: HISTERECTOMIA TOTAL LAPAROSCOPICA con doble anexectomia.', 126, 197, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H007', 'LEC-P1-001', 'PAC-00417', 'D11', 'contradiccion|aine|medicacion', 'Toma ibuprofeno 600 unos 3-4 días al mes por lumbalgia, anotado en 2024 y aún en mayo de 2026. La derivación dice «no AINEs de forma habitual», y el episodio de 2020 ya se atribuyó a AINE.', 'Refiere seguir tomando ibuprofeno 600 "cuando le duele la espalda", unos 3-4
dias al mes.', 308, 397, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H008', 'LEC-P1-001', 'PAC-00417', 'D01', 'contradiccion|pendiente|endoscopia', 'Adenoma tubular de 9 mm resecado en 2015 con control endoscópico recomendado para 2020: no consta que se haya hecho. Han pasado 11 años desde la colonoscopia.', 'RECOMENDACION: control endoscopico a los 5 anos (2020).', 461, 516, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H009', 'LEC-P1-001', 'PAC-00417', 'D05', 'pruebas|eco', 'Ecografía abdominal normal, pero de junio de 2020 (hace 6 años), hecha en Urgencias por el mismo tipo de dolor.', 'ECO ABDOMINAL (03/06/2020): higado de tamaño y ecoestructura normales, sin
LOEs. Vesicula alitiasica, pared fina.', 353, 466, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H010', 'LEC-P1-001', 'PAC-00417', 'D03', 'pruebas|eco', 'Antes hubo otra ecografía en 2017 por cólico renal: litiasis de 4 mm, hígado y vía biliar normales.', 'ECO ABDOMINAL (21/05/2017): litiasis de 4 mm en ureter proximal izquierdo', 241, 314, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H011', 'LEC-P1-001', 'PAC-00417', 'D15', 'pruebas|analitica', 'Analítica del 12/09/2026 sin alteraciones: Hb 13,4, GPT 24, GGT 31, amilasa 49, lipasa 32, ferritina 52, PCR 1,8.', 'GOT 23, GPT 24, GGT 31, FA 58, bilirrubina total 0.5, LDH 180,', 234, 296, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H012', 'LEC-P1-001', 'PAC-00417', 'D16', 'pruebas|hpylori', 'H. pylori en heces NEGATIVO el 18/09/2026. Nunca ha recibido tratamiento erradicador.', 'Antigeno de Helicobacter pylori en heces: NEGATIVO.', 0, 51, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H013', 'LEC-P1-001', 'PAC-00417', 'D16', 'pruebas|soh', 'Sangre oculta en heces negativa dos veces: en el cribado de 2022 y el 18/09/2026.', 'Sangre oculta en heces (inmunoquimica): NEGATIVA.', 52, 101, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H014', 'LEC-P1-001', 'PAC-00417', 'D01', 'pruebas|endoscopia', 'Colonoscopia completa en 2015 (pólipo en sigma resecado). No consta ninguna gastroscopia en toda la historia.', 'Hallazgos: polipo sesil de 9 mm en sigma, Paris 0-Is. Resecado con asa de
diatermia, recuperado.', 187, 283, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H015', 'LEC-P1-001', 'PAC-00417', 'D15', 'pruebas|celiaquia', 'Celiaquía descartada: anticuerpos antitransglutaminasa IgA negativos con IgA total normal (12/09/2026).', 'Ac antitransglutaminasa IgA: negativos. IgA total normal.', 363, 420, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H016', 'LEC-P1-001', 'PAC-00417', 'D17', 'medicacion', 'Medicación activa en octubre de 2026: enalapril 10 mg/24h, omeprazol 20 mg/24h, paracetamol a demanda y lágrimas artificiales. Lorazepam suspendido hace más de un año.', 'Toma: enalapril 10 mg/24h, omeprazol 20 mg/24h (el que le dieron en julio, lo
ha seguido tomando), paracetamol a demanda, lagrimas artificiales.', 202, 346, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H017', 'LEC-P1-001', 'PAC-00417', 'D17', 'medicacion|ibp', 'Ojo con el omeprazol: la enfermera anota el 06/10/2026 que lo ha seguido tomando sin interrupción desde julio, así que la «falta de respuesta» se mide sobre un IBP que no se ha suspendido.', 'omeprazol 20 mg/24h (el que le dieron en julio, lo
ha seguido tomando)', 229, 299, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H018', 'LEC-P1-001', 'PAC-00417', 'D02', 'antecedentes', 'HTA esencial grado 1 desde 2016, en tratamiento con enalapril y bien controlada (TA 136/84 en octubre).', 'DX: HTA esencial grado 1. Inicio enalapril 10 mg/24h.', 261, 314, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H019', 'LEC-P1-001', 'PAC-00417', 'D07', 'antecedentes', 'Anemia ferropénica secundaria a menometrorragias, resuelta tras la histerectomía de 2021 (Hb 10,1 → 12,4).', 'Hb de control
12.4 (previa 10.1)', 64, 96, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H020', 'LEC-P1-001', 'PAC-00417', 'D12', 'antecedentes|urgencias', 'Tres pasos por Urgencias: cólico renal en 2017, epigastralgia por AINE en 2020 y dolor torácico atípico en 2025 con troponinas y ECG normales.', 'Dco: dolor toracico atipico, probablemente de origen ansioso.
No cardiopatia isquemica demostrada.', 349, 447, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H021', 'LEC-P1-001', 'PAC-00417', 'D11', 'antecedentes|salud_mental', 'Ansiedad e insomnio en 2024 mientras cuidaba a su madre con demencia (GAD-7 12), con lorazepam en pauta corta. Su madre falleció en julio de 2026 y ahora duerme mejor.', 'Cuida de su madre con demencia
en domicilio desde hace 2 anos, duerme 4-5 h.', 47, 123, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H022', 'LEC-P1-001', 'PAC-00417', 'D17', 'peso', 'Peso 68 kg y talla 162 cm (IMC 25,9) el 06/10/2026. Venía de 71 kg en 2016 y 69,8 en 2021: baja lenta de años, no pérdida reciente.', 'Peso 68 kg, talla 162 cm, IMC 25.9.', 27, 62, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H023', 'LEC-P1-001', 'PAC-00417', 'D18', 'motivo', 'Viene por una primera consulta de Digestivo pedida el 12/10/2026 por dispepsia de 5 meses que no responde a IBP.', 'dispepsia no investigada que no responde a IBP, ruego valoracion y estudio.', 663, 738, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H024', 'LEC-P1-001', 'PAC-00417', 'D01', 'falta|pendiente', 'Lo que no hay en la historia: ninguna gastroscopia, el control de colonoscopia de 2020, y una ecografía abdominal reciente (la última es de 2020).', 'control endoscopico a los 5 anos (2020)', 476, 515, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H025', 'LEC-P1-001', 'PAC-00417', 'D04', 'social', 'Trabaja como auxiliar de geriatría y moviliza pacientes: ahí está el origen de la lumbalgia por la que toma ibuprofeno.', 'Trabaja de
auxiliar de geriatria, moviliza pacientes.', 63, 116, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H026', 'LEC-P1-001', 'PAC-00417', NULL, 'familia', 'No consta ningún antecedente familiar en la historia: ni cáncer digestivo, ni enfermedad inflamatoria, ni celiaquía.', NULL, NULL, NULL, 0.95, 'N');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P1-H027', 'LEC-P1-001', 'PAC-00417', 'D18', 'habitos', 'No fumadora, según la derivación. No consta registro de consumo de alcohol en ninguna nota.', 'no fumadora.', 299, 311, 0.95, 'S');
INSERT INTO fichas_consulta (id_ficha, id_cita, id_lectura, generada_en, formato, campos_con_dato, campos_sin_dato, vista_por, vista_en) VALUES ('FICHA-P1-001', 'CITA-2026-00417', 'LEC-P1-001', '2026-10-23 03:10:02', 'medici-v1', 19, 22, NULL, NULL);
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C001', 'FICHA-P1-001', 'cabecera', NULL, 'Chip', '58 años · Mujer', NULL, NULL, 1, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C002', 'FICHA-P1-001', 'cabecera', NULL, 'Chip', 'Castellano', NULL, NULL, 2, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C003', 'FICHA-P1-001', 'cabecera', NULL, 'Chip', 'Cita de tarde: cuidadora a cargo', NULL, NULL, 3, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C004', 'FICHA-P1-001', 'cabecera', NULL, 'Chip', 'Avisos por SMS', NULL, NULL, 4, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C005', 'FICHA-P1-001', 'cabecera', NULL, 'Chip', 'Prioridad ordinaria', NULL, NULL, 5, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C006', 'FICHA-P1-001', 'antes_de_empezar', NULL, 'Alergias', 'Metamizol', 'Exantema, prurito y edema palpebral tras la primera dosis iv. La derivación de hoy dice «NAMC».', 'D06', 6, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C007', 'FICHA-P1-001', 'antes_de_empezar', NULL, 'Seguridad', 'No consta', 'Sin escalas de caídas ni de úlceras. Sin multirresistentes registrados.', NULL, 7, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C008', 'FICHA-P1-001', 'antes_de_empezar', NULL, 'Implantes', 'No consta', 'Ningún dispositivo ni prótesis en la historia.', NULL, 8, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C009', 'FICHA-P1-001', 'antes_de_empezar', NULL, 'Voluntades', 'Sin registro', 'No consta documento de voluntades anticipadas ni ONR.', NULL, 9, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C010', 'FICHA-P1-001', 'por_que_viene', 'motivo', 'Motivo de derivación', 'Dispepsia de 5 meses que no responde a IBP.', 'No consta qué espera. Enfermería anota que «está preocupada».', 'D18', 10, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C011', 'FICHA-P1-001', 'por_que_viene', 'cambios', 'Peso', '68 kg', 'estable desde julio; 71 kg en 2016', 'D17', 11, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C012', 'FICHA-P1-001', 'por_que_viene', 'cambios', 'Hemoglobina', '13,4 g/dL', 'igual que en 2022 (13,2)', 'D15', 12, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C013', 'FICHA-P1-001', 'por_que_viene', 'cambios', 'Omeprazol', 'sin suspender', 'pautado 8 semanas en julio, lo sigue tomando', 'D17', 13, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C014', 'FICHA-P1-001', 'por_que_viene', 'problemas', 'Hipertensión arterial', 'TA 136/84', NULL, 'D02', 14, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C015', 'FICHA-P1-001', 'por_que_viene', 'problemas', 'Dispepsia refractaria a IBP', 'Desde 05/2026', NULL, 'D13', 15, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C016', 'FICHA-P1-001', 'por_que_viene', 'problemas', 'Adenoma tubular de colon', '2015 · control vencido', NULL, 'D01', 16, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C017', 'FICHA-P1-001', 'por_que_viene', 'problemas', 'Anemia ferropénica', 'Resuelta en 2021', NULL, 'D07', 17, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C018', 'FICHA-P1-001', 'por_que_viene', 'medicacion', 'Enalapril', '10 mg · cada 24 h · oral', NULL, 'D17', 18, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C019', 'FICHA-P1-001', 'por_que_viene', 'medicacion', 'Omeprazol', '20 mg · cada 24 h · oral', 'Pautado 8 semanas · sigue tomándolo', 'D17', 19, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C020', 'FICHA-P1-001', 'por_que_viene', 'medicacion', 'Ibuprofeno', '600 mg', 'Sin receta · lo toma ella, 3-4 días al mes', 'D11', 20, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C021', 'FICHA-P1-001', 'por_que_viene', 'medicacion', 'Paracetamol', '1 g · a demanda', NULL, 'D17', 21, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C022', 'FICHA-P1-001', 'por_que_viene', 'medicacion', 'Adherencia', 'No consta índice. Enfermería anotó buena adherencia al enalapril en 2021.', NULL, NULL, 22, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C023', 'FICHA-P1-001', 'constantes', NULL, 'Tensión', '136/84', '06/10/2026 · enfermería', 'D17', 23, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C024', 'FICHA-P1-001', 'constantes', NULL, 'Pulso', '70 lpm', '06/10/2026 · enfermería', 'D17', 24, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C025', 'FICHA-P1-001', 'constantes', NULL, 'Saturación', 'No consta', 'sin registro', NULL, 25, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C026', 'FICHA-P1-001', 'constantes', NULL, 'Peso · IMC', '68 · 25,9', '06/10/2026 · báscula', 'D17', 26, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C027', 'FICHA-P1-001', 'constantes', NULL, 'Wearables', 'No consta', 'sin dispositivos', NULL, 27, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C028', 'FICHA-P1-001', 'vida', NULL, 'Comunicación', 'Castellano. Avisos por SMS; pide consulta de tarde.', NULL, NULL, 28, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C029', 'FICHA-P1-001', 'vida', NULL, 'Apoyo', 'Cuidó en casa a su madre con demencia; falleció en julio de 2026.', NULL, 'D17', 29, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C030', 'FICHA-P1-001', 'vida', NULL, 'Autonomía', 'No consta Barthel ni cribado cognitivo. Trabaja y acude sola.', NULL, NULL, 30, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C031', 'FICHA-P1-001', 'vida', NULL, 'Hábitos', 'No fumadora. Consumo de alcohol: no consta en 11 años.', NULL, 'D18', 31, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C032', 'FICHA-P1-001', 'vida', NULL, 'Social', 'Auxiliar de geriatría: moviliza pacientes. De ahí la lumbalgia y el ibuprofeno.', NULL, 'D04', 32, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C033', 'FICHA-P1-001', 'vida', NULL, 'Familia', 'No consta ninguna anamnesis familiar, con un adenoma personal de 2015.', NULL, NULL, 33, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C034', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Cirugías', NULL, NULL, 34, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C035', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Ginecología y obstetricia', NULL, NULL, 35, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C036', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Ingresos y urgencias', NULL, NULL, 36, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C037', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Salud mental', NULL, NULL, 37, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C038', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Analíticas 24 meses', NULL, NULL, 38, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C039', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Imagen (TC, RMN, eco)', NULL, NULL, 39, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C040', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Cribados', NULL, NULL, 40, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P1-C041', 'FICHA-P1-001', 'temas', NULL, 'Tema', 'Vacunas', NULL, NULL, 41, 'N');
INSERT INTO pacientes (id_paciente, nhc, cipa, fecha_nacimiento, edad, sexo, preferencias_contacto, necesidades_accesibilidad, telefono, centro_salud, medico_ap) VALUES ('PAC-02856', 'PAC-02856', 'ESGL540911XXX', '1954-09-11', 72, 'Hombre', 'Llamada telefónica', 'Viene con su hijo. Oye mal por el teléfono fijo', '9XX XXX XXX', 'C.S. Ángela Uriarte', 'Dr. T. Molina Esparza');
INSERT INTO derivaciones_ap (id_derivacion, id_paciente, fecha_solicitud, especialidad_destino, prioridad, motivo_derivacion, observaciones_medico, centro_origen, medico_solicitante, estado_solicitud) VALUES ('DER-2026-04902', 'PAC-02856', '2026-10-14 09:00:00', 'APARATO DIGESTIVO', 'Normal', 'Anemia ferropénica a estudio (derivación DER-2026-04902)', 'MOTIVO: anemia ferropenica a estudio

varon de 72a con anemia microcitica Hb 9.6, ferritina 8, SOH +. astenia.
no antecedentes digestivos de interes. no dolor abdominal. no rectorragia.
tto: acenocumarol, bisoprolol, omeprazol, paracetamol.
analitica 28/09 en HCE.
derivo para valoracion y completar estudio, probable origen digestivo.
rgo valoracion. gracias', 'C.S. Ángela Uriarte', 'Dr. T. Molina Esparza', 'Validada');
INSERT INTO citas (id_cita, id_paciente, id_derivacion, fecha_asignacion, tipo_visita, estado_actual, prioridad, servicio, episodio, motivo_cita) VALUES ('CITA-2026-02856', 'PAC-02856', 'DER-2026-04902', '2026-10-23', 'Primera', 'Planificada', 'Normal', 'Aparato Digestivo', 'CEX-2026-118517', 'Anemia ferropénica a estudio (derivación DER-2026-04902)');
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E01', 'PAC-02856', '2012-05-06', 'Informe de endoscopia', 'Aparato Digestivo · Endoscopias', 'Dra. I. Salcedo Vera', 'HCIS', 'GASTROSCOPIA. Indicacion: pirosis y epigastralgia de meses.
Esofago: hernia de hiato por deslizamiento de unos 3 cm. Esofagitis por reflujo
grado A de Los Angeles. Estomago: mucosa con gastritis antral eritematosa, sin
ulceras. Pilrico permeable. Duodeno hasta 2a porcion sin lesiones.
Biopsias antrales: gastritis cronica, H. pylori NEGATIVO.
Dco: ERGE con esofagitis grado A + hernia de hiato. Tto: omeprazol 20 mg/24h,
medidas antirreflujo.', 64);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E02', 'PAC-02856', '2016-09-22', 'Informe de endoscopia', 'Aparato Digestivo · Endoscopias', 'Dr. L. Marchena Ruiz', 'HCIS', 'COLONOSCOPIA COMPLETA. Indicacion: estudio de anemia leve y cambio de ritmo.
Preparacion Boston 7/9. Se alcanza ciego.
Hallazgos: diverticulosis de sigma de moderada cuantia, sin signos de
diverticulitis. Polipo de 5 mm en colon transverso, resecado con pinza.
AP (informe 06/10/2016): polipo hiperplasico, sin displasia.
Hemorroides internas grado II.
RECOMENDACION: dado el resultado, control endoscopico a los 10 anos (2026) o
antes si clinica. Cribado de sangre oculta en el intervalo.', 71);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E03', 'PAC-02856', '2016-10-14', 'Informe de radiología', 'Radiodiagnóstico', 'Dra. M. Cuesta Barrio', 'HCIS', 'TAC ABDOMINAL SIN CONTRASTE (se suspende el contraste yodado).
Nota: durante la administracion de contraste yodado el paciente presenta
urticaria generalizada y sensacion de opresion faringea. Se suspende la
inyeccion, se administra corticoide y antihistaminico iv con buena respuesta.
SE REGISTRA ALERGIA A CONTRASTES YODADOS.
Estudio sin contraste: diverticulosis sigmoidea. Quiste simple en rinon
derecho de 22 mm. Resto sin hallazgos valorables. No adenopatias.', 64);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E04', 'PAC-02856', '2018-03-03', 'Informe de consulta', 'Cardiología', 'Dr. A. Peñalver Soto', 'HCIS', 'Motivo: palpitaciones y disnea de esfuerzo. ECG y Holter: FIBRILACION
AURICULAR PERMANENTE, FC media 82 lpm. Ecocardiograma: FEVI 58%, auricula
izquierda dilatada, sin valvulopatia significativa.
CHA2DS2-VASc 3 (edad, HTA). HAS-BLED 2.
Dco: FA no valvular permanente. Se inicia ANTICOAGULACION ORAL con
acenocumarol segun pauta de hematologia + bisoprolol 2.5 mg/24h.
Se explican signos de sangrado. Control de INR en su centro de salud.', 63);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E05', 'PAC-02856', '2019-11-27', 'Informe de alta de Urgencias', 'Urgencias Hospitalarias', 'Dr. F. Izaguirre Mota', 'HCIS', 'MOTIVO: deposiciones negras de 2 dias, mareo y debilidad. Anticoagulado con
acenocumarol.
EF: palidez, TA 104/62, FC 96. Tacto rectal: melenas.
Analitica: Hb 8.9 (previa 12.8), VCM 79, plaquetas 228.000, urea 72,
Cr 1.2, INR 3.4.
Se transfunden 2 concentrados de hematies. Se revierte parcialmente la
anticoagulacion. Omeprazol iv en perfusion.
GASTROSCOPIA URGENTE (27/11/2019): ANGIODISPLASIA GASTRICA en cuerpo-antro,
con sangrado activo en babeo. Tratada con COAGULACION CON ARGON PLASMA, con
cese del sangrado. Resto: hernia de hiato conocida, sin ulceras.
ALTA (30/11/2019) con Hb 10.4. Tto: omeprazol 40 mg/24h, hierro oral,
reintroducir acenocumarol segun hematologia. Control en consulta de Digestivo.', 100);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E06', 'PAC-02856', '2020-01-16', 'Informe de consulta', 'Hematología', 'Dra. B. Sanchidrián Lopo', 'HCIS', 'Anemia ferropenica postsangrado digestivo. Hb 10.1, ferritina 9, IST 8%.
Mala tolerancia al hierro oral (estreñimiento y nauseas).
Se administran 2 dosis de HIERRO CARBOXIMALTOSA IV (16/01 y 30/01/2020).
Control a los 3 meses: Hb 13.1, ferritina 96. Alta de hematologia.
Se recomienda vigilar nuevas perdidas: el paciente esta anticoagulado y tiene
una angiodisplasia gastrica tratada.', 56);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E07', 'PAC-02856', '2021-06-09', 'Hoja de quirófano y alta', 'Traumatología', 'Dr. G. Rendueles Pina', 'HCIS', 'Dco: gonartrosis tricompartimental derecha avanzada.
IQ 08/06/2021: ARTROPLASTIA TOTAL DE RODILLA DERECHA. Anestesia intradural.
Terapia puente con heparina de bajo peso molecular por anticoagulacion previa.
Postoperatorio sin complicaciones, Hb al alta 10.9.
Alta a rehabilitacion. Tto: paracetamol, HBPM 40 mg/24h hasta reiniciar
acenocumarol, rehabilitacion 6 semanas.', 46);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E08', 'PAC-02856', '2023-04-21', 'Informe de consulta', 'Nefrología', 'Dra. C. Ibarrola Sanz', 'HCIS', 'Motivo: deterioro lento de funcion renal. Cr 1.52, FG estimado (CKD-EPI)
48 ml/min/1.73, cociente albumina/creatinina 42 mg/g. Ecografia renal: riñones
de tamaño normal, quiste simple derecho ya conocido.
Dco: ENFERMEDAD RENAL CRONICA ESTADIO 3a, probablemente vascular.
Recomendaciones: evitar AINEs, ajustar farmacos a FG, evitar contrastes
yodados (ademas tiene alergia registrada), control anual.', 52);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E09', 'PAC-02856', '2024-02-08', 'Nota de consulta', 'Atención Primaria', 'Dr. T. Molina Esparza', 'HCIS', 'Revision de cronicos. TA 138/82, FC 74. INR en rango (2.4), controles
estables.
Revisando la medicacion: el paciente sigue tomando ADIRO 100 que le pautaron
hace años "para el corazon", ademas del acenocumarol. No consta indicacion
actual para la doble terapia y tiene antecedente de sangrado digestivo.
Se le indica suspender el Adiro y se avisa de que lo comente en cardiologia.
Pendiente de confirmar que lo ha retirado.
Resto: artrosis de rodillas, usa paracetamol. Se insiste en NO tomar AINEs
por la funcion renal.', 85);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E10', 'PAC-02856', '2024-09-19', 'Informe de consulta', 'Oftalmología', 'Dra. V. Loriente Prats', 'HCIS', 'Catarata nuclear bilateral, mayor en OD. AV 0.5 OD / 0.7 OI.
Se incluye en lista de espera para facoemulsificacion de OD. Se avisa de que
esta anticoagulado: valoracion preoperatoria antes de la IQ.', 34);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E11', 'PAC-02856', '2026-03-12', 'Analítica', 'Laboratorio · Atención Primaria', '—', 'HCIS', 'ANALITICA DE CONTROL.
Hemograma: Hb 10.2, Hto 32%, VCM 78, HCM 25, leucos 6.400, plaquetas 262.000.
Bioquimica: glucosa 101, Cr 1.58, FG 46, urea 61, Na 139, K 4.6,
proteinas totales 6.9, albumina 4.0, GOT 19, GPT 17, GGT 24,
ferritina 11, hierro 42, IST 9%, transferrina 310, B12 388, folato 7.2.
INR 2.6.
Comentario del laboratorio: anemia microcitica con perfil ferropenico.', 62);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E12', 'PAC-02856', '2026-07-07', 'Nota de enfermería', 'Atención Primaria · Enfermería', 'M. Tordesillas (enfermero)', 'HCIS', 'Control de cronicos y de INR. INR 2.3. TA 134/80.
Peso 72 kg (en noviembre de 2025 pesaba 78 kg). Talla 171 cm.
El paciente dice que come menos porque "se llena enseguida" y que esta mas
cansado al andar. No refiere dolor. Deposiciones oscuras de forma intermitente
segun su hijo, el paciente no lo tiene claro.
Se le dice que lo comente con su medico en la proxima cita.', 70);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E13', 'PAC-02856', '2026-09-28', 'Analítica', 'Laboratorio · Atención Primaria', '—', 'HCIS', 'ANALITICA.
Hemograma: Hb 9.6, Hto 30%, VCM 76, leucos 6.100, plaquetas 271.000,
reticulocitos 1.1%.
Bioquimica: Cr 1.61, FG 44, ferritina 8, hierro 38, IST 7%, PCR 4.2,
albumina 3.9, calcio 9.3, LDH 198. INR 2.5.
Sangre oculta en heces (inmunoquimica): POSITIVA.
Vitamina B12 y folato normales. TSH 2.3.', 48);
INSERT INTO documentos_hce (id_documento, id_paciente, fecha_documento, tipo_documento, servicio, autor, origen, texto, palabras) VALUES ('E14', 'PAC-02856', '2026-10-14', 'Derivación a consulta externa', 'Atención Primaria', 'Dr. T. Molina Esparza', 'HCIS', 'MOTIVO: anemia ferropenica a estudio

varon de 72a con anemia microcitica Hb 9.6, ferritina 8, SOH +. astenia.
no antecedentes digestivos de interes. no dolor abdominal. no rectorragia.
tto: acenocumarol, bisoprolol, omeprazol, paracetamol.
analitica 28/09 en HCE.
derivo para valoracion y completar estudio, probable origen digestivo.
rgo valoracion. gracias', 49);
INSERT INTO lecturas (id_lectura, id_paciente, id_cita, fecha_lectura, modelo, version_modelo, temperatura, documentos_leidos, milisegundos, ejecutada_en) VALUES ('LEC-P2-001', 'PAC-02856', 'CITA-2026-02856', '2026-10-23 03:10:00', 'lector-extractivo', 'demo-1', 0, 14, 900, 'servidor del hospital');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H001', 'LEC-P2-001', 'PAC-02856', 'E13', 'resumen', 'Varón de 72 años con anemia microcítica ferropénica que empeora: Hb 10,2 en marzo y 9,6 en septiembre de 2026, con ferritina de 11 a 8.', 'Hemograma: Hb 9.6, Hto 30%, VCM 76, leucos 6.100, plaquetas 271.000,', 11, 79, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H002', 'LEC-P2-001', 'PAC-02856', 'E13', 'resumen|soh', 'Sangre oculta en heces POSITIVA el 28/09/2026: la pérdida es digestiva hasta que se demuestre lo contrario.', 'Sangre oculta en heces (inmunoquimica): POSITIVA.', 213, 262, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H003', 'LEC-P2-001', 'PAC-02856', 'E12', 'resumen|alarma|peso', 'Pérdida de 6 kg en 8 meses y sensación de plenitud precoz, anotadas por enfermería en julio de 2026. No aparecen en la derivación.', 'Peso 72 kg (en noviembre de 2025 pesaba 78 kg).', 50, 97, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H004', 'LEC-P2-001', 'PAC-02856', 'E12', 'alarma|resumen', 'El hijo refiere deposiciones oscuras intermitentes. Está escrito en la nota de enfermería y en ninguna otra parte.', 'Deposiciones oscuras de forma intermitente
segun su hijo, el paciente no lo tiene claro.', 223, 311, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H005', 'LEC-P2-001', 'PAC-02856', 'E05', 'contradiccion|antecedentes|endoscopia', 'Sí tiene antecedentes digestivos de peso: angiodisplasia gástrica con sangrado activo en 2019, tratada con argón. La derivación dice «no antecedentes digestivos de interés».', 'GASTROSCOPIA URGENTE (27/11/2019): ANGIODISPLASIA GASTRICA en cuerpo-antro,
con sangrado activo en babeo.', 345, 450, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H006', 'LEC-P2-001', 'PAC-02856', 'E01', 'contradiccion|antecedentes', 'También tiene hernia de hiato con esofagitis grado A (2012) y diverticulosis con pólipo hiperplásico resecado (2016).', 'Dco: ERGE con esofagitis grado A + hernia de hiato.', 344, 395, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H007', 'LEC-P2-001', 'PAC-02856', 'E04', 'contradiccion|medicacion|anticoagulacion', 'Está anticoagulado con acenocumarol por fibrilación auricular permanente desde 2018: hay que planificar la endoscopia y la posible terapia puente.', 'Dco: FA no valvular permanente. Se inicia ANTICOAGULACION ORAL con
acenocumarol segun pauta de hematologia + bisoprolol 2.5 mg/24h.', 236, 367, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H008', 'LEC-P2-001', 'PAC-02856', 'E09', 'contradiccion|medicacion|aine', 'Riesgo de doble antitrombótico: en 2024 seguía tomando Adiro 100 además del acenocumarol sin indicación, se le dijo que lo suspendiera y quedó «pendiente de confirmar». La derivación no lo menciona.', 'el paciente sigue tomando ADIRO 100 que le pautaron
hace años "para el corazon", ademas del acenocumarol.', 105, 210, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H009', 'LEC-P2-001', 'PAC-02856', 'E03', 'alergias', 'ALERGIA A CONTRASTES YODADOS: urticaria y opresión faríngea en el TAC de 2016. Condiciona cualquier TAC con contraste.', 'SE REGISTRA ALERGIA A CONTRASTES YODADOS.', 286, 327, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H010', 'LEC-P2-001', 'PAC-02856', 'E08', 'antecedentes|renal', 'Enfermedad renal crónica estadio 3a (FG 44): obliga a ajustar fármacos, evita AINEs y pesa en la preparación de la colonoscopia.', 'Dco: ENFERMEDAD RENAL CRONICA ESTADIO 3a, probablemente vascular.', 205, 270, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H011', 'LEC-P2-001', 'PAC-02856', 'E02', 'pendiente|falta|endoscopia', 'El control de colonoscopia recomendado para 2026 está pendiente: la última es de septiembre de 2016.', 'RECOMENDACION: dado el resultado, control endoscopico a los 10 anos (2026) o', 353, 429, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H012', 'LEC-P2-001', 'PAC-02856', 'E02', 'pruebas|endoscopia', 'Endoscopias previas: gastroscopia 2012 (hernia de hiato, esofagitis A), colonoscopia 2016 (diverticulosis y pólipo hiperplásico) y gastroscopia urgente 2019 (angiodisplasia tratada con argón).', 'Hallazgos: diverticulosis de sigma de moderada cuantia, sin signos de
diverticulitis.', 119, 204, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H013', 'LEC-P2-001', 'PAC-02856', 'E13', 'pruebas|analitica', 'Analítica del 28/09/2026: Hb 9,6, VCM 76, ferritina 8, IST 7 %, PCR 4,2, INR 2,5, B12 y folato normales.', 'Bioquimica: Cr 1.61, FG 44, ferritina 8, hierro 38, IST 7%, PCR 4.2,', 100, 168, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H014', 'LEC-P2-001', 'PAC-02856', 'E03', 'pruebas|imagen', 'TAC abdominal de 2016, hecho SIN contraste por la reacción al yodo: diverticulosis y quiste renal simple.', 'Estudio sin contraste: diverticulosis sigmoidea.', 328, 376, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H015', 'LEC-P2-001', 'PAC-02856', 'E01', 'pruebas|hpylori', 'H. pylori negativo en biopsias antrales de 2012. No hay determinación posterior.', 'Biopsias antrales: gastritis cronica, H. pylori NEGATIVO.', 286, 343, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H016', 'LEC-P2-001', 'PAC-02856', 'E14', 'medicacion', 'Tratamiento según la derivación: acenocumarol, bisoprolol, omeprazol y paracetamol.', 'tto: acenocumarol, bisoprolol, omeprazol, paracetamol.', 186, 240, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H017', 'LEC-P2-001', 'PAC-02856', 'E06', 'antecedentes|hierro', 'Ya necesitó hierro intravenoso en 2020 (carboximaltosa x2) por mala tolerancia al hierro oral, tras el sangrado de 2019.', 'Se administran 2 dosis de HIERRO CARBOXIMALTOSA IV (16/01 y 30/01/2020).', 131, 203, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H018', 'LEC-P2-001', 'PAC-02856', 'E07', 'antecedentes|cirugias', 'Cirugías previas: artroplastia total de rodilla derecha en 2021, con terapia puente de heparina por la anticoagulación.', 'IQ 08/06/2021: ARTROPLASTIA TOTAL DE RODILLA DERECHA.', 53, 106, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H019', 'LEC-P2-001', 'PAC-02856', 'E05', 'antecedentes|urgencias', 'Un ingreso por hemorragia digestiva alta en 2019: Hb 8,9, INR 3,4 y transfusión de 2 concentrados.', 'Se transfunden 2 concentrados de hematies.', 230, 272, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H020', 'LEC-P2-001', 'PAC-02856', NULL, 'falta|pendiente', 'Lo que no hay: ninguna endoscopia desde 2019, ninguna colonoscopia desde 2016, y ninguna cápsula ni enteroscopia pese al antecedente de angiodisplasia.', NULL, NULL, NULL, 0.95, 'N');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H021', 'LEC-P2-001', 'PAC-02856', 'E14', 'motivo', 'Viene por una primera consulta pedida el 14/10/2026 para estudiar una anemia ferropénica con sangre oculta positiva.', 'derivo para valoracion y completar estudio, probable origen digestivo.', 265, 335, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H022', 'LEC-P2-001', 'PAC-02856', NULL, 'habitos', 'No consta en ninguna nota registro de tabaco ni de alcohol.', NULL, NULL, NULL, 0.95, 'N');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H023', 'LEC-P2-001', 'PAC-02856', NULL, 'familia', 'No consta ningún antecedente familiar en la historia.', NULL, NULL, NULL, 0.95, 'N');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H024', 'LEC-P2-001', 'PAC-02856', 'E12', 'social', 'Acude con su hijo y oye mal por teléfono: el aviso de la prueba conviene darlo por su hijo.', 'segun su hijo, el paciente no lo tiene claro.', 266, 311, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H025', 'LEC-P2-001', 'PAC-02856', 'E12', 'peso', 'Peso 72 kg y talla 171 cm en julio de 2026, frente a 78 kg en noviembre de 2025.', 'Peso 72 kg (en noviembre de 2025 pesaba 78 kg). Talla 171 cm.', 50, 111, 0.95, 'S');
INSERT INTO hechos (id_hecho, id_lectura, id_paciente, id_documento, categoria, afirmacion, cita_literal, pos_inicio, pos_fin, confianza, consta) VALUES ('P2-H026', 'LEC-P2-001', 'PAC-02856', NULL, 'salud_mental', 'No consta ningún problema de salud mental en la historia.', NULL, NULL, NULL, 0.95, 'N');
INSERT INTO fichas_consulta (id_ficha, id_cita, id_lectura, generada_en, formato, campos_con_dato, campos_sin_dato, vista_por, vista_en) VALUES ('FICHA-P2-001', 'CITA-2026-02856', 'LEC-P2-001', '2026-10-23 03:10:02', 'medici-v1', 20, 23, NULL, NULL);
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C001', 'FICHA-P2-001', 'cabecera', NULL, 'Chip', '72 años · Hombre', NULL, NULL, 1, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C002', 'FICHA-P2-001', 'cabecera', NULL, 'Chip', 'Oye mal por el teléfono fijo', NULL, NULL, 2, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C003', 'FICHA-P2-001', 'cabecera', NULL, 'Chip', 'Viene con su hijo', NULL, NULL, 3, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C004', 'FICHA-P2-001', 'cabecera', NULL, 'Chip', 'Anticoagulado', NULL, NULL, 4, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C005', 'FICHA-P2-001', 'cabecera', NULL, 'Chip', 'Prioridad ordinaria', NULL, NULL, 5, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C006', 'FICHA-P2-001', 'antes_de_empezar', NULL, 'Alergias', 'Contrastes yodados', 'Urticaria y opresión faríngea durante la inyección en un TAC. Condiciona cualquier prueba con contraste.', 'E03', 6, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C007', 'FICHA-P2-001', 'antes_de_empezar', NULL, 'Seguridad', 'No consta', 'Sin escala de caídas con 72 años, prótesis de rodilla y anticoagulación.', NULL, 7, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C008', 'FICHA-P2-001', 'antes_de_empezar', NULL, 'Implantes', 'Prótesis de rodilla dcha.', '2021. Material y compatibilidad con RMN: no constan.', 'E07', 8, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C009', 'FICHA-P2-001', 'antes_de_empezar', NULL, 'Voluntades', 'Sin registro', 'No consta documento de voluntades anticipadas ni ONR.', NULL, 9, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C010', 'FICHA-P2-001', 'por_que_viene', 'motivo', 'Motivo de derivación', 'Anemia ferropénica a estudio, probable origen digestivo.', 'No consta qué espera. La derivación no menciona la pérdida de peso ni las heces oscuras.', 'E14', 10, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C011', 'FICHA-P2-001', 'por_que_viene', 'cambios', 'Hemoglobina', '9,6 g/dL', 'bajando desde 10,2 en marzo', 'E13', 11, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C012', 'FICHA-P2-001', 'por_que_viene', 'cambios', 'Peso', '−7,7 %', 'en 8 meses · 78 → 72 kg', 'E12', 12, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C013', 'FICHA-P2-001', 'por_que_viene', 'cambios', 'Filtrado', '44 mL/min', 'era 48 en 2023', 'E13', 13, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C014', 'FICHA-P2-001', 'por_que_viene', 'cambios', 'Sangre oculta', 'Positiva', '28/09/2026', 'E13', 14, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C015', 'FICHA-P2-001', 'por_que_viene', 'problemas', 'Fibrilación auricular permanente', 'INR 2,5', NULL, 'E04', 15, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C016', 'FICHA-P2-001', 'por_que_viene', 'problemas', 'Enfermedad renal crónica', 'G3a · FG 44', NULL, 'E08', 16, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C017', 'FICHA-P2-001', 'por_que_viene', 'problemas', 'Angiodisplasia gástrica', '2019 · argón', NULL, 'E05', 17, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C018', 'FICHA-P2-001', 'por_que_viene', 'problemas', 'Anemia ferropénica recidivante', 'Ferritina 8', NULL, 'E13', 18, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C019', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Acenocumarol', 'según pauta · hematología · INR en rango', NULL, 'E04', 19, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C020', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Bisoprolol', '2,5 mg · cada 24 h · oral', NULL, 'E04', 20, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C021', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Omeprazol', '40 mg · cada 24 h · oral', NULL, 'E05', 21, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C022', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Adiro', '100 mg', 'Pendiente de retirar desde 2024 · preguntar hoy', 'E09', 22, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C023', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Paracetamol', '1 g · a demanda · AINEs contraindicados', NULL, 'E09', 23, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C024', 'FICHA-P2-001', 'por_que_viene', 'medicacion', 'Adherencia', 'No consta índice. Controles de INR estables en 2026.', NULL, NULL, 24, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C025', 'FICHA-P2-001', 'constantes', NULL, 'Tensión', '134/80', '07/07/2026 · enfermería', 'E12', 25, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C026', 'FICHA-P2-001', 'constantes', NULL, 'Pulso', 'No consta', 'sin registro reciente', NULL, 26, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C027', 'FICHA-P2-001', 'constantes', NULL, 'Saturación', 'No consta', 'sin registro', NULL, 27, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C028', 'FICHA-P2-001', 'constantes', NULL, 'Peso · IMC', '72 · 24,6', '07/07/2026 · báscula', 'E12', 28, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C029', 'FICHA-P2-001', 'constantes', NULL, 'Wearables', 'No consta', 'la FA se vio por Holter', NULL, 29, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C030', 'FICHA-P2-001', 'vida', NULL, 'Comunicación', 'Oye mal por el teléfono fijo: dar los avisos a través del hijo.', NULL, 'E12', 30, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C031', 'FICHA-P2-001', 'vida', NULL, 'Apoyo', 'Acude con su hijo, que aporta lo de las deposiciones oscuras.', NULL, 'E12', 31, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C032', 'FICHA-P2-001', 'vida', NULL, 'Autonomía', 'No consta Barthel ni cribado cognitivo.', NULL, NULL, 32, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C033', 'FICHA-P2-001', 'vida', NULL, 'Hábitos', 'Tabaco y alcohol: no constan en 14 años de historia.', NULL, NULL, 33, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C034', 'FICHA-P2-001', 'vida', NULL, 'Social', 'No consta profesión, vivienda ni situación económica.', NULL, NULL, 34, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C035', 'FICHA-P2-001', 'vida', NULL, 'Familia', 'No consta ninguna anamnesis familiar, con anemia y sangre oculta positiva.', NULL, NULL, 35, 'S');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C036', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Endoscopias', NULL, NULL, 36, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C037', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Ingresos y urgencias', NULL, NULL, 37, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C038', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Anticoagulación', NULL, NULL, 38, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C039', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Función renal', NULL, NULL, 39, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C040', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Analíticas 24 meses', NULL, NULL, 40, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C041', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Imagen (TC, RMN, eco)', NULL, NULL, 41, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C042', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Cribados', NULL, NULL, 42, 'N');
INSERT INTO ficha_campos (id_campo, id_ficha, bloque, seccion, etiqueta, valor, detalle, id_documento, orden, destacado) VALUES ('P2-C043', 'FICHA-P2-001', 'temas', NULL, 'Tema', 'Vacunas', NULL, NULL, 43, 'N');
COMMIT;
