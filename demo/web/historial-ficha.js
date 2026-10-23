// Ficha de contexto en 3 capas, tal como la tiene que producir el modelo local
// a partir de la historia. Versión resumida para la pantalla; la completa está
// en demo/fichas/ficha-contexto-P1.md y -P2.md.
// Cada fila: [campo, valor, documento | null]. null = no consta en la historia.

window.FICHAS = {
 P1: {
  cobertura: "63 campos de la ficha · 42 con dato · 21 sin dato en la historia",
  capas: [
   {n:"CAPA 1", t:"Control crítico y contexto de vida", secciones:[
    {t:"1. Identificación y acceso", filas:[
     ["Edad / sexo","58 años · Mujer (nac. 04/02/1968)",null],
     ["Barreras de comunicación","No consta",null],
     ["Alfabetización en salud (REALM-R)","No consta",null],
     ["Red de soporte","Invertida: ella era la cuidadora de su madre con demencia, fallecida en julio de 2026","D11"],
     ["Sobrecarga del cuidador (Zarit)","No consta. Procedía medirlo: GAD-7 12 y 4-5 h de sueño","D11"],
     ["Canal de contacto","SMS. Pide horario de tarde",null]]},
    {t:"2. Alergias y toxicidades", filas:[
     ["Alergia a medicamento","METAMIZOL (N02BB02): exantema, prurito y edema palpebral tras dosis iv, cede en 4 h","D06"],
     ["Mecanismo / severidad","Inmediata (tipo I), moderada. Sin prueba cutánea","D06"],
     ["Confirmación","Evento clínico 27/01/2021, reconfirmada en A.P. 12/03/2021","D07"],
     ["Toxicidad no inmune","Dispepsia atribuida a AINE en Urgencias (2020)","D05"],
     ["⚠ Discrepancia","La derivación de hoy declara «NAMC»","D18"]]},
    {t:"3. Alertas e implantes", filas:[
     ["Multirresistentes / aislamiento","No consta",null],
     ["Escalas de riesgo (caídas, Braden, Columbia)","No constan",null],
     ["Implantes y dispositivos","Ninguno registrado",null]]},
    {t:"4. Voluntades anticipadas", filas:[
     ["Voluntades / DNR / LET","No consta",null]]},
    {t:"5. Hábitos e inmunizaciones", filas:[
     ["Tabaco","No fumadora","D18"],
     ["Alcohol (UBE / AUDIT)","No consta ningún registro en 11 años",null],
     ["Sueño","Insomnio de conciliación en 2024 (4-5 h), mejor en octubre de 2026","D17"],
     ["Vacunas","Solo gripe el 05/10/2021. Sin lotes ni resto de calendario","D08"]]},
    {t:"6. Determinantes sociales", filas:[
     ["Profesión","Auxiliar de geriatría, moviliza pacientes: origen de la lumbalgia","D04"],
     ["Vivienda, entorno, copagos","No constan",null],
     ["Carga de cuidados (Z63.6)","Cuidadora de madre dependiente 2022-2026","D11"]]},
    {t:"7. Funcional, cognitivo y psicológico", filas:[
     ["Barthel / Lawton","No constan. Autónoma por lo descrito",null],
     ["Cribado cognitivo","No consta",null],
     ["Esfera psicológica","GAD-7 12 (2024), lorazepam en pauta corta; sin tomarlo hace más de un año","D11"],
     ["Estresor activo","Duelo: su madre falleció en julio de 2026","D17"]]},
    {t:"8. Carga hereditaria", filas:[
     ["Primer y segundo grado","Solo consta la demencia de la madre. Sin edades ni causas",null],
     ["Síndromes de cáncer familiar","No consta ninguna anamnesis familiar, con un adenoma personal de 2015",null]]}
   ]},
   {n:"CAPA 2", t:"Contexto clínico dinámico", secciones:[
    {t:"2.1 Motivo y cronología", filas:[
     ["Emisor / prioridad","Dra. Herrero Nieto · C.S. Ensanche de Vallecas · Normal","D18"],
     ["Motivo","Dispepsia de 5 meses que no responde a IBP","D18"],
     ["Cronología","Mayo 2026 inicio → julio recae al dejar el IBP → octubre sigue igual","D13"],
     ["Expectativa del paciente","No consta como tal; enfermería anota que está preocupada","D17"]]},
    {t:"2.2 Problemas activos", filas:[
     ["HTA esencial grado 1","Desde 2016, enalapril. TA 136/84 el 06/10/2026","D02"],
     ["Dispepsia refractaria a IBP","Desde mayo de 2026","D13"],
     ["Adenoma tubular de colon","Resecado en 2015, displasia de bajo grado","D01"],
     ["Anemia ferropénica","Resuelta tras la cirugía de 2021 (Hb 10,1 → 12,4)","D07"]]},
    {t:"2.3 Conciliación farmacológica", filas:[
     ["Vigente","Enalapril 10 mg/24 h · omeprazol 20 mg/24 h · paracetamol a demanda · lágrimas","D17"],
     ["⚠ Alerta de conciliación","El omeprazol era por 8 semanas desde julio y lo sigue tomando el 06/10: la «falta de respuesta» no es valorable","D17"],
     ["OTC / automedicación","Ibuprofeno 600, 3-4 días al mes por lumbalgia (2018, 2024, 2026)","D11"],
     ["Adherencia (MPR)","Sin índice. Enfermería anota buena adherencia al enalapril (2021)","D08"],
     ["⚠ Discrepancias","La derivación dice «no IQ previas» y «no AINEs de forma habitual»","D18"]]},
    {t:"2.4 Constantes y biometría", filas:[
     ["TA / FC","136/84 · 70 lpm (06/10/2026)","D17"],
     ["AMPA domiciliaria","148/92 de media en 2016 al diagnóstico. Sin registro actual","D02"],
     ["Wearables (HRV, sueño, FA, IAH)","Ningún dispositivo",null],
     ["Peso / talla / IMC","68 kg · 162 cm · 25,9","D17"],
     ["Variación ponderal","71 kg (2016) → 68 kg (2026). Estable en los últimos meses, sin pérdida reciente","D14"]]}
   ]},
   {n:"CAPA 3", t:"Historial profundo e intervenciones", secciones:[
    {t:"3.1 Quirúrgicos y obstétricos", filas:[
     ["Cirugía","Histerectomía total laparoscópica con doble anexectomía (27/01/2021), sin complicaciones","D06"],
     ["Obstétricos (G-P-A-C-E)","No constan",null],
     ["Menopausia","Quirúrgica desde enero de 2021, a los 52 años","D06"],
     ["Citologías (Bethesda)","No constan",null],
     ["Ingresos 5 años","Uno: Ginecología, 2 días en enero de 2021","D06"],
     ["Urgencias","Tres: cólico renal 2017, epigastralgia por AINE 2020, dolor torácico atípico 2025","D12"]]}
    ]}
  ],
  claves:[
   "La derivación declara NAMC, «no IQ previas» y «no AINEs»: las tres son falsas según la historia.",
   "El omeprazol no se ha suspendido: la falta de respuesta a IBP no es valorable.",
   "Control de colonoscopia vencido desde 2020 con adenoma previo.",
   "Nunca ha tenido gastroscopia; la única ecografía abdominal es de 2020.",
   "Celiaquía descartada y H. pylori negativo: no repetir.",
   "Vacíos de 11 años: anamnesis familiar y consumo de alcohol."
  ]
 },

 P2: {
  cobertura: "58 campos de la ficha · 40 con dato · 18 sin dato en la historia",
  capas: [
   {n:"CAPA 1", t:"Control crítico y contexto de vida", secciones:[
    {t:"1. Identificación y acceso", filas:[
     ["Edad / sexo","72 años · Hombre (nac. 11/09/1954)",null],
     ["Barreras de comunicación","Oye mal por teléfono fijo: avisar a través del hijo","E12"],
     ["Red de soporte","Acude con su hijo, que aporta lo de las deposiciones oscuras","E12"],
     ["Alfabetización en salud, idioma","No constan",null]]},
    {t:"2. Alergias y toxicidades", filas:[
     ["Alergia a contrastes","CONTRASTES YODADOS: urticaria y opresión faríngea durante la inyección en TC","E03"],
     ["Mecanismo / severidad","Inmediata (tipo I), moderada. Sin estudio alergológico","E03"],
     ["Alergias a medicamentos","Ninguna registrada",null],
     ["Toxicidad no inmune","Intolerancia al hierro oral: estreñimiento y náuseas","E06"],
     ["⚠ Repercusión hoy","Ninguna TC con contraste sin premedicar o buscar alternativa","E03"]]},
    {t:"3. Alertas e implantes", filas:[
     ["Implante ortopédico","Prótesis total de rodilla derecha (2021). Material y compatibilidad RMN: no constan","E07"],
     ["Implantes cardiovasculares","Ninguno; FA permanente sin dispositivo","E04"],
     ["Escalas de riesgo (caídas, Braden)","No constan, con 72 años, prótesis y anticoagulación",null]]},
    {t:"4. Voluntades anticipadas", filas:[
     ["Voluntades / DNR / LET","No consta",null]]},
    {t:"5. Hábitos e inmunizaciones", filas:[
     ["Tabaco","No consta registro en 14 años",null],
     ["Alcohol (UBE / AUDIT)","No consta registro. Vacío relevante en una anemia a estudio",null],
     ["Dieta / actividad","Come menos, «se llena enseguida», y más cansado al andar (julio 2026)","E12"],
     ["Vacunas","Ninguna registrada",null]]},
    {t:"6. Determinantes sociales", filas:[
     ["Profesión y exposiciones","No constan",null],
     ["Vivienda, barreras, copagos","No constan",null],
     ["Apoyo familiar","Hijo implicado en el seguimiento","E12"]]},
    {t:"7. Funcional, cognitivo y psicológico", filas:[
     ["Barthel / Lawton / cribado cognitivo","No constan",null],
     ["Esfera psicológica","No consta ningún problema",null],
     ["Dato funcional indirecto","Astenia progresiva y menor tolerancia al esfuerzo desde julio de 2026","E12"]]},
    {t:"8. Carga hereditaria", filas:[
     ["Primer y segundo grado","No consta ninguna anamnesis familiar",null],
     ["Cáncer colorrectal familiar","No consta, con anemia, SOH positiva y diverticulosis",null]]}
   ]},
   {n:"CAPA 2", t:"Contexto clínico dinámico", secciones:[
    {t:"2.1 Motivo y cronología", filas:[
     ["Emisor / prioridad","Dr. Molina Esparza · C.S. Ángela Uriarte · Normal","E14"],
     ["⚠ Prioridad discutible","Hb 9,6 descendente con SOH positiva y pérdida de peso","E13"],
     ["Cronología","Hb 10,2 (03/2026) → −6 kg y heces oscuras (07/2026) → Hb 9,6, SOH + (09/2026) → derivación","E11"],
     ["Expectativa del paciente","No consta",null]]},
    {t:"2.2 Problemas activos", filas:[
     ["Fibrilación auricular permanente","Desde 2018. FEVI 58 %, CHA₂DS₂-VASc 3, HAS-BLED 2","E04"],
     ["Enfermedad renal crónica G3a","FG 48 (2023) → 46 (03/2026) → 44 (09/2026), A2","E08"],
     ["Anemia ferropénica recidivante","2019 postsangrado, resuelta en 2020; recidiva en 2026","E06"],
     ["Angiodisplasia gástrica","Cuerpo-antro, sangrado activo en 2019 tratado con argón","E05"],
     ["ERGE con esofagitis A y hernia de hiato","2012","E01"],
     ["Diverticulosis sigmoidea","2016, confirmada en TC","E02"]]},
    {t:"2.3 Conciliación farmacológica", filas:[
     ["Anticoagulación","Acenocumarol desde 2018. INR 2,6 / 2,3 / 2,5 en 2026: en rango","E04"],
     ["⚠ Doble antitrombótico","Adiro 100 sin indicación junto al acenocumarol (2024); se indicó retirarlo y quedó «pendiente de confirmar»","E09"],
     ["Resto","Bisoprolol 2,5 mg · omeprazol · paracetamol a demanda","E14"],
     ["AINEs","Contraindicados por la función renal","E08"],
     ["⚠ Discrepancia","La derivación dice «no antecedentes digestivos de interés»","E14"]]},
    {t:"2.4 Constantes y biometría", filas:[
     ["TA / FC","134/80 (07/07/2026)","E12"],
     ["Wearables","Ninguno. La FA se detectó por ECG y Holter, no por reloj","E04"],
     ["Peso / talla / IMC","72 kg · 171 cm · 24,6","E12"],
     ["⚠ Variación ponderal","78 kg (11/2025) → 72 kg (07/2026): −6 kg, −7,7 % en 8 meses. Solo en enfermería","E12"]]}
   ]},
   {n:"CAPA 3", t:"Historial profundo e intervenciones", secciones:[
    {t:"3.1 Quirúrgicos y cuidados críticos", filas:[
     ["Cirugía","Artroplastia total de rodilla derecha (2021), intradural, con terapia puente de HBPM","E07"],
     ["Endoscopia terapéutica","Argón plasma sobre angiodisplasia gástrica (2019); polipectomía (2016)","E05"],
     ["Anatomía patológica","Pólipo 2016 hiperplásico; biopsias antrales 2012 con H. pylori negativo","E02"],
     ["Ingresos y transfusiones","HDA en 11/2019 con 2 concentrados; Traumatología en 2021","E05"],
     ["UCI / ventilación","Ninguna",null]]}
    ]}
  ],
  claves:[
   "La derivación dice «no antecedentes digestivos»: hay angiodisplasia gástrica sangrante tratada con argón en 2019.",
   "Está anticoagulado con acenocumarol: condiciona la endoscopia y el plan periprocedimiento.",
   "Posible doble antitrombótico: el Adiro de 2024 quedó pendiente de confirmar.",
   "Alergia a contrastes yodados.",
   "−6 kg en 8 meses y deposiciones oscuras, solo en la nota de enfermería.",
   "Colonoscopia de control vencida en 2026 y nunca una cápsula pese a la angiodisplasia.",
   "ERC G3a: ajustar preparación; el hierro oral ya fracasó y hubo hierro iv en 2020."
  ]
 }
};
