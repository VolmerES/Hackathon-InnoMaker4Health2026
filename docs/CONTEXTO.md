# Contexto del reto

Transcripción del material impreso que se entregó el 23‑oct‑2026 (fotos en
[`briefing/`](briefing/)) y de la información pública del evento.

## El evento

| | |
|---|---|
| Nombre | **Innomakers4Health 2026** |
| Organizan | Fundación Pfizer, Pfizer y Wayra, con el **Hospital Universitario de La Princesa** (Madrid), que ha definido el reto |
| Fechas | **23 y 24 de octubre de 2026** |
| Sede | 42 Madrid · Fundación Telefónica |
| Formato | 100 personas en equipos de 5 (unos 20 equipos), con mentores del hospital y de Pfizer |
| Premio | **3.000 €**, aceleración en InnomakersLAB (Pfizer + Wayra) con 6 meses de mentoría y **posible piloto en La Princesa** (no garantizado). Además, opción a un proceso de selección para una estancia de dos semanas en Boston (Richi LifeSciences) |

Ni el material ni la web publican los criterios de evaluación ni el formato del
pitch: **hay que preguntarlos a la organización**.

## La pregunta

> **¿Cómo podríamos liberar tiempo clínico reduciendo tareas administrativas y
> mejorando procesos para optimizar la experiencia de pacientes y profesionales
> en consultas externas?**

**Breve contexto (del documento):** las consultas externas implican mucho más
que el tiempo que el profesional dedica al paciente. A su alrededor hay
preparación, gestión de citas, consulta de información, coordinación, pruebas,
interconsultas y seguimiento. Todo eso aumenta la carga administrativa y reduce
el tiempo para la atención clínica.

## Los tres pilares del proyecto

Decididos el 23‑oct. Toda propuesta se mide contra estos tres:

### 1. Viabilidad

Se tiene que poder pilotar en La Princesa sin rehacer nada.

- **Usa datos que el hospital ya registra.** Los CSV imitan sus sistemas de
  agenda, citas y pruebas: no hacen falta sensores ni captura nueva.
- **Se monta sobre lo que existe**, como capa encima de la agenda y no como
  sustituto. Hay que preguntar a Informática y a Tecnologías y sistemas de
  información cómo se integraría.
- **Privacidad:** los datos de salud son de categoría especial en el RGPD. El
  tratamiento se queda dentro del hospital y el prototipo solo usa datos
  sintéticos.
- **Una persona decide al final.** El sistema propone y el personal
  administrativo o de enfermería confirma.
- **Coste bajo y medible.** Se paga con el tiempo que libera (paso 5 de la hoja).

### 2. Accesibilidad

Que funcione también para quien peor lo tiene para llegar a la consulta.

- **Datos:** `PACIENTES.necesidades_accesibilidad` y
  `PACIENTES.preferencias_contacto`, y `COMUNICACIONES.canal` y
  `respuesta_paciente` para ver qué canal funciona con quién.
- **Cada paciente recibe el aviso por el canal que puede usar.** Una persona
  mayor sin smartphone recibe una llamada o un SMS, no una notificación en una
  app.
- **Solo se le ofrece un hueco compatible** con sus necesidades, como movilidad,
  acompañante o intérprete.
- **Menos viajes:** juntar en el mismo día la prueba y la consulta cuando se
  pueda (acto único).
- **La interfaz del prototipo cumple WCAG** y los mensajes van en lectura fácil.

### 3. Medicina personalizada

En este reto se traduce en un **circuito personalizado**: cada paciente recorre
las consultas externas según su caso, no según un protocolo igual para todos.

- **Datos:** prioridad y motivo de la derivación (`DERIVACIONES_AP`), pruebas
  pendientes (`PRUEBAS`), historial de cambios y ausencias (`MOVIMIENTOS_CITAS`)
  y perfil y preferencias (`PACIENTES`).
- **Ejemplos:**
  - La cita de revisión se pone cuando ese paciente vaya a tener su resultado,
    no a un plazo fijo.
  - El recordatorio se ajusta a su historial: quien suele faltar recibe más
    apoyo.
  - Los huecos se reparten por necesidad clínica.
- **Límite:** en sentido estricto, la medicina personalizada es tratamiento
  según genética o biomarcadores, y eso no está en los datos. Hay que confirmar
  con los mentores qué entiende el jurado por medicina personalizada.

**Cuidado con predecir ausencias.** Un modelo que adivina quién va a faltar
puede acabar castigando a los más vulnerables, que suelen ser también los de
peor accesibilidad. La predicción solo sirve para **dar más apoyo**, como otro
recordatorio, otro canal u ofrecer transporte. Nunca para quitar la cita o
bajar la prioridad.

## Qué le interesa a cada organizador

Hay tres organizadores y cada uno quiere algo distinto del ganador. Lo que
sigue sale de su información pública (ver [Fuentes](#fuentes)). Lo que es
deducción nuestra va marcado como tal.

### Hospital Universitario de La Princesa: quien lo pilotaría

- **Ha definido el reto** y podría pilotar la propuesta ganadora en un entorno
  asistencial real.
- **Ya trabaja con IA, y de forma premiada.** En los premios Best in Class en IA
  2026 ganó dos premios y tuvo cinco finalistas.
- **El precedente más cercano es Nostradamus.** Uno de esos finalistas, en la
  categoría «Gestión Hospitalaria Inteligente», es un modelo que predice qué
  pacientes de Urgencias van a ingresar. Lo presentan como pasar *«de un modelo
  reactivo a un modelo predictivo»*. Es justo el enfoque de nuestras ideas,
  aplicado a consultas externas.
- **Usa la etiqueta «Medicina Personalizada» como categoría propia** en esos
  premios: Pandora (cáncer de páncreas) y Predicrisis (migraña con pulsera). Nos
  dice qué entienden ellos por medicina personalizada.
- **Tiene equipo de datos propio**: la Unidad de Análisis de Datos del IIS‑Princesa
  hace IA para retos clínicos *y organizativos*. Es el equipo que mantendría
  nuestra solución, y entre los mentores está su data scientist.
- **Su Unidad de Apoyo a la Innovación** capta ideas externas, las evalúa con una
  Comisión de Innovación y gestiona la propiedad intelectual y la transferencia.
  Pide que los proyectos sean *«potencialmente transferibles»*.
- **Humanización.** Tiene un chatbot para dudas quirúrgicas (Doctora Abril) que
  *«no busca sustituir al profesional sanitario»*. Su IA de diagnóstico funciona
  *«siempre bajo supervisión del médico especialista»* y tiene un comité de ética
  clínica. La IA ayuda y el profesional decide.

### Pfizer: el socio de la industria

- **Quiere estar en la transformación digital de los hospitales.** En julio de
  2026 montó con Deloitte **InnovaSalud Spain**, una red de responsables de
  innovación de 14 hospitales para adoptar IA en el diagnóstico, el tratamiento,
  la **gestión hospitalaria** y la prevención. La Princesa no está en esa lista.
- **Preocupaciones declaradas de esa red:** protección de datos, sesgos de los
  algoritmos, ética e integración *«responsable»*.
- **Sus mentores** son de acceso al mercado, innovación hospitalaria y
  **rediseño de procesos**, estrategia digital y áreas terapéuticas
  (cardiovascular, vacunas, oncología, urgencias).
- *Deducción:* a Pfizer le conviene que el médico tenga más tiempo para
  diagnosticar y seguir el tratamiento, y que los pacientes no se pierdan entre
  la prueba y la revisión. «Revisiones sin resultado» encaja con eso. El reto de
  2025 (biomarcadores) apunta a su interés por la medicina de precisión.

### Fundación Pfizer: el lado social

- **Misión:** impulsar la ciencia, la tecnología y la innovación en salud *y
  acercarlas a la sociedad*.
- **Su tercera área es la innovación social**: la que mejora las condiciones de
  vida de la población.
- **Empezó centrada en el envejecimiento saludable.** *Deducción:* una propuesta
  que funcione también para pacientes mayores y con dificultades de acceso
  encaja con su historia.

### Wayra (Telefónica): quien acelera e invierte

- **Lo que más valora es que sea muy viable** (dicho en el evento, 23‑oct).
  Quiere apoyar el proyecto ganador y **llevarlo a la práctica de verdad**, no
  quedarse en una idea bonita.
- **Acelera al ganador** seis meses en InnomakersLAB, con acceso a las unidades
  de negocio de Telefónica y posible inversión (ticket medio de hasta unos
  250.000 €).
- **En la inversión busca** tracción validada, un producto que resuelva un
  problema real y potencial de escalar. En 2024 Wayra España invirtió sobre todo
  en digitalizar sectores tradicionales, salud incluida.
- **En 2025 la categoría Startups quedó desierta.** El listón de «esto puede ser
  una empresa» es alto.
- *Deducción:* le interesa que la solución sirva a **cualquier hospital con
  consultas externas**, no solo a La Princesa. Son todos los hospitales del
  país, así que hay que contar el modelo de negocio aunque sea en una
  diapositiva. Las comunicaciones con el paciente (SMS, voz, mensajería) son
  negocio de Telefónica.

### Qué hay que contar en el pitch para cada uno

| Organizador | Lo que quiere oír | Pilar |
|---|---|---|
| La Princesa | Funciona con sus datos y sistemas, el profesional decide y su equipo de datos puede mantenerlo. Es «el Nostradamus de consultas externas» | Viabilidad |
| Pfizer | Más tiempo clínico y menos pacientes perdidos entre la prueba y la revisión. IA responsable: sin sesgos y con privacidad | Medicina personalizada |
| Fundación Pfizer | Llega también a mayores y a quien tiene más difícil acceder | Accesibilidad |
| Wayra | **Que se puede implantar de verdad y pronto**: cómo entra en el hospital, qué hace falta y cuánto cuesta. Sirve a todos los hospitales, con modelo de negocio y una cifra de impacto que se pueda vender | Viabilidad |

**Conclusión:** la viabilidad pesa más que nada. Entre una idea brillante difícil
de implantar y una sencilla que se pueda pilotar el mes que viene, gana la
segunda.

## El MVP en 24 horas

Hay **24 horas** para hacer el MVP. Como lo que más se valora es la viabilidad,
el MVP tiene que **demostrar que se puede implantar**, no solo que la idea es
buena.

### Qué construir

Una **bandeja de trabajo diaria** para el personal administrativo de consultas
externas. Cada mañana enseña qué va a fallar en los próximos días y propone qué
hacer. La persona revisa y confirma con un clic.

**Módulo 1: revisiones sin resultado** (el núcleo, porque son solo reglas).

- Avisa de las citas de revisión de los próximos días cuya prueba no tendrá
  resultado a tiempo: `PRUEBAS.fecha_programada` o `estado_prueba` frente a la
  fecha del hueco de `CITAS.id_cita_revision`.
- Propone moverla al primer hueco libre después del resultado previsto, en la
  agenda que toca.
- Prepara el aviso al paciente por su canal preferido
  (`PACIENTES.preferencias_contacto`).

**Módulo 2: huecos liberados** (también solo reglas, sin predicción).

- Cuando se anula una cita (`MOVIMIENTOS_CITAS`), propone a quién ofrecer el
  hueco. Mira las derivaciones que esperan para esa especialidad, por prioridad
  y antigüedad, y solo a pacientes cuyas necesidades de accesibilidad encajen.
- Prepara la oferta por el canal adecuado.

**Pantalla de impacto**, calculada con los datos:

- revisiones que se hicieron sin resultado;
- huecos que se perdieron;
- minutos de tareas administrativas asociadas (`TAREAS_ADMINISTRATIVAS`);
- lo que se habría recuperado con la bandeja.

**Fuera del MVP:**

- La **predicción de ausencias**: hace falta validar el modelo y vigilar los
  sesgos. Va como fase 2 en el pitch.
- Integración real con el HIS, envío real de mensajes y login.

### Por qué es viable

Esto es lo que hay que contar en el pitch.

- **No toca los sistemas del hospital.** Solo lee: empieza con una exportación
  diaria en CSV y después pasa a una lectura con HL7/FHIR. Hay que preguntar a
  Informática qué HIS usan y qué pueden exportar.
- **Solo reglas en la primera versión.** Se pueden explicar y auditar, y no hace
  falta validar ningún modelo clínico para pilotar.
- **El personal decide.** La bandeja propone y nunca cambia una cita sola.
- **Privacidad:** funciona dentro de la red del hospital y los datos no salen.
- **Lo mantiene su propio equipo**, la Unidad de Análisis de Datos del IIS‑Princesa.
- **Plan de piloto:** un servicio, tres meses, con indicadores medidos antes y
  después:
  - revisiones sin resultado;
  - huecos perdidos;
  - minutos administrativos;
  - días de espera.

### Herramientas (propuesta)

Lo más rápido para una aplicación de datos que corra en local:

- **Python + DuckDB** para consultar los CSV directamente con SQL.
- **Streamlit** para la bandeja y la pantalla de impacto.

Así no hay base de datos que montar y el mismo código sirve en un ordenador del
hospital. Se puede cambiar según lo que domine el equipo.

### Reparto del tiempo (orientativo)

| Horas | Qué |
|---|---|
| 0‑2 | Cargar los CSV y comprobar con números que los dos problemas existen y cuánto pesan |
| 2‑3 | Decidir, ponerle nombre y la frase de «X que hace Y». Hablar con los mentores de Consultas Externas e Informática |
| 3‑13 | Construir: capa de datos, reglas de los dos módulos y bandeja |
| 13‑16 | Pantalla de impacto |
| 16‑20 | Pitch: problema, cifra, demo, por qué es viable, piloto, modelo de negocio |
| 20‑22 | Grabar un vídeo de la demo por si falla en directo |
| 22‑24 | Ensayar y margen |

## Cómo usar los datos

Los datos sirven para identificar ineficiencias, analizar procesos, detectar
tareas que se pueden automatizar o desarrollar y validar la propuesta.

> **Importante:** los datos son **sintéticos** y solo son un apoyo. No es
> obligatorio usar todos los archivos ni que la solución se base en datos.

**Material:** 11 CSV con datos sintéticos y un PDF que explica la estructura y
las relaciones entre los archivos. Todavía no están en el repo; irán en
[`data/`](../data/).

## Por dónde empezar (recomendación de la organización)

No hay que resolver todos los problemas de consultas externas: hay que elegir
**una necesidad concreta** y construir sobre ella.

1. **Identificar el problema:** ¿qué tarea o proceso consume tiempo innecesariamente?
2. **Analizar la situación actual** con los datos y con los mentores.
3. **Plantear la solución:** ¿cómo simplificar, mejorar o automatizar ese proceso?
4. **Desarrollar un prototipo** que permita ver cómo funcionaría.
5. **Demostrar su impacto:** qué tiempo, recursos o pasos ahorra y cómo se mediría.

Preguntas que la propuesta debe responder:

- **¿Cuál es su dimensión?** ¿Cuánto tiempo o recursos consume hoy?
- **¿Cómo funciona la solución?**
- **¿Qué impacto tendría?** ¿Cuánto tiempo clínico libera? ¿Cómo mejora la experiencia de pacientes y profesionales?
- **¿Es viable?** ¿Se podría implantar en un hospital real? Sistemas existentes, integración, privacidad y recursos necesarios. Para esto, consultar a los mentores.

## Los datos: 11 CSV

El conjunto simula las consultas externas del hospital: desde que Atención
Primaria pide la consulta hasta que se asigna la cita, se envían
comunicaciones, se atiende al paciente y se hacen las tareas administrativas.

| Archivo | Qué contiene |
|---|---|
| `PACIENTES` | Edad, sexo, preferencias de contacto y accesibilidad |
| `DERIVACIONES_AP` | Solicitudes de consulta de Atención Primaria |
| `AGENDAS` | Especialidades, profesionales ficticios, horarios y capacidad |
| `HUECOS_AGENDA` | Franjas horarias libres u ocupadas |
| `CITAS` | Citas asignadas, estados, prioridades y tipo de visita |
| `MOVIMIENTOS_CITAS` | Historial de creaciones, anulaciones y cambios |
| `OCUPACIONES_HUECO` | Qué cita ocupa cada franja de agenda |
| `COMUNICACIONES` | Recordatorios, canales y respuestas de pacientes |
| `TAREAS_ADMINISTRATIVAS` | Tareas, duración, perfil profesional y si son automatizables |
| `CIRCUITO_CONSULTA` | Horas de llegada, admisión, entrada y salida de consulta |
| `PRUEBAS` | Pruebas solicitadas, programadas y realizadas |

### Modelo entidad‑relación

Transcrito del diagrama ([`briefing/4-modelo-er.jpeg`](briefing/4-modelo-er.jpeg)).
`FK?` es una clave foránea opcional.

```text
PACIENTES               Perfil ficticio del paciente
  PK id_paciente
     edad, sexo, preferencias_contacto, necesidades_accesibilidad

DERIVACIONES_AP         Solicitudes de Atención Primaria
  PK id_derivacion
  FK id_paciente            → PACIENTES
     fecha_solicitud, especialidad_destino, prioridad, motivo_derivacion,
     observaciones_medico, estado_solicitud

AGENDAS                 Oferta de consulta por servicio
  PK id_agenda
     servicio, especialidad, tipo_consulta, profesional_ficticio,
     duracion_cita, horario_inicio, horario_fin, capacidad

HUECOS_AGENDA           Franjas citables de cada agenda
  PK id_hueco
  FK id_agenda              → AGENDAS
     fecha, hora_inicio, hora_fin, estado_hueco, tipo_hueco

CITAS                   Entidad central · estado vigente
  PK id_cita
  FK  id_paciente           → PACIENTES
  FK? id_derivacion         → DERIVACIONES_AP
  FK  id_hueco              → HUECOS_AGENDA (el vigente)
     fecha_asignacion, tipo_visita, estado_actual, prioridad

MOVIMIENTOS_CITAS       Historial de eventos de la cita
  PK id_movimiento
  FK  id_cita               → CITAS
     fecha_hora_movimiento, tipo_movimiento, origen_movimiento, motivo,
     estado_anterior, estado_nuevo
  FK? id_hueco_anterior     → HUECOS_AGENDA
  FK? id_hueco_nuevo        → HUECOS_AGENDA

OCUPACIONES_HUECO       Qué cita ocupa cada hueco (máx. 1 ocupación activa por hueco)
  PK id_ocupacion
  FK id_hueco               → HUECOS_AGENDA
  FK id_cita                → CITAS
     inicio_ocupacion, fin_ocupacion, motivo_fin, ocupacion_activa

COMUNICACIONES          Avisos, ofertas y respuestas
  PK id_comunicacion
  FK? id_cita               → CITAS
  FK  id_paciente           → PACIENTES
  FK? id_hueco_ofertado     → HUECOS_AGENDA
     canal, motivo, fecha_envio, respuesta_paciente, resultado

TAREAS_ADMINISTRATIVAS  Gestiones manuales y su duración
  PK id_tarea
  FK? id_cita               → CITAS
     perfil_profesional, tipo_tarea, fecha_hora_inicio, duracion_minutos,
     automatizable, incidencia_asociada

CIRCUITO_CONSULTA       Tiempos de la cita atendida (0..1 por cita)
  PK id_registro
  FK id_cita                → CITAS (única)
     hora_llegada, hora_admision, hora_entrada_consulta,
     hora_salida_consulta, tiempo_espera, tiempo_atencion

PRUEBAS                 Pruebas solicitadas en consulta
  PK id_prueba
  FK  id_paciente           → PACIENTES
  FK  id_cita_origen        → CITAS
  FK? id_cita_revision      → CITAS
     tipo_prueba, fecha_solicitud, fecha_programada, fecha_realizacion,
     estado_prueba, resultado_disponible
```

El flujo es demanda → oferta → cita:
`DERIVACIONES_AP` → `CITAS` ← `HUECOS_AGENDA` ← `AGENDAS`. Alrededor de la cita
están su historial (`MOVIMIENTOS_CITAS`, `OCUPACIONES_HUECO`), lo que se le dice
al paciente (`COMUNICACIONES`), lo que se hace a mano (`TAREAS_ADMINISTRATIVAS`),
el día de la consulta (`CIRCUITO_CONSULTA`) y lo que se pide en ella (`PRUEBAS`).

## Mentores

Hay mentores del **Hospital Universitario de La Princesa** y de **Pfizer** para
resolver dudas, contrastar ideas, entender el entorno sanitario y valorar la
viabilidad. Los nombres están en la hoja impresa; aquí solo se recogen los
cargos, para saber a quién preguntar.

**Hospital**

- Ingeniería biomédica · Unidad de Datos y Unidad de Información Clínico‑Asistencial
- Supervisión de enfermería · Unidad de Consultas Externas (2 personas)
- Supervisión de área en procesos enfermeros e I+D+i · Dirección de Enfermería
- Tecnologías y sistemas de información
- Medicina de Urgencias
- Facultativo de Endocrinología y Nutrición
- Ingeniería biomédica · Unidad de Innovación
- Data scientist · Unidad de Apoyo a la Innovación
- Servicio de Informática
- Dirección Médica del hospital

**Pfizer**

- Comunicación y emprendimiento
- Acceso al mercado, innovación hospitalaria y rediseño de procesos
- Precio y financiación de medicamentos, sistema sanitario
- Marketing (varias personas) y estrategia de canales digitales
- Delegada digital (e‑rep), antes en supply chain
- Estrategia digital
- Cardiovascular, vacunas y antiinfecciosas
- Cirugía, oncología y urgencias
- Ventas (hospitales privados)

A quién preguntar:

- **Cómo funciona hoy la consulta externa:** supervisión de enfermería de Consultas Externas.
- **Datos y su significado:** Unidad de Datos y data scientist de Innovación.
- **Integración con los sistemas del hospital y privacidad:** Informática y Tecnologías y sistemas de información.
- **Si se podría pilotar:** Dirección Médica y Unidad de Innovación.

## Ganadores de ediciones anteriores

Datos de la página de cada edición en la web de la Fundación Pfizer.

| Año | Reto | 1.º | 2.º | 3.º |
|---|---|---|---|---|
| 2025 | Acelerar el uso de biomarcadores en la práctica clínica | **BioPatch**: parche que detecta el cáncer de mama midiendo la conductividad eléctrica de las células, sin invasión y barato | — | — |
| 2024 | Resistencia a los antimicrobianos | **Aeris** (3.000 €): plataforma con IA que detecta pronto las infecciones hospitalarias | **ReCero** (2.000 €): dispositivo para depositar antibióticos sobrantes, con incentivos | **Acté** (1.000 €): tejido antimicrobiano |
| 2023 | Riesgo cardiovascular | **Thea** (3.000 €): pulsera que mide la tensión en tiempo real y la manda a una app | **Microbuddy** (2.000 €): parche que mide biomarcadores cardiovasculares y envía los resultados al móvil | **Coda by Ateraid** (1.000 €): parche que vigila de forma continua el riesgo de aterosclerosis |
| 2022 | Salud mental con tecnología / sanidad sostenible | **Biopamp** (3.000 €): polímeros que limpian las aguas residuales de bacterias resistentes | **Medea** (2.000 €): software con datos para personalizar tratamientos de salud mental | **InMed** (1.000 €): que se reciclen más medicamentos con el punto SIGRE |
| 2021 | Que el paciente entienda su enfermedad / verificar la información sanitaria en internet | **Cori – Better Diabetes**: asistente digital para personas con diabetes | **Paigo**: app que explica a los niños su enfermedad antes, durante y después de la consulta | **ALMA**: IA que valida la información de salud que circula en internet |
| 2020 | Stock y dispensación de medicamentos / eficiencia de la telemedicina | **Pharmawhale** (3.000 €): conectar las farmacias de hospital para gestionar y planificar existencias, con IA | **The Warriors – Care4u** (2.000 €): telemedicina útil para pacientes y profesionales | **Valhalla – Health EDI** (1.000 €): app de dispensación y stock con IA que ajusta las compras |
| 2019 | Adherencia al tratamiento y diagnóstico precoz | **BioMakers – Hera** (3.000 €): sistema que acompaña a pacientes con depresión | **Contigo** (2.000 €): seguimiento por visión con IA para avisar de caídas o riesgos en mayores | **Previctus** (1.000 €): app con machine learning y pulseras para detectar pronto el ictus |

En 2021 se repartieron 6.000 € entre los tres, sin decir cuánto a cada uno. En
2025 solo se publicó el ganador.

El precedente más cercano a este año es **2020**: es el único reto de gestión
del hospital y no de un dispositivo clínico, y en él ganaron Pharmawhale y
Health EDI con IA aplicada a la logística.

### Qué tienen en común los primeros puestos

Son patrones sacados de 7 casos con una sola frase de descripción cada uno, no
criterios oficiales del jurado.

1. **Un nombre corto y fácil de recordar.** Hera, Pharmawhale, Cori, Biopamp, Thea,
   Aeris y BioPatch. Ninguno se llama «Plataforma de…».
2. **Se explican en una frase de «X que hace Y»**, como «pulsera que mide la
   tensión en tiempo real». Cada uno es un solo objeto que resuelve un solo
   problema.
3. **Responden a la pregunta del año al pie de la letra.**
4. **La mayoría se adelanta al problema en vez de reaccionar** (5 de 7): detectar
   o vigilar antes de que pase (Aeris, BioPatch, Thea), planificar antes de que
   falte (Pharmawhale) o acompañar para que el paciente no abandone el
   tratamiento (Hera).
5. **Se entienden y se ven sin explicaciones**: un dispositivo que se puede
   enseñar o un asistente o plataforma con una tarea clara.

No todos son hardware ni todos llevan IA. Unos se centran en el paciente y
otros en el sistema sanitario.

## Ideas candidatas

Siguen la fórmula de los ganadores: un producto con nombre que se adelanta a un
problema concreto de consultas externas, con una cifra de impacto calculada con
los datos de la organización (el paso 5 de su hoja).

### 1. Revisiones sin resultado

Detectar, con días de margen, las citas de revisión a las que la prueba no habrá
llegado y moverlas antes de que se pierdan.

- **Datos:** `PRUEBAS` (`fecha_programada`, `fecha_realizacion`,
  `resultado_disponible`, `id_cita_revision`) cruzado con `CITAS` y su hueco en
  `HUECOS_AGENDA`.
- **Impacto que se puede medir:** revisiones que se hicieron sin el resultado,
  cuántas se recolocaron después (`MOVIMIENTOS_CITAS`) y los minutos de
  `TAREAS_ADMINISTRATIVAS` ligados a esas citas.

### 2. Huecos vacíos

Prever qué citas se van a anular o el paciente no va a venir, y ofrecer ese hueco
antes de que quede libre.

- **Datos:** el historial de `MOVIMIENTOS_CITAS` y `COMUNICACIONES`
  (`respuesta_paciente`, `resultado`, `id_hueco_ofertado`) para aprender quién
  falla. Después `OCUPACIONES_HUECO` y `HUECOS_AGENDA` para ver los huecos que se
  quedaron sin usar, y `DERIVACIONES_AP` para saber quién espera.
- **Impacto que se puede medir:** huecos perdidos al mes, ofertas que acabaron en
  cita, llamadas manuales evitadas (`TAREAS_ADMINISTRATIVAS` con
  `automatizable`) y días de espera recortados.

### Cómo cumplen los pilares

| | Revisiones sin resultado | Huecos vacíos |
|---|---|---|
| **Viabilidad** | Alta: son reglas sobre datos que ya existen y no necesitan modelo | Media: hace falta un modelo de ausencias y validarlo |
| **Accesibilidad** | Avisa por el canal del paciente y evita viajes inútiles; puede juntar prueba y revisión | Ofrece el hueco por el canal adecuado y solo si es compatible con sus necesidades |
| **Medicina personalizada** | Fuerte: la revisión se pone según las pruebas de cada paciente | Media: el apoyo depende del historial de cada uno (con el cuidado de arriba) |

Las dos se pueden combinar en un mismo producto: un asistente que, para cada
paciente, se adelanta a lo que va a fallar en su circuito.

## Fuentes

- [Fundación Pfizer · Innomakers4Health](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health)
- [La Princesa · premios Best in Class en IA 2026](https://www.comunidad.madrid/hospital/laprincesa/noticia/hospital-publico-princesa-obtiene-dos-premios-primera-edicion-best-in-class-inteligencia-artificial)
- [IIS‑Princesa · Unidad de Apoyo a la Innovación](https://www.iis-princesa.org/innovacion/unidad-innovacion/)
- [Roche · humanización en La Princesa](https://www.roche.es/historias/humanizacion-hospitales/hospital-universitario-princesa-madrid)
- [Portal ERP · Pfizer y Deloitte crean InnovaSalud Spain](https://portalerp.com/es/noticia/pfizer-y-deloitte-crean-una-red-con-14-hospitales-para-acelerar-la-ia-sanitaria)
- [Corresponsables · entrevista a la Fundación Pfizer](https://www.corresponsables.com/entrevistas/ponemos-foco-innovacion-salud-base-nuestra-actividad-pfizer/)
- [Fundación Pfizer · Memoria 2023](https://www.fundacionpfizer.org/sites/default/files/Memoria2023.pdf)
- [El Referente · Wayra invirtió 9,3 M€ en 37 startups en 2024](https://elreferente.es/?p=174557)
- [Emprendedores · Innomakers4Health 2025](https://emprendedores.es/startups/innomakers4health-wayra/)
- Ediciones anteriores: [2025](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2025) · [2024](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2024) · [2023](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2023) · [2022](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2022) · [2021](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2021) · [2020](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2020) · [2019](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health/innomakers4health-2019)
- [Wayra · anuncio del reto](https://wayra.com/news/wayra-pfizer-and-the-pfizer-foundation-launch-a-challenge-to-optimise-efficiency-in-hospital-outpatient-clinics)
- [La Ecuación Digital](https://www.laecuaciondigital.com/empresas/healthtech/wayra-pfizer-reto-consultas-externas-hospitalarias/)
