// Historia clínica sintética completa de un paciente de la demo de Medicci.
// NADA de esto es de una persona real: está escrito para el hackathon imitando
// el registro de las notas clínicas españolas (abreviaturas, prisa, copia-pega).
// Cada "hecho" del agente lleva la cita literal del documento del que sale.

window.PACIENTE = {
  nombre: "María Alonso Vega",
  nhc: "PAC-00417",
  edad: 58,
  sexo: "Mujer",
  nacimiento: "04/02/1968",
  cipa: "ALVG680204XXX",
  centro: "C.S. Ensanche de Vallecas",
  medico_ap: "Dra. C. Herrero Nieto",
  telefono: "6XX XXX XXX",
  contacto: "SMS",
  acc: "Horario de tarde (cuidadora a cargo)",
  cita: {
    fecha: "Hoy, 23/10/2026 · 16:40",
    tipo: "Primera consulta",
    servicio: "Aparato Digestivo",
    agenda: "Dra. Sanz Oria · consulta 3B",
    motivo: "Dispepsia de 5 meses que no responde a IBP (derivación DER-2026-04871)",
    derivacion: "DER-2026-04871"
  }
};

window.DOCS = [
 {id:"D01", fecha:"18/03/2015", orden:"2015-03-18", tipo:"Informe de endoscopia", servicio:"Aparato Digestivo · Endoscopias", autor:"Dr. L. Marchena Ruiz", texto:
`COLONOSCOPIA COMPLETA. Indicacion: SOH + en cribado poblacional.
Preparacion Boston 8/9. Se progresa hasta ciego (valvula ileocecal y orificio
apendicular identificados). Retirada lenta.
Hallazgos: polipo sesil de 9 mm en sigma, Paris 0-Is. Resecado con asa de
diatermia, recuperado. Resto de mucosa colonica sin lesiones. Hemorroides
internas grado I.
AP (anatomia patologica, informe 02/04/2015): adenoma tubular con displasia de
bajo grado, margenes libres.
RECOMENDACION: control endoscopico a los 5 anos (2020).`},

 {id:"D02", fecha:"09/02/2016", orden:"2016-02-09", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dr. M. Pardo Sierra", texto:
`Acude por cifras altas de TA tomadas en farmacia. AMPA 2 semanas: media
148/92. Asintomatica. No cefalea. No disnea.
Peso 71 kg, talla 162 cm, IMC 27.
Analitica: funcion renal normal, Cr 0.8, K 4.2, col total 221, LDL 142.
ECG: ritmo sinusal, sin alteraciones.
DX: HTA esencial grado 1. Inicio enalapril 10 mg/24h. Dieta hiposodica.
Control en 2 meses.`},

 {id:"D03", fecha:"21/05/2017", orden:"2017-05-21", tipo:"Informe de alta de Urgencias", servicio:"Urgencias Hospitalarias", autor:"Dra. N. Olmeda Paz", texto:
`MOTIVO: dolor lumbar izq de inicio brusco, irradiado a fosa iliaca, con
nauseas. No fiebre. No sdme miccional.
EF: puñopercusion renal izq +. Abdomen blando, no peritonismo.
Analitica: leucos 9.100, Cr 0.9, PCR 4. Sedimento: microhematuria.
ECO ABDOMINAL (21/05/2017): litiasis de 4 mm en ureter proximal izquierdo con
minima ectasia pielica. Higado, vesicula y via biliar normales. Pancreas
normal. Riñon derecho normal.
Tratamiento en urgencias: dexketoprofeno iv + hidratacion, cede el dolor.
ALTA. Dco: colico renal izquierdo no complicado. Tto: dexketoprofeno 25 mg
c/8h si dolor, abundante liquido, filtrar orina. Control por su MAP.`},

 {id:"D04", fecha:"14/11/2018", orden:"2018-11-14", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`Lumbalgia mecanica de 10 dias, sin irradiacion, no sdme cauda. Trabaja de
auxiliar de geriatria, moviliza pacientes.
EF: contractura paravertebral lumbar, Lasegue -.
Tto: ibuprofeno 600 c/8h 5 dias + paracetamol de rescate. Calor local.
Se le explica que si lo necesita puede repetir el ibuprofeno en episodios
puntuales, no mas de 5 dias seguidos.`},

 {id:"D05", fecha:"03/06/2020", orden:"2020-06-03", tipo:"Informe de alta de Urgencias", servicio:"Urgencias Hospitalarias", autor:"Dr. J. Belmonte Arce", texto:
`MOTIVO: epigastralgia de 3 dias, ardor, nauseas sin vomitos. Peor con el
estomago vacio. Tomo ibuprofeno la semana pasada por dolor de espalda.
EF: BEG, afebril, TA 139/85. Abdomen blando, molestias en epigastrio sin
defensa. Murphy -. Blumberg -.
ECG normal. Analitica: Hb 13.1, leucos 7.800, amilasa 54, bilirrubina 0.6,
GOT 21 GPT 19, Cr 0.8, PCR 3.
ECO ABDOMINAL (03/06/2020): higado de tamaño y ecoestructura normales, sin
LOEs. Vesicula alitiasica, pared fina. Via biliar no dilatada. Pancreas sin
alteraciones valorables. Bazo y riñones normales. No liquido libre.
ALTA. Dco: gastritis / dispepsia probablemente por AINE. Tto: omeprazol 20 mg
antes del desayuno 4 semanas, evitar AINEs. Si no mejora, valorar por su MAP.`},

 {id:"D06", fecha:"27/01/2021", orden:"2021-01-27", tipo:"Hoja de quirófano y alta", servicio:"Ginecología", autor:"Dra. P. Aguirre Lastra", texto:
`INGRESO 26/01/2021. ALTA 28/01/2021.
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
3 meses. NO ADMINISTRAR METAMIZOL (Nolotil) NI DERIVADOS PIRAZOLONICOS.`},

 {id:"D07", fecha:"12/03/2021", orden:"2021-03-12", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`Revision post IQ ginecologica. Buena evolucion, herida cerrada. Hb de control
12.4 (previa 10.1). Sigue con hierro oral.
Se actualiza hoja de alergias: METAMIZOL (exantema y edema palpebral en el
postoperatorio, enero 2021).
Se recuerda no tomar Nolotil ni compuestos con metamizol.`},

 {id:"D08", fecha:"05/10/2021", orden:"2021-10-05", tipo:"Nota de enfermería", servicio:"Atención Primaria · Enfermería", autor:"E. Vilches (enfermera)", texto:
`Control de HTA. TA 132/80, FC 72. Peso 69.8 kg. Adherencia buena al enalapril.
Se revisa tecnica de AMPA. Consejo dietetico. Proxima revision en 6 meses.
Vacuna gripe administrada.`},

 {id:"D09", fecha:"19/04/2022", orden:"2022-04-19", tipo:"Analítica", servicio:"Laboratorio · Atención Primaria", autor:"—", texto:
`ANALITICA DE CONTROL (ayunas).
Hemograma: Hb 13.2, Hto 40%, VCM 88, leucos 6.900, plaquetas 251.000.
Bioquimica: glucosa 98, HbA1c 5.4%, Cr 0.82, FG >90, Na 140, K 4.1,
col total 214, LDL 138, HDL 56, TG 101, GOT 22, GPT 20, GGT 28, FA 61,
bilirrubina total 0.5, TSH 1.9, ferritina 48, vitamina D 18 (baja).
Sangre oculta en heces (cribado): NEGATIVA.`},

 {id:"D10", fecha:"08/06/2023", orden:"2023-06-08", tipo:"Informe de consulta", servicio:"Oftalmología", autor:"Dr. S. Reina Cobo", texto:
`Revision por ojo seco y cansancio visual. AV 0.9 ambos ojos con correccion.
PIO 15/16. Polo anterior: menisco lagrimal bajo. Fondo de ojo sin
retinopatia hipertensiva. Dco: sdme de ojo seco leve. Tto: lagrimas
artificiales sin conservantes 4-6 veces/dia. Alta a su MAP.`},

 {id:"D11", fecha:"14/11/2024", orden:"2024-11-14", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`Acude por insomnio de conciliacion y ansiedad. Cuida de su madre con demencia
en domicilio desde hace 2 anos, duerme 4-5 h. No animo bajo mantenido, no
ideacion. GAD-7: 12.
Se ofrece derivacion a psicologia (lista larga) y se pauta lorazepam 1 mg al
acostarse maximo 4 semanas. Medidas de higiene del sueño.
Refiere seguir tomando ibuprofeno 600 "cuando le duele la espalda", unos 3-4
dias al mes. Se insiste en que lo limite y en que tome paracetamol primero.`},

 {id:"D12", fecha:"02/02/2025", orden:"2025-02-02", tipo:"Informe de alta de Urgencias", servicio:"Urgencias Hospitalarias", autor:"Dra. R. Quiles Amat", texto:
`MOTIVO: dolor centrotoracico opresivo de 20 min en reposo, sin irradiacion,
con sensacion de angustia. Ya cedido a su llegada.
EF: BEG, TA 144/88, FC 78, SatO2 98%. AC ritmico sin soplos. AP normal.
ECG: RS a 76 lpm, sin alteraciones de la repolarizacion. Troponina ultrasensible
seriada (0 y 3 h): normal, sin curva. Rx torax: sin hallazgos.
ALTA. Dco: dolor toracico atipico, probablemente de origen ansioso.
No cardiopatia isquemica demostrada. Se recomienda control por su MAP.
Nota: la paciente pregunta por su dolor de estomago, se le indica que lo
comente con su medica de familia.`},

 {id:"D13", fecha:"18/05/2026", orden:"2026-05-18", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`Desde hace unas 3 semanas molestia epigastrica despues de comer, con
sensacion de llenarse enseguida. No disfagia, no vomitos, no perdida de peso.
Ritmo intestinal normal, no rectorragia. Duerme algo mejor.
EF: abdomen blando, molestia a la palpacion en epigastrio, no masas.
Dco: dispepsia funcional / posible dispepsia por AINE (sigue tomando
ibuprofeno a demanda).
Tto: omeprazol 20 mg/24h antes del desayuno 8 semanas. Suspender ibuprofeno,
paracetamol si dolor. Revision al acabar.`},

 {id:"D14", fecha:"21/07/2026", orden:"2026-07-21", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`Revision. Con el omeprazol mejoro bastante, al dejarlo hace 2 semanas han
vuelto las molestias igual que antes. Sigue sin disfagia, sin vomitos, peso
estable (68 kg). Dice haber dejado el ibuprofeno casi del todo.
Se repite tanda de omeprazol 20 mg/24h otras 8 semanas y se solicita analitica
completa y test de H. pylori en heces. Si no mejora, derivar a Digestivo.`},

 {id:"D15", fecha:"12/09/2026", orden:"2026-09-12", tipo:"Analítica", servicio:"Laboratorio · Atención Primaria", autor:"—", texto:
`ANALITICA (ayunas).
Hemograma: Hb 13.4, Hto 41%, VCM 89, leucos 7.100, plaquetas 243.000.
Bioquimica: glucosa 94, HbA1c 5.5%, Cr 0.79, FG >90, Na 141, K 4.3,
proteinas totales 7.1, albumina 4.4, col total 209, LDL 131, HDL 58, TG 94,
GOT 23, GPT 24, GGT 31, FA 58, bilirrubina total 0.5, LDH 180,
amilasa 49, lipasa 32, ferritina 52, hierro 84, TSH 2.1, PCR 1.8,
Ac antitransglutaminasa IgA: negativos. IgA total normal.`},

 {id:"D16", fecha:"18/09/2026", orden:"2026-09-18", tipo:"Resultado de pruebas", servicio:"Laboratorio · Atención Primaria", autor:"—", texto:
`Antigeno de Helicobacter pylori en heces: NEGATIVO.
Sangre oculta en heces (inmunoquimica): NEGATIVA.
Coprocultivo: no se aislan enteropatogenos.
Parasitos en heces (3 muestras): negativo.`},

 {id:"D17", fecha:"06/10/2026", orden:"2026-10-06", tipo:"Nota de enfermería", servicio:"Atención Primaria · Enfermería", autor:"E. Vilches (enfermera)", texto:
`Control. TA 136/84, FC 70. Peso 68 kg, talla 162 cm, IMC 25.9.
Refiere que las molestias de estomago siguen igual y que esta preocupada.
Comenta que su madre fallecio en julio y que ahora duerme mejor.
Toma: enalapril 10 mg/24h, omeprazol 20 mg/24h (el que le dieron en julio, lo
ha seguido tomando), paracetamol a demanda, lagrimas artificiales.
No toma lorazepam desde hace mas de un ano.`},

 {id:"D18", fecha:"12/10/2026", orden:"2026-10-12", tipo:"Derivación a consulta externa", servicio:"Atención Primaria", autor:"Dra. C. Herrero Nieto", texto:
`MOTIVO: dispepsia. estudio

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
gracias`}
];

// Lo que el agente responde. Cada hecho cita literalmente su documento:
// `cita` tiene que existir tal cual dentro del texto de `doc`.
window.HECHOS = [
 {k:"resumen", t:"Mujer de 58 años con dispepsia epigástrica posprandial desde mayo de 2026, con plenitud precoz, sin disfagia, sin vómitos y con peso estable.", doc:"D13", cita:"molestia epigastrica despues de comer, con\nsensacion de llenarse enseguida"},
 {k:"resumen", t:"Dos tandas de omeprazol de 8 semanas: mejora mientras lo toma y recae al suspenderlo.", doc:"D14", cita:"Con el omeprazol mejoro bastante, al dejarlo hace 2 semanas han\nvuelto las molestias igual que antes"},
 {k:"resumen", t:"Estudio de Primaria ya completo y sin hallazgos: analítica con perfil hepático, celiaquía, H. pylori y sangre oculta.", doc:"D15", cita:"Ac antitransglutaminasa IgA: negativos"},
 {k:"resumen|alarma", t:"No hay datos de alarma registrados en ninguna nota: sin disfagia, sin vómitos, sin pérdida de peso (68 kg estables desde julio) y sin anemia (Hb 13,4).", doc:"D14", cita:"Sigue sin disfagia, sin vomitos, peso\nestable (68 kg)"},
 {k:"contradiccion|alergias", t:"ALERGIA A METAMIZOL documentada en 2021. La derivación de hoy dice «NAMC» (sin alergias conocidas).", doc:"D06", cita:"NO ADMINISTRAR METAMIZOL (Nolotil) NI DERIVADOS PIRAZOLONICOS."},
 {k:"contradiccion|cirugias", t:"Sí tiene cirugía previa: histerectomía total laparoscópica con doble anexectomía en enero de 2021. La derivación dice «no IQ previas».", doc:"D06", cita:"IQ 27/01/2021: HISTERECTOMIA TOTAL LAPAROSCOPICA con doble anexectomia."},
 {k:"contradiccion|aine|medicacion", t:"Toma ibuprofeno 600 unos 3-4 días al mes por lumbalgia, anotado en 2024 y aún en mayo de 2026. La derivación dice «no AINEs de forma habitual», y el episodio de 2020 ya se atribuyó a AINE.", doc:"D11", cita:"Refiere seguir tomando ibuprofeno 600 \"cuando le duele la espalda\", unos 3-4\ndias al mes."},
 {k:"contradiccion|pendiente|endoscopia", t:"Adenoma tubular de 9 mm resecado en 2015 con control endoscópico recomendado para 2020: no consta que se haya hecho. Han pasado 11 años desde la colonoscopia.", doc:"D01", cita:"RECOMENDACION: control endoscopico a los 5 anos (2020)."},
 {k:"pruebas|eco", t:"Ecografía abdominal normal, pero de junio de 2020 (hace 6 años), hecha en Urgencias por el mismo tipo de dolor.", doc:"D05", cita:"ECO ABDOMINAL (03/06/2020): higado de tamaño y ecoestructura normales, sin\nLOEs. Vesicula alitiasica, pared fina."},
 {k:"pruebas|eco", t:"Antes hubo otra ecografía en 2017 por cólico renal: litiasis de 4 mm, hígado y vía biliar normales.", doc:"D03", cita:"ECO ABDOMINAL (21/05/2017): litiasis de 4 mm en ureter proximal izquierdo"},
 {k:"pruebas|analitica", t:"Analítica del 12/09/2026 sin alteraciones: Hb 13,4, GPT 24, GGT 31, amilasa 49, lipasa 32, ferritina 52, PCR 1,8.", doc:"D15", cita:"GOT 23, GPT 24, GGT 31, FA 58, bilirrubina total 0.5, LDH 180,"},
 {k:"pruebas|hpylori", t:"H. pylori en heces NEGATIVO el 18/09/2026. Nunca ha recibido tratamiento erradicador.", doc:"D16", cita:"Antigeno de Helicobacter pylori en heces: NEGATIVO."},
 {k:"pruebas|soh", t:"Sangre oculta en heces negativa dos veces: en el cribado de 2022 y el 18/09/2026.", doc:"D16", cita:"Sangre oculta en heces (inmunoquimica): NEGATIVA."},
 {k:"pruebas|endoscopia", t:"Colonoscopia completa en 2015 (pólipo en sigma resecado). No consta ninguna gastroscopia en toda la historia.", doc:"D01", cita:"Hallazgos: polipo sesil de 9 mm en sigma, Paris 0-Is. Resecado con asa de\ndiatermia, recuperado."},
 {k:"pruebas|celiaquia", t:"Celiaquía descartada: anticuerpos antitransglutaminasa IgA negativos con IgA total normal (12/09/2026).", doc:"D15", cita:"Ac antitransglutaminasa IgA: negativos. IgA total normal."},
 {k:"medicacion", t:"Medicación activa en octubre de 2026: enalapril 10 mg/24h, omeprazol 20 mg/24h, paracetamol a demanda y lágrimas artificiales. Lorazepam suspendido hace más de un año.", doc:"D17", cita:"Toma: enalapril 10 mg/24h, omeprazol 20 mg/24h (el que le dieron en julio, lo\nha seguido tomando), paracetamol a demanda, lagrimas artificiales."},
 {k:"medicacion|ibp", t:"Ojo con el omeprazol: la enfermera anota el 06/10/2026 que lo ha seguido tomando sin interrupción desde julio, así que la «falta de respuesta» se mide sobre un IBP que no se ha suspendido.", doc:"D17", cita:"omeprazol 20 mg/24h (el que le dieron en julio, lo\nha seguido tomando)"},
 {k:"antecedentes", t:"HTA esencial grado 1 desde 2016, en tratamiento con enalapril y bien controlada (TA 136/84 en octubre).", doc:"D02", cita:"DX: HTA esencial grado 1. Inicio enalapril 10 mg/24h."},
 {k:"antecedentes", t:"Anemia ferropénica secundaria a menometrorragias, resuelta tras la histerectomía de 2021 (Hb 10,1 → 12,4).", doc:"D07", cita:"Hb de control\n12.4 (previa 10.1)"},
 {k:"antecedentes|urgencias", t:"Tres pasos por Urgencias: cólico renal en 2017, epigastralgia por AINE en 2020 y dolor torácico atípico en 2025 con troponinas y ECG normales.", doc:"D12", cita:"Dco: dolor toracico atipico, probablemente de origen ansioso.\nNo cardiopatia isquemica demostrada."},
 {k:"antecedentes|salud_mental", t:"Ansiedad e insomnio en 2024 mientras cuidaba a su madre con demencia (GAD-7 12), con lorazepam en pauta corta. Su madre falleció en julio de 2026 y ahora duerme mejor.", doc:"D11", cita:"Cuida de su madre con demencia\nen domicilio desde hace 2 anos, duerme 4-5 h."},
 {k:"peso", t:"Peso 68 kg y talla 162 cm (IMC 25,9) el 06/10/2026. Venía de 71 kg en 2016 y 69,8 en 2021: baja lenta de años, no pérdida reciente.", doc:"D17", cita:"Peso 68 kg, talla 162 cm, IMC 25.9."},
 {k:"motivo", t:"Viene por una primera consulta de Digestivo pedida el 12/10/2026 por dispepsia de 5 meses que no responde a IBP.", doc:"D18", cita:"dispepsia no investigada que no responde a IBP, ruego valoracion y estudio."},
 {k:"falta|pendiente", t:"Lo que no hay en la historia: ninguna gastroscopia, el control de colonoscopia de 2020, y una ecografía abdominal reciente (la última es de 2020).", doc:"D01", cita:"control endoscopico a los 5 anos (2020)"},
 {k:"social", t:"Trabaja como auxiliar de geriatría y moviliza pacientes: ahí está el origen de la lumbalgia por la que toma ibuprofeno.", doc:"D04", cita:"Trabaja de\nauxiliar de geriatria, moviliza pacientes."},
 {k:"familia", t:"No consta ningún antecedente familiar en la historia: ni cáncer digestivo, ni enfermedad inflamatoria, ni celiaquía.", doc:null, cita:null},
 {k:"habitos", t:"No fumadora, según la derivación. No consta registro de consumo de alcohol en ninguna nota.", doc:"D18", cita:"no fumadora."}
];

// Preguntas sugeridas: cada una con las claves de hecho que contesta.
window.PREGUNTAS = [
 {q:"¿Qué le ha pasado?", k:["resumen"]},
 {q:"¿Qué pruebas tiene hechas y de cuándo?", k:["pruebas"]},
 {q:"¿Hay algo que contradiga la derivación?", k:["contradiccion"]},
 {q:"¿Alergias?", k:["alergias"]},
 {q:"¿Qué medicación toma?", k:["medicacion"]},
 {q:"¿Tiene datos de alarma?", k:["alarma"]},
 {q:"¿Qué falta por hacer?", k:["falta","pendiente"]},
 {q:"¿Cirugías previas?", k:["cirugias"]},
 {q:"¿Cómo ha ido el peso?", k:["peso"]},
 {q:"¿Por qué viene hoy?", k:["motivo"]}
];

// Vocabulario para encaminar la pregunta escrita a mano hacia claves de hecho.
window.LEXICO = [
 {k:"resumen", v:["que le ha pasado","resumen","resume","historia","cuentame","que tiene","situacion","por que esta aqui","caso"]},
 {k:"alarma", v:["alarma","bandera","roja","grave","urgente","riesgo","sintoma de alarma","perdida de peso","sangrado","disfagia","anemia"]},
 {k:"contradiccion", v:["contradic","discrepan","cuadra","falso","error","mal puesto","no coincide","difiere","contrasta","fiable"]},
 {k:"alergias", v:["alergia","alergica","namc","metamizol","nolotil","intolerancia"]},
 {k:"cirugias", v:["cirugia","iq","operad","quirofano","intervencion","histerectomia"]},
 {k:"medicacion", v:["medicacion","medicamento","tratamiento","toma","farmaco","pastilla","tto","omeprazol","ibp","enalapril","lorazepam"]},
 {k:"aine", v:["aine","ibuprofeno","antiinflamatorio","nolotil","analgesico"]},
 {k:"pruebas", v:["prueba","pruebas","analitica","analisis","eco","ecografia","endoscopia","gastroscopia","colonoscopia","hpylori","h pylori","helicobacter","sangre oculta","soh","celiaquia","resultado","hecho","hecha"]},
 {k:"endoscopia", v:["endoscopia","gastroscopia","colonoscopia","polipo","adenoma","colon"]},
 {k:"eco", v:["eco","ecografia","abdominal","higado","vesicula","biliar"]},
 {k:"analitica", v:["analitica","analisis","hemograma","bioquimica","hb","hemoglobina","transaminasas","gpt","got","ferritina","pcr","amilasa","lipasa"]},
 {k:"hpylori", v:["pylori","helicobacter","hp","erradica"]},
 {k:"soh", v:["sangre oculta","soh","heces","cribado"]},
 {k:"celiaquia", v:["celiaquia","celiaca","transglutaminasa","gluten"]},
 {k:"antecedentes", v:["antecedente","ap","previo","historico","cronico","hta","tension","diabetes","anemia","ingreso","urgencias"]},
 {k:"salud_mental", v:["ansiedad","animo","depresion","insomnio","duerme","sueño","psico","estres","cuidadora","madre"]},
 {k:"peso", v:["peso","kilos","kg","imc","adelgaz","engord","talla"]},
 {k:"falta", v:["falta","pendiente","no se ha hecho","queda","pedir","solicitar","que hago","siguiente paso"]},
 {k:"pendiente", v:["pendiente","caducad","vencid","control","seguimiento","revision"]},
 {k:"motivo", v:["motivo","por que viene","derivacion","cita de hoy","hoy","consulta de hoy","deriva"]},
 {k:"social", v:["trabajo","trabaja","social","familia","vive","cuidado","laboral"]},
 {k:"familia", v:["familiar","familia","padre","madre","hermano","herencia","genetic"]},
 {k:"habitos", v:["fuma","tabaco","alcohol","bebe","habito","drogas","dieta","ejercicio"]},
 {k:"ibp", v:["ibp","omeprazol","inhibidor","protector","respuesta al tratamiento"]},
 {k:"anticoagulacion", v:["anticoagul","acenocumarol","sintrom","inr","apixaban","rivaroxaban","heparina","antiagreg","adiro","aas","sangrado","hemorragia"]},
 {k:"renal", v:["renal","riñon","rinon","creatinina","filtrado","erc","nefro","aclaramiento"]},
 {k:"imagen", v:["tac","scanner","escaner","radiografia","rx","resonancia","contraste","yodo","imagen"]},
 {k:"hierro", v:["hierro","ferritina","ferropenia","transfusion","anemia","hemoglobina"]}
];
