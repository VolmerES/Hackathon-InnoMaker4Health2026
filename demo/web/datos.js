// Datos sintéticos de la demo de Jano. Generado desde demo/derivaciones/.
window.CASOS = [
 {
  "id": "DER-2026-04871",
  "paciente": {
   "nombre": "María Alonso Vega",
   "nhc": "PAC-00417",
   "edad": 58,
   "sexo": "Mujer",
   "contacto": "SMS",
   "acc": "Horario de tarde (cuidadora a cargo)"
  },
  "centro": "C.S. Ensanche de Vallecas",
  "medico": "Dra. C. Herrero Nieto",
  "fecha": "12/10/2026 09:14",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Dispepsia refractaria a IBP",
   "evolucion": "5 meses",
   "antecedentes": [
    "HTA en tratamiento",
    "No AINE habitual",
    "No fumadora"
   ],
   "pruebas_ap": [
    "Hemograma y bioquímica con perfil hepático · 12/09/2026",
    "Antígeno H. pylori en heces · negativo · 18/09/2026",
    "Sangre oculta en heces · negativa · 18/09/2026"
   ],
   "banderas": [],
   "faltan": [],
   "confianza": 0.94
  },
  "evidencia": {
   "evolucion": "dolor epigastrico de 5 meses",
   "motivo": "dispepsia no investigada que no responde a IBP",
   "antecedentes": "AP: HTA (enalapril). no IQ previas. no AINEs de forma habitual. no fumadora. NAMC.",
   "pruebas_ap": "analitica 12/09: hemograma normal Hb 13.4"
  },
  "propuesta": {
   "ruta": "prueba",
   "etiqueta": "Prueba antes de la primera consulta",
   "pruebas": [
    "Ecografía abdominal"
   ],
   "evita": [
    "Analítica con perfil hepático (hecha en Primaria el 12/09)",
    "Test de H. pylori (hecho y negativo)"
   ],
   "protocolo": "Digestivo · Dispepsia no investigada ≥55 años (v3, aprobado 09/2026)",
   "nota": "Primera consulta citada cuando el resultado de la ecografía esté disponible. Hueco de tarde."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04871\nid_paciente .......... PAC-00417        edad 58      sexo Mujer\nfecha_solicitud ...... 12/10/2026 09:14\norigen ............... C.S. Ensanche de Vallecas · Dra. C. Herrero Nieto\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: dispepsia. estudio\n\nmujer 58a, dolor epigastrico de 5 meses, posprandial, al principio dia si dia no y ahora\ncasi diario. plenitud precoz. no disfagia no vomitos. peso estable (68 kg). ritmo intestinal\nnormal, no rectorragias.\nAP: HTA (enalapril). no IQ previas. no AINEs de forma habitual. no fumadora. NAMC.\nha hecho 2 tandas de omeprazol 20mg/24h 8 sem cada una, mejora algo y al dejarlo vuelve.\nEF: abd blando depresible, molestias a la palpacion en epigastrio, no masas ni megalias,\nmurphy -\nanalitica 12/09: hemograma normal Hb 13.4 / bioq con perfil hepatico normal GPT 24 GGT 31\nAg H.pylori heces NEGATIVO 18/09. SOH negativa 18/09 (todo en HCE)\ndispepsia no investigada que no responde a IBP, ruego valoracion y estudio. gracias\n"
 },
 {
  "id": "DER-2026-04872",
  "paciente": {
   "nombre": "Antonio Ruiz Palomar",
   "nhc": "PAC-01928",
   "edad": 71,
   "sexo": "Hombre",
   "contacto": "Llamada telefónica",
   "acc": "Movilidad reducida, acompañante. Hipoacusia"
  },
  "centro": "C.S. Alcalá de Guadaíra",
  "medico": "Dr. J. Mendoza Barral",
  "fecha": "12/10/2026 10:02",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Reanudar seguimiento",
   "motivo": "Colitis ulcerosa en remisión · cita de revisión perdida",
   "evolucion": "Diagnóstico 2019 · última consulta 14/03/2026",
   "antecedentes": [
    "Colitis ulcerosa izquierda (2019)",
    "Mesalazina 3 g/24 h",
    "Sin brotes en el último año"
   ],
   "pruebas_ap": [
    "Hemograma, bioquímica y PCR · 02/10/2026",
    "Calprotectina fecal 48 µg/g · 02/10/2026"
   ],
   "banderas": [],
   "faltan": [],
   "confianza": 0.97
  },
  "evidencia": {
   "tipo": "perdio cita revision digestivo, solicita nueva",
   "evolucion": "DX 2019 EN SEGUIMIENTO POR DIGESTIVO",
   "motivo": "NO HAY CAMBIOS CLINICOS NI NADA NUEVO QUE VALORAR",
   "pruebas_ap": "CALPROTECTINA 48."
  },
  "propuesta": {
   "ruta": "revision",
   "etiqueta": "Revisión directa, sin primera consulta",
   "pruebas": [],
   "evita": [
    "Primera consulta de valoración (el Servicio ya le sigue)"
   ],
   "protocolo": "Digestivo · Reanudación de seguimiento (v3)",
   "nota": "Agenda de revisiones de la Dra. Sanz. Aviso por llamada telefónica, no por SMS."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04872\nid_paciente .......... PAC-01928        edad 71      sexo Hombre\nfecha_solicitud ...... 12/10/2026 10:02\norigen ............... C.S. Alcalá de Guadaíra · Dr. J. Mendoza Barral\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: perdio cita revision digestivo, solicita nueva\n\nPACIENTE DE 71 A CON COLITIS ULCEROSA IZQUIERDA DX 2019 EN SEGUIMIENTO POR DIGESTIVO\n(ULTIMA CONSULTA 14/03/26, DRA SANZ). TTO MESALAZINA 3G/24H. SIN BROTES EN EL ULTIMO AÑO,\n1-2 DEPS/DIA SIN SANGRE Y SIN DOLOR.\nTENIA CITA EL 22/09 Y NO PUDO IR PORQUE INGRESARON A SU MUJER. LLAMO AL HOSPITAL Y LE\nDICEN QUE TIENE QUE PEDIRLA OTRA VEZ A TRAVES DE SU MEDICO DE FAMILIA, ASI QUE LA PIDO YO.\nNO HAY CAMBIOS CLINICOS NI NADA NUEVO QUE VALORAR, SOLO RECUPERAR SU SEGUIMIENTO.\nANALITICA 02/10 NORMAL, PCR 2.1. CALPROTECTINA 48.\nVIENE CON ACOMPAÑANTE, ANDA MAL Y OYE POCO, MEJOR AVISARLE POR TELEFONO A LA HIJA.\n"
 },
 {
  "id": "DER-2026-04873",
  "paciente": {
   "nombre": "Lucía Fernández Mota",
   "nhc": "PAC-00862",
   "edad": 34,
   "sexo": "Mujer",
   "contacto": "App / correo",
   "acc": "—"
  },
  "centro": "C.S. Daroca",
  "medico": "Dra. P. Oliván Cerdá",
  "fecha": "12/10/2026 11:37",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Diarrea a estudio",
   "evolucion": "No consta",
   "antecedentes": [],
   "pruebas_ap": [],
   "banderas": [],
   "faltan": [
    "Tiempo de evolución",
    "Presencia de sangre o productos patológicos",
    "Analítica básica con PCR",
    "Peso y si hay pérdida"
   ],
   "confianza": 0.88
  },
  "evidencia": {
   "motivo": "MOTIVO: diarrea",
   "evolucion": "acude por diarrea",
   "faltan": "molestias abdominales. derivo a digestivo para estudio."
  },
  "propuesta": {
   "ruta": "devolver",
   "etiqueta": "Devolver a Primaria con una pregunta concreta",
   "pruebas": [],
   "evita": [
    "Primera consulta que acabaría pidiendo la analítica básica"
   ],
   "protocolo": "Digestivo · Mínimos de la derivación por diarrea (v3)",
   "nota": "Mensaje al centro: «Para citar esta derivación necesitamos tiempo de evolución, si hay sangre en las heces, peso actual y una analítica con hemograma, bioquímica, PCR y serología de celiaquía.»"
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04873\nid_paciente .......... PAC-00862        edad 34      sexo Mujer\nfecha_solicitud ...... 12/10/2026 11:37\norigen ............... C.S. Daroca · Dra. P. Oliván Cerdá\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: diarrea\n\nacude por diarrea. molestias abdominales. derivo a digestivo para estudio.\n"
 },
 {
  "id": "DER-2026-04874",
  "paciente": {
   "nombre": "Rosa Iglesias Durán",
   "nhc": "PAC-02240",
   "edad": 62,
   "sexo": "Mujer",
   "contacto": "SMS",
   "acc": "—"
  },
  "centro": "C.S. Justicia",
  "medico": "Dr. A. Villalba Torres",
  "fecha": "12/10/2026 12:05",
  "prioridad_ap": "Preferente",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Rectorragia con anemia ferropénica",
   "evolucion": "3 semanas",
   "antecedentes": [
    "Padre con cáncer de colon a los 68 años"
   ],
   "pruebas_ap": [
    "Hemograma: Hb 10,8 g/dL, VCM 76 fL, ferritina 9 ng/mL · 09/10/2026",
    "Bioquímica · 09/10/2026"
   ],
   "banderas": [
    "Rectorragia",
    "Anemia ferropénica",
    "Pérdida de 3 kg",
    "Antecedente familiar de primer grado"
   ],
   "faltan": [],
   "confianza": 0.96
  },
  "evidencia": {
   "evolucion": "desde hace 3 semanas",
   "banderas": "ha perdido\n3 kg sin querer en 2 meses",
   "antecedentes": "AF: padre ca colon a los 68.",
   "pruebas_ap": "Hb 10.8 VCM 76 ferritina 9",
   "motivo": "Rectorragia + anemia ferropenica + AF de 1er grado"
  },
  "propuesta": {
   "ruta": "prueba",
   "etiqueta": "Prueba preferente antes de la primera consulta",
   "pruebas": [
    "Colonoscopia completa con sedación (preferente)"
   ],
   "evita": [
    "Analítica de anemia (hecha en Primaria el 09/10)"
   ],
   "protocolo": "Digestivo · Rectorragia con datos de alarma (v3)",
   "nota": "Criterio de sospecha de neoplasia: la petición queda preparada y el circuito marca el caso para preferente."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04874\nid_paciente .......... PAC-02240        edad 62      sexo Mujer\nfecha_solicitud ...... 12/10/2026 12:05\norigen ............... C.S. Justicia · Dr. A. Villalba Torres\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Preferente\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: RECTORRAGIA DE 3 SEMANAS - PREFERENTE\n\nmujer de 62 a. sangrado rojo oscuro mezclado con las heces en casi todas las deposiciones\ndesde hace 3 semanas. ha cambiado el ritmo, antes 1 dep/dia y ahora 3-4 blandas. ha perdido\n3 kg sin querer en 2 meses. no fiebre. no dolor anal ni esfuerzo defecatorio.\ntacto rectal: no hemorroides externas no fisuras, dedo limpio.\nAF: padre ca colon a los 68.\nanalitica 09/10: Hb 10.8 VCM 76 ferritina 9 --> ferropenia. resto de bioq normal.\nRectorragia + anemia ferropenica + AF de 1er grado. derivo PREFERENTE para descartar neo\ncolorrectal.\n"
 },
 {
  "id": "DER-2026-04875",
  "paciente": {
   "nombre": "Iván Beltrán Quesada",
   "nhc": "PAC-00733",
   "edad": 29,
   "sexo": "Hombre",
   "contacto": "App",
   "acc": "Turnos: pide cita de mañana"
  },
  "centro": "C.S. Jaime Vera",
  "medico": "Dra. M. Lobato Esteve",
  "fecha": "12/10/2026 13:21",
  "prioridad_ap": "Preferente",
  "ficha": {
   "tipo": "Revisión por empeoramiento",
   "motivo": "Probable brote de enfermedad de Crohn ileal",
   "evolucion": "3 semanas de empeoramiento",
   "antecedentes": [
    "Crohn ileal (2022)",
    "Azatioprina 150 mg/24 h",
    "Seguimiento Dr. Cuevas"
   ],
   "pruebas_ap": [
    "PCR 38 mg/L, Hb 12,1 g/dL · 10/10/2026",
    "Calprotectina fecal 740 µg/g · 10/10/2026"
   ],
   "banderas": [
    "Calprotectina 740 µg/g",
    "PCR elevada"
   ],
   "faltan": [],
   "confianza": 0.93
  },
  "evidencia": {
   "tipo": "crohn conocido, empeoramiento",
   "evolucion": "desde hace 3 semanas",
   "banderas": "calprotectina fecal 740",
   "pruebas_ap": "PCR 38",
   "motivo": "impresion: brote"
  },
  "propuesta": {
   "ruta": "revision",
   "etiqueta": "Revisión preferente con su especialista",
   "pruebas": [],
   "evita": [
    "Primera consulta de valoración",
    "Repetir analítica y calprotectina"
   ],
   "protocolo": "Digestivo · EII en brote (v3)",
   "nota": "Agenda del Dr. Cuevas, primer hueco preferente de mañana."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04875\nid_paciente .......... PAC-00733        edad 29      sexo Hombre\nfecha_solicitud ...... 12/10/2026 13:21\norigen ............... C.S. Jaime Vera · Dra. M. Lobato Esteve\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Preferente\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: crohn conocido, empeoramiento. adelantar revision\n\nvaron 29a, EC ileal dx 2022, seguimiento en digestivo de la princesa (dr cuevas, ultima rev\n05/05/26), con azatioprina 150mg/24h.\ndesde hace 3 semanas 5-6 deps/dia, dolor en FID, astenia. no fiebre, no sangre. no ha\ntomado corticoides.\nanalitica 10/10: PCR 38, Hb 12.1. calprotectina fecal 740.\nimpresion: brote. no necesita una primera consulta, necesita que le vean antes de lo que\nle toca. trabaja a turnos, si puede ser por la mañana.\n"
 },
 {
  "id": "DER-2026-04876",
  "paciente": {
   "nombre": "Pilar Sanchís Robledo",
   "nhc": "PAC-01104",
   "edad": 46,
   "sexo": "Mujer",
   "contacto": "SMS",
   "acc": "Intérprete de lengua de signos"
  },
  "centro": "C.S. Pacífico",
  "medico": "Dr. R. Caamaño Prieto",
  "fecha": "12/10/2026 14:48",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Sospecha de enfermedad hepática esteatósica metabólica",
   "evolucion": "Transaminasas altas en 2 controles separados 3 meses",
   "antecedentes": [
    "Obesidad grado I (IMC 32)",
    "Hipertrigliceridemia",
    "AUDIT-C 0"
   ],
   "pruebas_ap": [
    "Perfil hepático 05/08 y 02/10/2026",
    "Serologías VHB y VHC negativas · 05/08/2026",
    "Ecografía abdominal · 28/09/2026: esteatosis"
   ],
   "banderas": [],
   "faltan": [],
   "confianza": 0.91
  },
  "evidencia": {
   "evolucion": "GPT 78 (05/08) y 71 (02/10)",
   "antecedentes": "obesidad grado I (IMC 32), hipertrigliceridemia",
   "pruebas_ap": "ECO ABD 28/09 (hecha en el centro de especialidades)",
   "motivo": "sospecho EHmet"
  },
  "propuesta": {
   "ruta": "prueba",
   "etiqueta": "Prueba antes de la primera consulta",
   "pruebas": [
    "Elastografía hepática (FibroScan)"
   ],
   "evita": [
    "Ecografía abdominal (hecha el 28/09 en el centro de especialidades)",
    "Serologías VHB y VHC (hechas el 05/08)"
   ],
   "protocolo": "Digestivo · Esteatosis hepática metabólica (v3)",
   "nota": "FIB-4 calculable con la analítica del 02/10: 0,89 (bajo riesgo). La elastografía confirma el estadio en la misma visita. Reservar intérprete de lengua de signos."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04876\nid_paciente .......... PAC-01104        edad 46      sexo Mujer\nfecha_solicitud ...... 12/10/2026 14:48\norigen ............... C.S. Pacífico · Dr. R. Caamaño Prieto\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: hipertransaminasemia + esteatosis en eco\n\n46a, obesidad grado I (IMC 32), hipertrigliceridemia. asintomatica.\nGPT 78 (05/08) y 71 (02/10), GOT 54, GGT 96, plaquetas 243000, albumina 4.3.\nserologias VHB y VHC negativas (05/08). no alcohol (AUDIT-C 0). no farmacos hepatotoxicos.\nECO ABD 28/09 (hecha en el centro de especialidades): higado de tamaño normal con aumento\ndifuso de la ecogenicidad compatible con esteatosis. via biliar normal, bazo normal.\nsospecho EHmet, derivo para valoracion y para estadificar fibrosis.\nOJO: la paciente es sorda, necesita interprete de lengua de signos en la cita.\n"
 },
 {
  "id": "DER-2026-04877",
  "paciente": {
   "nombre": "Jesús Carvajal Mena",
   "nhc": "PAC-02771",
   "edad": 69,
   "sexo": "Hombre",
   "contacto": "Llamada telefónica",
   "acc": "Vive solo. Sin acceso a internet"
  },
  "centro": "C.S. Montesa",
  "medico": "Dra. E. Nogueira Sáez",
  "fecha": "12/10/2026 15:30",
  "prioridad_ap": "Preferente",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Disfagia progresiva con síndrome constitucional",
   "evolucion": "2 meses",
   "antecedentes": [
    "Fumador de 20 cig./día",
    "Consumo diario de alcohol"
   ],
   "pruebas_ap": [
    "Hemograma: Hb 11,2 g/dL · 11/10/2026",
    "Bioquímica: albúmina 3,4 g/dL · 11/10/2026"
   ],
   "banderas": [
    "Disfagia progresiva de sólidos a triturados",
    "Pérdida de 6 kg",
    "Alta sospecha de neoplasia"
   ],
   "faltan": [],
   "confianza": 0.95
  },
  "evidencia": {
   "evolucion": "Desde agosto disfagia",
   "banderas": "primero con solidos y en las ultimas 2 semanas tb con triturados",
   "pruebas_ap": "albumina 3.4",
   "motivo": "ALTA SOSPECHA DE NEOPLASIA ESOFAGICA"
  },
  "propuesta": {
   "ruta": "persona",
   "etiqueta": "A una persona: sospecha de neoplasia",
   "pruebas": [
    "Gastroscopia (propuesta, pendiente de decisión del facultativo)"
   ],
   "evita": [],
   "protocolo": "Digestivo · Banderas rojas → revisión por facultativo de guardia de consulta",
   "nota": "Las banderas rojas no se encaminan solas. Jano lo pone primero en la lista y avisa al Servicio el mismo día."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04877\nid_paciente .......... PAC-02771        edad 69      sexo Hombre\nfecha_solicitud ...... 12/10/2026 15:30\norigen ............... C.S. Montesa · Dra. E. Nogueira Sáez\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Preferente\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: DISFAGIA PROGRESIVA + PERDIDA DE PESO - PREFERENTE\n\nVaron 69a, fumador de 20 cig/dia desde los 18, bebedor de 2-3 cervezas al dia.\nDesde agosto disfagia, primero con solidos y en las ultimas 2 semanas tb con triturados.\nSensacion de atascamiento retroesternal. Ha perdido 6 kg (de 78 a 72) sin hacer dieta.\nAstenia importante.\nEF: delgadez llamativa, no adenopatias cervicales palpables, abdomen sin masas.\nAnalitica 11/10: Hb 11.2 VCM 82, albumina 3.4, resto normal.\nDisfagia organica progresiva con sindrome constitucional y factores de riesgo.\nALTA SOSPECHA DE NEOPLASIA ESOFAGICA. Derivo preferente y llamo yo al servicio.\nVive solo y no tiene internet, avisar por telefono.\n"
 },
 {
  "id": "DER-2026-04878",
  "paciente": {
   "nombre": "Nuria Peláez Cifuentes",
   "nhc": "PAC-00559",
   "edad": 41,
   "sexo": "Mujer",
   "contacto": "App",
   "acc": "—"
  },
  "centro": "C.S. Las Cortes",
  "medico": "Dr. F. Arjona Vidal",
  "fecha": "12/10/2026 16:12",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Primera consulta",
   "motivo": "Dispepsia persistente tras erradicación de H. pylori",
   "evolucion": "7 meses",
   "antecedentes": [
    "H. pylori erradicado (julio 2026, confirmado 05/09)",
    "8 semanas de IBP a dosis plena"
   ],
   "pruebas_ap": [
    "Antígeno H. pylori: positivo 14/07, negativo 05/09/2026",
    "Hemograma y bioquímica normales · 05/09/2026",
    "Ecografía abdominal sin hallazgos · 20/09/2026"
   ],
   "banderas": [],
   "faltan": [],
   "confianza": 0.92
  },
  "evidencia": {
   "evolucion": "epigastralgia de 7 meses",
   "antecedentes": "control a las 6 semanas (05/09) NEGATIVO, erradicado",
   "pruebas_ap": "eco abd 20/09 sin hallazgos",
   "motivo": "dispepsia persistente, H pylori ya erradicado"
  },
  "propuesta": {
   "ruta": "prueba",
   "etiqueta": "Prueba antes de la primera consulta",
   "pruebas": [
    "Gastroscopia"
   ],
   "evita": [
    "Ecografía abdominal (hecha el 20/09)",
    "Test de H. pylori (ya erradicado y confirmado)",
    "Analítica (hecha el 05/09)"
   ],
   "protocolo": "Digestivo · Dispepsia tras erradicación (v3)",
   "nota": "Tres pruebas que la consulta habría pedido ya están hechas. Solo falta la endoscopia."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04878\nid_paciente .......... PAC-00559        edad 41      sexo Mujer\nfecha_solicitud ...... 12/10/2026 16:12\norigen ............... C.S. Las Cortes · Dr. F. Arjona Vidal\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: dispepsia persistente, H pylori ya erradicado\n\n41a, epigastralgia de 7 meses. Ag Hp en heces + el 14/07. hizo cuadruple con bismuto 10\ndias en julio con buena adherencia. control a las 6 semanas (05/09) NEGATIVO, erradicado.\nsigue igual de molesta a pesar de eso y de 8 semanas de IBP a dosis plena. no perdida de\npeso, no disfagia, no anemia, no vomitos.\nanalitica 05/09 normal. eco abd 20/09 sin hallazgos.\nno se me ocurre nada mas desde aqui, ruego valoracion.\n"
 },
 {
  "id": "DER-2026-04879",
  "paciente": {
   "nombre": "Mohamed Benali Idrissi",
   "nhc": "PAC-03012",
   "edad": 52,
   "sexo": "Hombre",
   "contacto": "SMS",
   "acc": "Intérprete de árabe"
  },
  "centro": "C.S. Embajadores",
  "medico": "Dr. S. Quintanar Ruano",
  "fecha": "12/10/2026 17:03",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Sin determinar",
   "motivo": "Dolor abdominal inespecífico a estudio",
   "evolucion": "No consta («hace tiempo»)",
   "antecedentes": [],
   "pruebas_ap": [
    "Analítica previa: no se adjunta",
    "Rx abdomen en Urgencias 08/2026: informe no incluido"
   ],
   "banderas": [],
   "faltan": [
    "Tiempo de evolución",
    "Qué estudio se indicó en la interconsulta previa",
    "Resultados de la analítica que se menciona"
   ],
   "confianza": 0.41
  },
  "evidencia": {
   "evolucion": "molestias abd inespecificas ds \"hace tiempo\"",
   "faltan": "pdte completar estudio s/ indicacion previa",
   "pruebas_ap": "analitica ant (no se adjunta)",
   "motivo": "dolor abd a estudio"
  },
  "propuesta": {
   "ruta": "persona",
   "etiqueta": "A una persona: la lectura no es fiable",
   "pruebas": [],
   "evita": [],
   "protocolo": "Umbral de confianza del Servicio: 0,75",
   "nota": "Texto abreviado y con referencias a documentos que no vienen. Jano no encamina: lo pasa a la administrativa de consultas con lo poco que ha podido extraer."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04879\nid_paciente .......... PAC-03012        edad 52      sexo Hombre\nfecha_solicitud ...... 12/10/2026 17:03\norigen ............... C.S. Embajadores · Dr. S. Quintanar Ruano\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: vto estudio dig\n\npcte c/ molestias abd inespecificas ds \"hace tiempo\". ya comentado en IC no presencial (ver\nIC 09/26). pdte completar estudio s/ indicacion previa. resto s/p. no AP relevantes\nconocidos. tto actual: ver hoja de medicacion. analitica ant (no se adjunta). Rx abd en urg\n08/26, informe en HCE. dolor abd a estudio, rgo valoracion.\n"
 },
 {
  "id": "DER-2026-04880",
  "paciente": {
   "nombre": "Carmen Dorado Liébana",
   "nhc": "PAC-01577",
   "edad": 77,
   "sexo": "Mujer",
   "contacto": "Llamada telefónica",
   "acc": "Silla de ruedas. Acude con hija"
  },
  "centro": "C.S. Ibiza",
  "medico": "Dra. B. Trillo Vázquez",
  "fecha": "12/10/2026 18:22",
  "prioridad_ap": "Normal",
  "ficha": {
   "tipo": "Revisión",
   "motivo": "Estreñimiento crónico funcional mal controlado",
   "evolucion": "Seguimiento desde 2021 · última consulta 18/02/2026",
   "antecedentes": [
    "Colonoscopia 2021 sin hallazgos",
    "Macrogol con control irregular"
   ],
   "pruebas_ap": [
    "Hemograma, bioquímica y TSH normales · 01/10/2026",
    "Sangre oculta en heces negativa · 01/10/2026"
   ],
   "banderas": [],
   "faltan": [],
   "confianza": 0.95
  },
  "evidencia": {
   "tipo": "en seguimiento por digestivo desde 2021",
   "pruebas_ap": "SOH negativa 01/10",
   "motivo": "estreñimiento cronico mal controlado"
  },
  "propuesta": {
   "ruta": "revision",
   "etiqueta": "Revisión directa, sin primera consulta",
   "pruebas": [],
   "evita": [
    "Primera consulta de valoración",
    "Repetir colonoscopia (2021 normal, sin datos de alarma)"
   ],
   "protocolo": "Digestivo · Estreñimiento crónico en seguimiento (v3)",
   "nota": "Transporte sanitario pendiente de confirmar: el circuito lo marca al citar. Un solo viaje."
  },
  "texto": "=== EXPORTACION NOCTURNA HIS · TABLA DERIVACIONES_AP · DATOS SINTETICOS ===\nid_derivacion ........ DER-2026-04880\nid_paciente .......... PAC-01577        edad 77      sexo Mujer\nfecha_solicitud ...... 12/10/2026 18:22\norigen ............... C.S. Ibiza · Dra. B. Trillo Vázquez\nespecialidad_destino . APARATO DIGESTIVO\nprioridad ............ Normal\nestado_solicitud ..... Pendiente de validar\n--- motivo_derivacion y observaciones_medico (texto libre, tal cual lo escribio el medico de familia) ---\n\nMOTIVO: estreñimiento cronico mal controlado, revision digestivo\n\n77a, en seguimiento por digestivo desde 2021 por estreñimiento cronico funcional, ultima\nconsulta 18/02/26. colono 2021 sin hallazgos. toma macrogol pero de forma irregular.\nestos 2 meses hace deposicion cada 4-5 dias y con esfuerzo, sin sangre, sin dolor y sin\nperdida de peso. no vomitos. sin datos de alarma.\nanalitica 01/10 normal, TSH 2.4. SOH negativa 01/10.\npido revision con su servicio para ajustar tto. va en silla de ruedas y viene con la hija,\na ver si puede ser todo en el mismo viaje.\n"
 }
];
