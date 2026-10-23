// Segundo paciente de la demo + los hallazgos clave de los dos.
// Los "hallazgos" son la lista contra la que se compara lo que saca el médico
// leyendo y lo que saca preguntando a Jano. Todo sintético.

window.P2 = {
 paciente: {
  nombre: "Ramón Esteve Gálvez",
  nhc: "PAC-02856",
  edad: 72,
  sexo: "Hombre",
  nacimiento: "11/09/1954",
  cipa: "ESGL540911XXX",
  centro: "C.S. Ángela Uriarte",
  medico_ap: "Dr. T. Molina Esparza",
  telefono: "9XX XXX XXX",
  contacto: "Llamada telefónica",
  acc: "Viene con su hijo. Oye mal por el teléfono fijo",
  cita: {
    fecha: "Hoy, 23/10/2026 · 17:10",
    tipo: "Primera consulta",
    servicio: "Aparato Digestivo",
    agenda: "Dra. Sanz Oria · consulta 3B",
    motivo: "Anemia ferropénica a estudio (derivación DER-2026-04902)",
    derivacion: "DER-2026-04902"
  }
 },

 docs: [
 {id:"E01", fecha:"06/05/2012", orden:"2012-05-06", tipo:"Informe de endoscopia", servicio:"Aparato Digestivo · Endoscopias", autor:"Dra. I. Salcedo Vera", texto:
`GASTROSCOPIA. Indicacion: pirosis y epigastralgia de meses.
Esofago: hernia de hiato por deslizamiento de unos 3 cm. Esofagitis por reflujo
grado A de Los Angeles. Estomago: mucosa con gastritis antral eritematosa, sin
ulceras. Pilrico permeable. Duodeno hasta 2a porcion sin lesiones.
Biopsias antrales: gastritis cronica, H. pylori NEGATIVO.
Dco: ERGE con esofagitis grado A + hernia de hiato. Tto: omeprazol 20 mg/24h,
medidas antirreflujo.`},

 {id:"E02", fecha:"22/09/2016", orden:"2016-09-22", tipo:"Informe de endoscopia", servicio:"Aparato Digestivo · Endoscopias", autor:"Dr. L. Marchena Ruiz", texto:
`COLONOSCOPIA COMPLETA. Indicacion: estudio de anemia leve y cambio de ritmo.
Preparacion Boston 7/9. Se alcanza ciego.
Hallazgos: diverticulosis de sigma de moderada cuantia, sin signos de
diverticulitis. Polipo de 5 mm en colon transverso, resecado con pinza.
AP (informe 06/10/2016): polipo hiperplasico, sin displasia.
Hemorroides internas grado II.
RECOMENDACION: dado el resultado, control endoscopico a los 10 anos (2026) o
antes si clinica. Cribado de sangre oculta en el intervalo.`},

 {id:"E03", fecha:"14/10/2016", orden:"2016-10-14", tipo:"Informe de radiología", servicio:"Radiodiagnóstico", autor:"Dra. M. Cuesta Barrio", texto:
`TAC ABDOMINAL SIN CONTRASTE (se suspende el contraste yodado).
Nota: durante la administracion de contraste yodado el paciente presenta
urticaria generalizada y sensacion de opresion faringea. Se suspende la
inyeccion, se administra corticoide y antihistaminico iv con buena respuesta.
SE REGISTRA ALERGIA A CONTRASTES YODADOS.
Estudio sin contraste: diverticulosis sigmoidea. Quiste simple en rinon
derecho de 22 mm. Resto sin hallazgos valorables. No adenopatias.`},

 {id:"E04", fecha:"03/03/2018", orden:"2018-03-03", tipo:"Informe de consulta", servicio:"Cardiología", autor:"Dr. A. Peñalver Soto", texto:
`Motivo: palpitaciones y disnea de esfuerzo. ECG y Holter: FIBRILACION
AURICULAR PERMANENTE, FC media 82 lpm. Ecocardiograma: FEVI 58%, auricula
izquierda dilatada, sin valvulopatia significativa.
CHA2DS2-VASc 3 (edad, HTA). HAS-BLED 2.
Dco: FA no valvular permanente. Se inicia ANTICOAGULACION ORAL con
acenocumarol segun pauta de hematologia + bisoprolol 2.5 mg/24h.
Se explican signos de sangrado. Control de INR en su centro de salud.`},

 {id:"E05", fecha:"27/11/2019", orden:"2019-11-27", tipo:"Informe de alta de Urgencias", servicio:"Urgencias Hospitalarias", autor:"Dr. F. Izaguirre Mota", texto:
`MOTIVO: deposiciones negras de 2 dias, mareo y debilidad. Anticoagulado con
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
reintroducir acenocumarol segun hematologia. Control en consulta de Digestivo.`},

 {id:"E06", fecha:"16/01/2020", orden:"2020-01-16", tipo:"Informe de consulta", servicio:"Hematología", autor:"Dra. B. Sanchidrián Lopo", texto:
`Anemia ferropenica postsangrado digestivo. Hb 10.1, ferritina 9, IST 8%.
Mala tolerancia al hierro oral (estreñimiento y nauseas).
Se administran 2 dosis de HIERRO CARBOXIMALTOSA IV (16/01 y 30/01/2020).
Control a los 3 meses: Hb 13.1, ferritina 96. Alta de hematologia.
Se recomienda vigilar nuevas perdidas: el paciente esta anticoagulado y tiene
una angiodisplasia gastrica tratada.`},

 {id:"E07", fecha:"09/06/2021", orden:"2021-06-09", tipo:"Hoja de quirófano y alta", servicio:"Traumatología", autor:"Dr. G. Rendueles Pina", texto:
`Dco: gonartrosis tricompartimental derecha avanzada.
IQ 08/06/2021: ARTROPLASTIA TOTAL DE RODILLA DERECHA. Anestesia intradural.
Terapia puente con heparina de bajo peso molecular por anticoagulacion previa.
Postoperatorio sin complicaciones, Hb al alta 10.9.
Alta a rehabilitacion. Tto: paracetamol, HBPM 40 mg/24h hasta reiniciar
acenocumarol, rehabilitacion 6 semanas.`},

 {id:"E08", fecha:"21/04/2023", orden:"2023-04-21", tipo:"Informe de consulta", servicio:"Nefrología", autor:"Dra. C. Ibarrola Sanz", texto:
`Motivo: deterioro lento de funcion renal. Cr 1.52, FG estimado (CKD-EPI)
48 ml/min/1.73, cociente albumina/creatinina 42 mg/g. Ecografia renal: riñones
de tamaño normal, quiste simple derecho ya conocido.
Dco: ENFERMEDAD RENAL CRONICA ESTADIO 3a, probablemente vascular.
Recomendaciones: evitar AINEs, ajustar farmacos a FG, evitar contrastes
yodados (ademas tiene alergia registrada), control anual.`},

 {id:"E09", fecha:"08/02/2024", orden:"2024-02-08", tipo:"Nota de consulta", servicio:"Atención Primaria", autor:"Dr. T. Molina Esparza", texto:
`Revision de cronicos. TA 138/82, FC 74. INR en rango (2.4), controles
estables.
Revisando la medicacion: el paciente sigue tomando ADIRO 100 que le pautaron
hace años "para el corazon", ademas del acenocumarol. No consta indicacion
actual para la doble terapia y tiene antecedente de sangrado digestivo.
Se le indica suspender el Adiro y se avisa de que lo comente en cardiologia.
Pendiente de confirmar que lo ha retirado.
Resto: artrosis de rodillas, usa paracetamol. Se insiste en NO tomar AINEs
por la funcion renal.`},

 {id:"E10", fecha:"19/09/2024", orden:"2024-09-19", tipo:"Informe de consulta", servicio:"Oftalmología", autor:"Dra. V. Loriente Prats", texto:
`Catarata nuclear bilateral, mayor en OD. AV 0.5 OD / 0.7 OI.
Se incluye en lista de espera para facoemulsificacion de OD. Se avisa de que
esta anticoagulado: valoracion preoperatoria antes de la IQ.`},

 {id:"E11", fecha:"12/03/2026", orden:"2026-03-12", tipo:"Analítica", servicio:"Laboratorio · Atención Primaria", autor:"—", texto:
`ANALITICA DE CONTROL.
Hemograma: Hb 10.2, Hto 32%, VCM 78, HCM 25, leucos 6.400, plaquetas 262.000.
Bioquimica: glucosa 101, Cr 1.58, FG 46, urea 61, Na 139, K 4.6,
proteinas totales 6.9, albumina 4.0, GOT 19, GPT 17, GGT 24,
ferritina 11, hierro 42, IST 9%, transferrina 310, B12 388, folato 7.2.
INR 2.6.
Comentario del laboratorio: anemia microcitica con perfil ferropenico.`},

 {id:"E12", fecha:"07/07/2026", orden:"2026-07-07", tipo:"Nota de enfermería", servicio:"Atención Primaria · Enfermería", autor:"M. Tordesillas (enfermero)", texto:
`Control de cronicos y de INR. INR 2.3. TA 134/80.
Peso 72 kg (en noviembre de 2025 pesaba 78 kg). Talla 171 cm.
El paciente dice que come menos porque "se llena enseguida" y que esta mas
cansado al andar. No refiere dolor. Deposiciones oscuras de forma intermitente
segun su hijo, el paciente no lo tiene claro.
Se le dice que lo comente con su medico en la proxima cita.`},

 {id:"E13", fecha:"28/09/2026", orden:"2026-09-28", tipo:"Analítica", servicio:"Laboratorio · Atención Primaria", autor:"—", texto:
`ANALITICA.
Hemograma: Hb 9.6, Hto 30%, VCM 76, leucos 6.100, plaquetas 271.000,
reticulocitos 1.1%.
Bioquimica: Cr 1.61, FG 44, ferritina 8, hierro 38, IST 7%, PCR 4.2,
albumina 3.9, calcio 9.3, LDH 198. INR 2.5.
Sangre oculta en heces (inmunoquimica): POSITIVA.
Vitamina B12 y folato normales. TSH 2.3.`},

 {id:"E14", fecha:"14/10/2026", orden:"2026-10-14", tipo:"Derivación a consulta externa", servicio:"Atención Primaria", autor:"Dr. T. Molina Esparza", texto:
`MOTIVO: anemia ferropenica a estudio

varon de 72a con anemia microcitica Hb 9.6, ferritina 8, SOH +. astenia.
no antecedentes digestivos de interes. no dolor abdominal. no rectorragia.
tto: acenocumarol, bisoprolol, omeprazol, paracetamol.
analitica 28/09 en HCE.
derivo para valoracion y completar estudio, probable origen digestivo.
rgo valoracion. gracias`}
 ],

 hechos: [
 {k:"resumen", t:"Varón de 72 años con anemia microcítica ferropénica que empeora: Hb 10,2 en marzo y 9,6 en septiembre de 2026, con ferritina de 11 a 8.", doc:"E13", cita:"Hemograma: Hb 9.6, Hto 30%, VCM 76, leucos 6.100, plaquetas 271.000,"},
 {k:"resumen|soh", t:"Sangre oculta en heces POSITIVA el 28/09/2026: la pérdida es digestiva hasta que se demuestre lo contrario.", doc:"E13", cita:"Sangre oculta en heces (inmunoquimica): POSITIVA."},
 {k:"resumen|alarma|peso", t:"Pérdida de 6 kg en 8 meses y sensación de plenitud precoz, anotadas por enfermería en julio de 2026. No aparecen en la derivación.", doc:"E12", cita:"Peso 72 kg (en noviembre de 2025 pesaba 78 kg)."},
 {k:"alarma|resumen", t:"El hijo refiere deposiciones oscuras intermitentes. Está escrito en la nota de enfermería y en ninguna otra parte.", doc:"E12", cita:"Deposiciones oscuras de forma intermitente\nsegun su hijo, el paciente no lo tiene claro."},
 {k:"contradiccion|antecedentes|endoscopia", t:"Sí tiene antecedentes digestivos de peso: angiodisplasia gástrica con sangrado activo en 2019, tratada con argón. La derivación dice «no antecedentes digestivos de interés».", doc:"E05", cita:"GASTROSCOPIA URGENTE (27/11/2019): ANGIODISPLASIA GASTRICA en cuerpo-antro,\ncon sangrado activo en babeo."},
 {k:"contradiccion|antecedentes", t:"También tiene hernia de hiato con esofagitis grado A (2012) y diverticulosis con pólipo hiperplásico resecado (2016).", doc:"E01", cita:"Dco: ERGE con esofagitis grado A + hernia de hiato."},
 {k:"contradiccion|medicacion|anticoagulacion", t:"Está anticoagulado con acenocumarol por fibrilación auricular permanente desde 2018: hay que planificar la endoscopia y la posible terapia puente.", doc:"E04", cita:"Dco: FA no valvular permanente. Se inicia ANTICOAGULACION ORAL con\nacenocumarol segun pauta de hematologia + bisoprolol 2.5 mg/24h."},
 {k:"contradiccion|medicacion|aine", t:"Riesgo de doble antitrombótico: en 2024 seguía tomando Adiro 100 además del acenocumarol sin indicación, se le dijo que lo suspendiera y quedó «pendiente de confirmar». La derivación no lo menciona.", doc:"E09", cita:"el paciente sigue tomando ADIRO 100 que le pautaron\nhace años \"para el corazon\", ademas del acenocumarol."},
 {k:"alergias", t:"ALERGIA A CONTRASTES YODADOS: urticaria y opresión faríngea en el TAC de 2016. Condiciona cualquier TAC con contraste.", doc:"E03", cita:"SE REGISTRA ALERGIA A CONTRASTES YODADOS."},
 {k:"antecedentes|renal", t:"Enfermedad renal crónica estadio 3a (FG 44): obliga a ajustar fármacos, evita AINEs y pesa en la preparación de la colonoscopia.", doc:"E08", cita:"Dco: ENFERMEDAD RENAL CRONICA ESTADIO 3a, probablemente vascular."},
 {k:"pendiente|falta|endoscopia", t:"El control de colonoscopia recomendado para 2026 está pendiente: la última es de septiembre de 2016.", doc:"E02", cita:"RECOMENDACION: dado el resultado, control endoscopico a los 10 anos (2026) o"},
 {k:"pruebas|endoscopia", t:"Endoscopias previas: gastroscopia 2012 (hernia de hiato, esofagitis A), colonoscopia 2016 (diverticulosis y pólipo hiperplásico) y gastroscopia urgente 2019 (angiodisplasia tratada con argón).", doc:"E02", cita:"Hallazgos: diverticulosis de sigma de moderada cuantia, sin signos de\ndiverticulitis."},
 {k:"pruebas|analitica", t:"Analítica del 28/09/2026: Hb 9,6, VCM 76, ferritina 8, IST 7 %, PCR 4,2, INR 2,5, B12 y folato normales.", doc:"E13", cita:"Bioquimica: Cr 1.61, FG 44, ferritina 8, hierro 38, IST 7%, PCR 4.2,"},
 {k:"pruebas|imagen", t:"TAC abdominal de 2016, hecho SIN contraste por la reacción al yodo: diverticulosis y quiste renal simple.", doc:"E03", cita:"Estudio sin contraste: diverticulosis sigmoidea."},
 {k:"pruebas|hpylori", t:"H. pylori negativo en biopsias antrales de 2012. No hay determinación posterior.", doc:"E01", cita:"Biopsias antrales: gastritis cronica, H. pylori NEGATIVO."},
 {k:"medicacion", t:"Tratamiento según la derivación: acenocumarol, bisoprolol, omeprazol y paracetamol.", doc:"E14", cita:"tto: acenocumarol, bisoprolol, omeprazol, paracetamol."},
 {k:"antecedentes|hierro", t:"Ya necesitó hierro intravenoso en 2020 (carboximaltosa x2) por mala tolerancia al hierro oral, tras el sangrado de 2019.", doc:"E06", cita:"Se administran 2 dosis de HIERRO CARBOXIMALTOSA IV (16/01 y 30/01/2020)."},
 {k:"antecedentes|cirugias", t:"Cirugías previas: artroplastia total de rodilla derecha en 2021, con terapia puente de heparina por la anticoagulación.", doc:"E07", cita:"IQ 08/06/2021: ARTROPLASTIA TOTAL DE RODILLA DERECHA."},
 {k:"antecedentes|urgencias", t:"Un ingreso por hemorragia digestiva alta en 2019: Hb 8,9, INR 3,4 y transfusión de 2 concentrados.", doc:"E05", cita:"Se transfunden 2 concentrados de hematies."},
 {k:"falta|pendiente", t:"Lo que no hay: ninguna endoscopia desde 2019, ninguna colonoscopia desde 2016, y ninguna cápsula ni enteroscopia pese al antecedente de angiodisplasia.", doc:null, cita:null},
 {k:"motivo", t:"Viene por una primera consulta pedida el 14/10/2026 para estudiar una anemia ferropénica con sangre oculta positiva.", doc:"E14", cita:"derivo para valoracion y completar estudio, probable origen digestivo."},
 {k:"habitos", t:"No consta en ninguna nota registro de tabaco ni de alcohol.", doc:null, cita:null},
 {k:"familia", t:"No consta ningún antecedente familiar en la historia.", doc:null, cita:null},
 {k:"social", t:"Acude con su hijo y oye mal por teléfono: el aviso de la prueba conviene darlo por su hijo.", doc:"E12", cita:"segun su hijo, el paciente no lo tiene claro."},
 {k:"peso", t:"Peso 72 kg y talla 171 cm en julio de 2026, frente a 78 kg en noviembre de 2025.", doc:"E12", cita:"Peso 72 kg (en noviembre de 2025 pesaba 78 kg). Talla 171 cm."},
 {k:"salud_mental", t:"No consta ningún problema de salud mental en la historia.", doc:null, cita:null}
 ],

 preguntas: [
  {q:"¿Qué le ha pasado?", k:["resumen"]},
  {q:"¿Qué endoscopias tiene y de cuándo?", k:["endoscopia"]},
  {q:"¿Hay algo que contradiga la derivación?", k:["contradiccion"]},
  {q:"¿Alergias?", k:["alergias"]},
  {q:"¿Qué medicación toma?", k:["medicacion"]},
  {q:"¿Está anticoagulado?", k:["anticoagulacion"]},
  {q:"¿Tiene datos de alarma?", k:["alarma"]},
  {q:"¿Cómo está la función renal?", k:["renal"]},
  {q:"¿Qué falta por hacer?", k:["falta","pendiente"]},
  {q:"¿Por qué viene hoy?", k:["motivo"]}
 ]
};

// Hallazgos clave: la lista con la que se compara al médico contra Jano.
// `peso` es lo que cuesta si se pasa por alto: alto, medio.
window.HALLAZGOS = {
 P1: [
  {id:"h1", t:"Alergia a metamizol (la derivación dice NAMC)", doc:"D06", peso:"alto"},
  {id:"h2", t:"Histerectomía en 2021 (la derivación dice «no IQ previas»)", doc:"D06", peso:"medio"},
  {id:"h3", t:"Toma ibuprofeno 3-4 días al mes (la derivación dice «no AINEs»)", doc:"D11", peso:"alto"},
  {id:"h4", t:"Adenoma de colon en 2015 con control a 5 años nunca hecho", doc:"D01", peso:"alto"},
  {id:"h5", t:"Sigue tomando el omeprazol desde julio: la «falta de respuesta» no es valorable", doc:"D17", peso:"alto"},
  {id:"h6", t:"La única ecografía abdominal es de 2020, hecha en Urgencias", doc:"D05", peso:"medio"},
  {id:"h7", t:"Nunca le han hecho una gastroscopia", doc:"D01", peso:"medio"},
  {id:"h8", t:"Celiaquía ya descartada en septiembre de 2026", doc:"D15", peso:"medio"},
  {id:"h9", t:"Episodio previo de dispepsia por AINE en Urgencias (2020)", doc:"D05", peso:"medio"},
  {id:"h10", t:"Cuidaba a su madre con demencia, fallecida en julio de 2026", doc:"D11", peso:"medio"}
 ],
 P2: [
  {id:"g1", t:"Angiodisplasia gástrica con sangrado en 2019 tratada con argón (la derivación dice «no antecedentes digestivos»)", doc:"E05", peso:"alto"},
  {id:"g2", t:"Anticoagulado con acenocumarol por fibrilación auricular", doc:"E04", peso:"alto"},
  {id:"g3", t:"Posible doble antitrombótico: Adiro 100 sin indicación, «pendiente de confirmar» que lo retiró", doc:"E09", peso:"alto"},
  {id:"g4", t:"Alergia a contrastes yodados", doc:"E03", peso:"alto"},
  {id:"g5", t:"Pérdida de 6 kg en 8 meses, solo en la nota de enfermería", doc:"E12", peso:"alto"},
  {id:"g6", t:"Deposiciones oscuras referidas por el hijo, solo en la nota de enfermería", doc:"E12", peso:"alto"},
  {id:"g7", t:"Enfermedad renal crónica 3a (FG 44): ajustar fármacos y preparación", doc:"E08", peso:"medio"},
  {id:"g8", t:"Colonoscopia de control de 2026 pendiente desde 2016", doc:"E02", peso:"medio"},
  {id:"g9", t:"Ya necesitó hierro intravenoso en 2020 por intolerancia al oral", doc:"E06", peso:"medio"},
  {id:"g10", t:"Hernia de hiato con esofagitis grado A (2012)", doc:"E01", peso:"medio"}
 ]
};
