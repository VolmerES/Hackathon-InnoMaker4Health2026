// Ficha de primera consulta con el formato de Medicci: lo que hay que saber
// antes de entrar, en tarjetas, con la fuente de cada dato y «No consta»
// cuando la historia no lo tiene. Datos sintéticos.

window.FICHAS = {

P1: {
  eyebrow: "Primera consulta · derivada por Atención Primaria",
  nombre: "María A. V.",
  chips: [
    {t:"58 años · Mujer"},
    {t:"Castellano"},
    {t:"Cita de tarde: cuidadora a cargo", destacado:true},
    {t:"Avisos por SMS"},
    {t:"Prioridad ordinaria"}
  ],
  antes: [
    {t:"Alergias", v:"Metamizol", d:"Exantema, prurito y edema palpebral tras la primera dosis iv. La derivación de hoy dice «NAMC».", f:"HCIS · Ginecología · 01/2021", doc:"D06", rojo:true},
    {t:"Seguridad", v:"No consta", d:"Sin escalas de caídas ni de úlceras. Sin multirresistentes registrados.", f:"—"},
    {t:"Implantes", v:"No consta", d:"Ningún dispositivo ni prótesis en la historia.", f:"—"},
    {t:"Voluntades", v:"Sin registro", d:"No consta documento de voluntades anticipadas ni ONR.", f:"—"}
  ],
  motivo: {
    t:"Dispepsia de 5 meses que no responde a IBP.",
    espera:"No consta qué espera. Enfermería anota que «está preocupada».",
    f:"Derivación AP · 12/10/2026", doc:"D18"
  },
  cambios: {
    titulo:"Cambios recientes",
    filas:[
      {t:"Peso", v:"68 kg", d:"estable desde julio; 71 kg en 2016", tend:"plano", doc:"D17"},
      {t:"Hemoglobina", v:"13,4 g/dL", d:"igual que en 2022 (13,2)", tend:"plano", doc:"D15"},
      {t:"Omeprazol", v:"sin suspender", d:"pautado 8 semanas en julio, lo sigue tomando", tend:"sube", doc:"D17"}
    ],
    f:"Laboratorio y enfermería · últimos 24 meses"
  },
  problemas: {
    filas:[
      {t:"Hipertensión arterial", v:"TA 136/84", doc:"D02"},
      {t:"Dispepsia refractaria a IBP", v:"Desde 05/2026", doc:"D13"},
      {t:"Adenoma tubular de colon", v:"2015 · control vencido", doc:"D01"},
      {t:"Anemia ferropénica", v:"Resuelta en 2021", doc:"D07"}
    ],
    f:"HCIS · Atención Primaria"
  },
  medicacion: {
    filas:[
      {f:"Enalapril", dosis:"10 mg", pauta:"cada 24 h · oral", doc:"D17"},
      {f:"Omeprazol", dosis:"20 mg", pauta:"cada 24 h · oral", aviso:"Pautado 8 semanas · sigue tomándolo", doc:"D17"},
      {f:"Ibuprofeno", dosis:"600 mg", aviso:"Sin receta · lo toma ella, 3-4 días al mes", doc:"D11"},
      {f:"Paracetamol", dosis:"1 g", pauta:"a demanda", doc:"D17"}
    ],
    adherencia:"No consta índice. Enfermería anotó buena adherencia al enalapril en 2021.",
    f:"Receta electrónica · Declarado por la paciente"
  },
  constantes: [
    {t:"Tensión", v:"136/84", d:"06/10/2026 · enfermería", doc:"D17"},
    {t:"Pulso", v:"70 lpm", d:"06/10/2026 · enfermería", doc:"D17"},
    {t:"Saturación", v:"No consta", d:"sin registro"},
    {t:"Peso · IMC", v:"68 · 25,9", d:"06/10/2026 · báscula", doc:"D17"},
    {t:"Wearables", v:"No consta", d:"sin dispositivos"}
  ],
  vida: [
    {t:"Comunicación", d:"Castellano. Avisos por SMS; pide consulta de tarde."},
    {t:"Apoyo", d:"Cuidó en casa a su madre con demencia; falleció en julio de 2026.", doc:"D17"},
    {t:"Autonomía", d:"No consta Barthel ni cribado cognitivo. Trabaja y acude sola."},
    {t:"Hábitos", d:"No fumadora. Consumo de alcohol: no consta en 11 años.", doc:"D18"},
    {t:"Social", d:"Auxiliar de geriatría: moviliza pacientes. De ahí la lumbalgia y el ibuprofeno.", doc:"D04"},
    {t:"Familia", d:"No consta ninguna anamnesis familiar, con un adenoma personal de 2015.", marco:true}
  ],
  temas: ["Cirugías","Ginecología y obstetricia","Ingresos y urgencias","Salud mental",
          "Analíticas 24 meses","Imagen (TC, RMN, eco)","Cribados","Vacunas"],
  ejemplo: {
    q:"¿Se ha hecho alguna gastroscopia?",
    a:"No consta ninguna gastroscopia en sus datos.",
    donde:"Buscado en: endoscopias, informes de Digestivo, procedimientos"
  }
},

P2: {
  eyebrow: "Primera consulta · derivada por Atención Primaria",
  nombre: "Ramón E. G.",
  chips: [
    {t:"72 años · Hombre"},
    {t:"Oye mal por el teléfono fijo", destacado:true},
    {t:"Viene con su hijo"},
    {t:"Anticoagulado"},
    {t:"Prioridad ordinaria"}
  ],
  antes: [
    {t:"Alergias", v:"Contrastes yodados", d:"Urticaria y opresión faríngea durante la inyección en un TAC. Condiciona cualquier prueba con contraste.", f:"HCIS · Radiodiagnóstico · 10/2016", doc:"E03", rojo:true},
    {t:"Seguridad", v:"No consta", d:"Sin escala de caídas con 72 años, prótesis de rodilla y anticoagulación.", f:"—"},
    {t:"Implantes", v:"Prótesis de rodilla dcha.", d:"2021. Material y compatibilidad con RMN: no constan.", f:"HCIS · Traumatología · 2021", doc:"E07"},
    {t:"Voluntades", v:"Sin registro", d:"No consta documento de voluntades anticipadas ni ONR.", f:"—"}
  ],
  motivo: {
    t:"Anemia ferropénica a estudio, probable origen digestivo.",
    espera:"No consta qué espera. La derivación no menciona la pérdida de peso ni las heces oscuras.",
    f:"Derivación AP · 14/10/2026", doc:"E14"
  },
  cambios: {
    titulo:"Cambios recientes",
    filas:[
      {t:"Hemoglobina", v:"9,6 g/dL", d:"bajando desde 10,2 en marzo", tend:"baja", doc:"E13"},
      {t:"Peso", v:"−7,7 %", d:"en 8 meses · 78 → 72 kg", tend:"baja", doc:"E12"},
      {t:"Filtrado", v:"44 mL/min", d:"era 48 en 2023", tend:"baja", doc:"E13"},
      {t:"Sangre oculta", v:"Positiva", d:"28/09/2026", tend:"sube", doc:"E13"}
    ],
    f:"Laboratorio y enfermería · últimos 24 meses"
  },
  problemas: {
    filas:[
      {t:"Fibrilación auricular permanente", v:"INR 2,5", doc:"E04"},
      {t:"Enfermedad renal crónica", v:"G3a · FG 44", doc:"E08"},
      {t:"Angiodisplasia gástrica", v:"2019 · argón", doc:"E05"},
      {t:"Anemia ferropénica recidivante", v:"Ferritina 8", doc:"E13"}
    ],
    f:"HCIS · Atención Primaria y Digestivo"
  },
  medicacion: {
    filas:[
      {f:"Acenocumarol", dosis:"según pauta", pauta:"hematología · INR en rango", doc:"E04"},
      {f:"Bisoprolol", dosis:"2,5 mg", pauta:"cada 24 h · oral", doc:"E04"},
      {f:"Omeprazol", dosis:"40 mg", pauta:"cada 24 h · oral", doc:"E05"},
      {f:"Adiro", dosis:"100 mg", aviso:"Pendiente de retirar desde 2024 · preguntar hoy", doc:"E09"},
      {f:"Paracetamol", dosis:"1 g", pauta:"a demanda · AINEs contraindicados", doc:"E09"}
    ],
    adherencia:"No consta índice. Controles de INR estables en 2026.",
    f:"Receta electrónica · Atención Primaria"
  },
  constantes: [
    {t:"Tensión", v:"134/80", d:"07/07/2026 · enfermería", doc:"E12"},
    {t:"Pulso", v:"No consta", d:"sin registro reciente"},
    {t:"Saturación", v:"No consta", d:"sin registro"},
    {t:"Peso · IMC", v:"72 · 24,6", d:"07/07/2026 · báscula", doc:"E12"},
    {t:"Wearables", v:"No consta", d:"la FA se vio por Holter"}
  ],
  vida: [
    {t:"Comunicación", d:"Oye mal por el teléfono fijo: dar los avisos a través del hijo.", doc:"E12", marco:true},
    {t:"Apoyo", d:"Acude con su hijo, que aporta lo de las deposiciones oscuras.", doc:"E12"},
    {t:"Autonomía", d:"No consta Barthel ni cribado cognitivo."},
    {t:"Hábitos", d:"Tabaco y alcohol: no constan en 14 años de historia."},
    {t:"Social", d:"No consta profesión, vivienda ni situación económica."},
    {t:"Familia", d:"No consta ninguna anamnesis familiar, con anemia y sangre oculta positiva.", marco:true}
  ],
  temas: ["Endoscopias","Ingresos y urgencias","Anticoagulación","Función renal",
          "Analíticas 24 meses","Imagen (TC, RMN, eco)","Cribados","Vacunas"],
  ejemplo: {
    q:"¿Tiene antecedentes digestivos?",
    a:"Sí: angiodisplasia gástrica con sangrado tratada con argón en 2019, hernia de hiato con esofagitis A y diverticulosis.",
    donde:"Buscado en: endoscopias, informes de alta, Digestivo"
  }
}

};
