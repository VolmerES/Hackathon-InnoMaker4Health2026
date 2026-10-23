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
- **Al director del hospital le gusta la IA local** (dicho al equipo el 23‑oct).
  Está en el jurado.

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

### La restricción que manda: personas y dinero

Lo que más falta en un hospital es **personal y presupuesto** (dicho por el
equipo, 23‑oct). Por eso la solución tiene que resolver lo máximo posible con el
mínimo de las dos cosas:

- **No puede añadir trabajo.** Lo rutinario tiene que ir solo y el personal solo
  ve lo que de verdad necesita a una persona.
- **Tiene que costar casi nada.** Software libre, sin licencias de terceros y,
  como mucho, un servidor dentro del hospital para la IA local, si su Unidad de
  Análisis de Datos no tiene ya uno ⚠️.
- **El paciente se gestiona solo cuando puede** (con un enlace para elegir la
  hora). Quien no puede pasa a una persona. **La automatización libera tiempo
  justo para atender a quien más lo necesita.**

### La idea elegida: la puerta de entrada (Jano)

**Decidido el 23‑oct por la tarde**, por tres motivos:

- **Media sala va a por los pacientes que faltan.** Hay que diferenciarse.
- **Al director de La Princesa, que está en el jurado, le gusta la IA local.**
- **Hay evidencia publicada.** Cuando la prueba ya está hecha, la primera
  consulta resuelve el caso en una sola visita en el **49 %** de los pacientes,
  frente al **22 %** sin ella (estudio multicéntrico, 795 primeras visitas). En
  Digestivo sube del 34 al 57 %.

**Nombre provisional: Jano**, el dios romano de las puertas y los comienzos.
Tiene dos caras: una mira a Primaria y la otra al hospital. Sigue la línea de
nombres de los ganadores (Hera, Thea, Aeris). Alternativas: *Umbral*, *Prima*.

**La frase:** *Jano lee cada noche, dentro del hospital, las derivaciones de
Primaria y prepara cada caso para que la primera consulta lo resuelva.*

**El problema que ataca:**

- La Princesa hace **1,65 consultas sucesivas por cada primera**: 294.369
  sucesivas al año (memoria de 2022).
- Muchas de esas sucesivas solo sirven para ver el resultado de una prueba que se
  podía haber hecho antes.
- Además, en los CSV, **dos de cada tres derivaciones no son un problema nuevo**
  ([HALLAZGOS](HALLAZGOS.md#7-la-puerta-de-entrada)):
  - **revisiones** de pacientes que el servicio ya conoce;
  - pacientes que **perdieron su revisión** y tienen que volver a pasar por su
    médico de cabecera.

  El tamaño es sintético; hay que preguntar a la Unidad de Datos cuánto pesa en
  La Princesa ⚠️.

### Qué construir

| Paso | Qué hace | Quién |
|---|---|---|
| 1. **Leer** | Cada noche, la IA local lee las derivaciones nuevas y saca una ficha: tipo (nueva, revisión o reanudar seguimiento), edad, antecedentes, tiempo de evolución, **pruebas ya hechas en Primaria**, datos que faltan, un resumen de una frase y lo segura que está | IA local |
| 2. **Encaminar** | Reglas de cada servicio: *reanudar seguimiento* → cita de revisión directa, sin primera consulta ni nueva validación. *Revisión por empeoramiento* → revisión preferente. *Nueva* → protocolo de pruebas según el motivo, **sin repetir las que ya hizo Primaria**. *Faltan datos* → se devuelve al médico de cabecera con la pregunta concreta. *Poca confianza* → a una persona | Reglas |
| 3. **Validar en bloque** | El especialista ve la lista de la semana con la ficha y la propuesta de cada derivación, y la aprueba o corrige con un clic. Hoy ya valida derivaciones; con la ficha tarda menos que leyendo el texto | Especialista |
| 4. **Citar** | Las pruebas se programan antes de la primera consulta y la primera, cuando el resultado ya está. Si se puede, prueba y consulta el mismo día (acto único). El paciente elige la hora desde un enlace | Automático |
| 5. **Consultar** | El especialista recibe al paciente con el resumen y los resultados, y lo resuelve en una visita | Especialista |

**La IA solo lee y extrae; no decide nada clínico.** Lo clínico lo deciden los
protocolos que aprueba cada servicio y el especialista, que valida en bloque.

**Pantalla de impacto**, con los datos de la demo:

- derivaciones procesadas, y cuántas de cada tipo;
- **acierto del lector frente al `motivo_derivacion` real de los CSV**;
- pruebas adelantadas y pruebas duplicadas evitadas;
- revisiones evitadas (escenario);
- devoluciones por datos incompletos;
- casos enviados a una persona.

**Fuera del MVP:**

- La integración real con el HIS y la petición electrónica de pruebas.
- Ajustar el modelo con texto real.
- Que el propio paciente pueda volver a pedir su revisión perdida sin pasar por
  Primaria (fase 2).

**Si la IA falla en directo:** se precalculan las fichas de la demo y hay una
regla de respaldo. En los CSV el texto es de plantilla, así que una regla
sencilla también clasifica bien.

### Por qué es viable

Esto es lo que hay que contar en el pitch.

- **IA local:** corre en un servidor del hospital. **Ningún dato de un paciente
  sale a la nube** y no se paga por uso.
- **Poca carga:** La Princesa recibe unas 63.000 derivaciones de Primaria al año
  (el 35 % de sus 178.888 primeras), unas **250 por día laborable**, que se
  procesan de noche.
- **Modelos abiertos con licencia libre** (Apache 2.0): sin coste de licencia y
  se pueden usar en un producto comercial.
- **Mes en paralelo:** durante un mes, Jano lee las mismas derivaciones que el
  personal sin decidir nada, y se mide su acierto antes de activarlo.
- **Las reglas son del servicio y se pueden auditar.** La IA solo extrae.
- **No añade personal:** el especialista valida fichas en bloque en lugar de leer
  derivaciones una a una, y admisión cita menos sucesivas.
- **Privacidad:** los datos de salud no salen de la red del hospital.
- **Lo mantiene su propio equipo**, la Unidad de Análisis de Datos del IIS‑Princesa.
- ⚠️ **Lo regulatorio:** adelantar una prueba es una decisión clínica, aunque la
  tome un protocolo. Se mitiga con protocolos aprobados por el servicio y
  validación del especialista. Hay que confirmar con los mentores si entra en el
  reglamento de productos sanitarios (MDR).
- **Plan de piloto:** **Digestivo**, que es donde hay evidencia (del 34 al 57 %),
  durante tres meses, midiendo antes y después:
  - % de primeras resueltas en una visita;
  - sucesivas por primera;
  - días hasta el diagnóstico;
  - pruebas duplicadas;
  - minutos de validación.

### Aparcado: los huecos vacíos

El diseño anterior era un circuito automático contra las faltas: recordatorio
con respuesta, citas dobles, segundo aviso y reofrecer el hueco, con una bandeja
solo de excepciones. Se aparca porque media sala va a ir a por eso. Está en el
historial de git (commit `8726248`) y sus cifras reales siguen en
[`HALLAZGOS.md`](HALLAZGOS.md#cifras-reales-para-el-pitch). Si sobra tiempo, el
enlace al paciente para elegir hora es la misma pieza.

### El equipo

| Quién | Perfil | Papel en el hackathon |
|---|---|---|
| Dev 1 | Software y robótica | **El lector y las reglas**: la IA local (instrucciones, ficha en JSON, medir el acierto contra los CSV), las reglas de encaminado y la API. Cuenta la viabilidad en el pitch |
| Dev 2 | Software | **Frontend**: la bandeja de validación del especialista, la ficha y la pantalla de impacto |
| UI/UX | Diseño | **La ficha** (lo que ve el especialista en 10 segundos), la devolución al médico de cabecera y el mensaje al paciente, con accesibilidad (WCAG). Hace las diapositivas y **presenta la demo** |
| Biomédica | Ingeniería biomédica | **Los protocolos**: qué pruebas adelantar según el motivo en cada especialidad, validados con los mentores clínicos (Endocrinología, Urgencias, Enfermería de Consultas Externas). Lo regulatorio y la evidencia. **Abre el pitch** |

Esto lo propone Claude; el reparto lo decide el equipo.

**La robótica no hace falta aquí.** Este reto premia la viabilidad, y meter
hardware la resta.

### Herramientas (propuesta)

- **IA local:** modelos preentrenados de Hugging Face (ver
  [Modelos de Hugging Face](#modelos-de-hugging-face)), servidos dentro del
  hospital. En [`analisis/lector_ia.py`](../analisis/lector_ia.py) hay un primer
  lector, todavía sin probar, que mide el acierto contra los CSV.
- **Backend:** Python + FastAPI + DuckDB sobre los CSV (ya está
  [`analisis/datos.py`](../analisis/datos.py)).
- **Frontend:** una web sencilla con el framework que mejor domine el Dev 2.
- **Diseño:** Figma en las primeras horas, para que el frontend no espere.

### Modelos de Hugging Face

Todos se descargan una vez y después funcionan **sin internet**, dentro del
hospital (`HF_HUB_OFFLINE=1`). Licencias y descargas comprobadas en el Hub el
23‑oct.

**Capa 1: modelos pequeños, sin GPU.** Hacen casi todo el trabajo. Son de unos
125 M de parámetros y corren en un servidor normal en milisegundos.

| Para qué | Modelo | Licencia |
|---|---|---|
| **Pruebas ya hechas** («analítica», «ecografía»…) | [`BSC-NLP4BIA/bsc-bio-ehr-es-carmen-medprocner`](https://huggingface.co/BSC-NLP4BIA/bsc-bio-ehr-es-carmen-medprocner): procedimientos en texto clínico español (F1 0,81) | CC‑BY‑4.0 |
| **Antecedentes y enfermedades** (HTA, DL, hipotiroidismo…) | [`BSC-NLP4BIA/bsc-bio-ehr-es-carmen-distemist`](https://huggingface.co/BSC-NLP4BIA/bsc-bio-ehr-es-carmen-distemist) | CC‑BY‑4.0 |
| **Negaciones** («sin hallazgos», «sin mejoría») | [`BSC-NLP4BIA/negation-tagger`](https://huggingface.co/BSC-NLP4BIA/negation-tagger) | CC‑BY‑4.0 |
| **Anonimizar** antes de guardar registros | [`BSC-NLP4BIA/bsc-bio-ehr-es-meddocan`](https://huggingface.co/BSC-NLP4BIA/bsc-bio-ehr-es-meddocan) | CC‑BY‑4.0 |
| **Tipo de derivación** (nueva, revisión o reanudar) | [`PlanTL-GOB-ES/bsc-bio-ehr-es`](https://huggingface.co/PlanTL-GOB-ES/bsc-bio-ehr-es), RoBERTa clínico español, ajustado con las derivaciones etiquetadas. O sin entrenar: [`MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7`](https://huggingface.co/MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7) | Apache 2.0 / MIT |

Los modelos de la capa 1 son del **Barcelona Supercomputing Center**, del Plan
de Tecnologías del Lenguaje del Gobierno. Están entrenados con historias
clínicas en español. Para el pitch: *IA pública española, dentro del hospital*.

**Capa 2, opcional: un modelo generativo para el resumen de una frase.**

| Modelo | Licencia | Nota |
|---|---|---|
| [`BSC-LT/salamandra-7b-instruct`](https://huggingface.co/BSC-LT/salamandra-7b-instruct) | Apache 2.0 | Del proyecto público español ALIA (BSC). Hay versión de 2B |
| [`Qwen/Qwen3-4B`](https://huggingface.co/Qwen/Qwen3-4B) o [`Qwen3-8B`](https://huggingface.co/Qwen/Qwen3-8B) | Apache 2.0 | Los más descargados. Buenos dando JSON con formato fijo |

Necesita una GPU de gama media o CPU de noche: 250 derivaciones caben de sobra.
Si no hay presupuesto para GPU, **Jano funciona sin esta capa**.

**Evitar:** `google/medgemma-*` (acceso restringido y licencia propia) y
`mistralai/Ministral-8B-Instruct-2410` (licencia no comercial). Los dos chocan
con un producto que Wayra quiera vender.

**Para el pitch:**

- **El hospital ya etiqueta cada derivación al validarla.** Esas etiquetas
  sirven para reajustar el clasificador dentro del hospital, gratis y sin que
  los datos salgan.
- **CC‑BY‑4.0** permite uso comercial; solo obliga a citar al BSC.

**Ojo con la demo:** el texto de los CSV es de plantilla. Un clasificador
ajustado con él acertará casi el 100 %, y esa cifra no vale nada. Lo que luce
es la **ficha**: pruebas, antecedentes y negaciones sacadas del texto. Si se da
una cifra de acierto, hay que decir que es sobre datos sintéticos.

### Más usos de la IA local

Ideas para sumar a Jano, ordenadas de más a menos recomendable para estas 24
horas. Todas funcionan dentro del hospital y con licencias que permiten uso
comercial (comprobadas en Hugging Face el 23‑oct).

| # | Uso | Modelos | Pilar | Esfuerzo |
|---|---|---|---|---|
| 1 | **Instrucciones de la prueba accesibles.** La cita y la preparación («ven en ayunas») en lectura fácil, **en el idioma del paciente y en audio**, para quien necesita intérprete o tiene discapacidad visual | Generativo de la capa 2 para la lectura fácil. Traducción: `Helsinki-NLP/opus-mt-es-*` o `google/madlad400-3b-mt` (Apache 2.0). Voz: `hexgrad/Kokoro-82M` (Apache 2.0) o Piper (MIT) | Accesibilidad | Bajo |
| 2 | **Devolución redactada.** Si faltan datos, la IA escribe al médico de cabecera la pregunta concreta | Generativo | Viabilidad | Bajo |
| 3 | **Resumen antes de la consulta.** Derivación y resultados en tres líneas para el especialista | Generativo | Tiempo clínico | Bajo |
| 4 | **Candidatas a e‑consulta.** Marca las derivaciones que podrían resolverse por escrito, sin visita (por ejemplo, dudas sobre una analítica), y redacta un borrador de respuesta que el especialista firma. En Cardiología, la interconsulta no presencial cerró el 54 % de los casos sin visita | Clasificador y generativo | Tiempo clínico | Medio · ⚠️ clínico |
| 5 | **Señales de alarma.** Detecta en el texto cosas como «pérdida de peso» o «sangrado», sin negación, y avisa al especialista de que revise la prioridad | `carmen-distemist` + `negation-tagger` | Personalizada | Medio · ⚠️ MDR |
| 6 | **Anonimizar** las derivaciones para que la Unidad de Datos pueda usarlas en investigación | `meddocan` | Viabilidad | Bajo |
| 7 | **Dictado del informe.** El especialista dicta y se escribe solo | `openai/whisper-large-v3-turbo` (MIT) | Tiempo clínico | Alto: otro producto, fase 2 |

**Evitar** `facebook/nllb-*`, que tiene licencia no comercial, y
`BSC-LT/salamandraTA-7b-instruct`, que es GPL‑3.0.

**Recomendación:**

- **Sumar el 1 a la demo.** María recibe las instrucciones de su ecografía en
  árabe y en audio. Es barato, se ve muy bien y apuntala la accesibilidad, que
  es lo de la Fundación Pfizer.
- **El 2 y el 3** salen casi gratis si ya hay modelo generativo.
- **El 4 y el 5** van como una frase de fase 2 en el pitch.
- **No meter el 7:** es otro producto y probablemente otros equipos hagan algo
  parecido.

### Reparto del tiempo (orientativo)

| Bloque | Devs | UI/UX | Biomédica |
|---|---|---|---|
| 1 | Contrato de la API (la ficha en JSON). Lector con el modelo elegido y su acierto sobre una muestra | Boceto de la ficha y de la bandeja | Mentores: ¿pasa esto en La Princesa? ¿Quién valida las derivaciones? ¿MDR? |
| 2 | Reglas de encaminado y backend. Bandeja con datos reales | Diseño final y textos (devolución al médico de cabecera, mensaje al paciente) | Protocolos de Digestivo (y de una segunda especialidad si da tiempo) |
| 3 | Pantalla de impacto. Precalcular las fichas de la demo | Diapositivas | Cifras y guion del pitch |
| 4 | Congelar el código. Grabar un vídeo de la demo por si falla | Ensayar | Ensayar |

## Cómo funcionan las derivaciones en Madrid

Según la web de la Comunidad de Madrid (Cita Sanitaria). Lo marcado con ⚠️ hay
que confirmarlo con los mentores.

1. **El médico de cabecera decide derivar.** Escribe el **parte de
   interconsulta** (texto libre: motivo, antecedentes, pruebas hechas) y da al
   paciente un **justificante de petición de cita** con un número.
2. **El paciente pide la primera cita:**
   - por internet, con la app *Cita Sanitaria Madrid* o en el quiosco del centro
     de salud;
   - o le llama el **Centro de Gestión de Citas** en menos de 3 días laborables.
3. **Libre elección:** puede elegir cualquier hospital o centro de
   especialidades con agenda abierta. **Todas las pruebas y consultas de ese
   proceso se hacen en el centro elegido.**
4. **La cita cae en una agenda del hospital.** Si el hospital revisa la
   derivación y quién lo hace (admisión o el especialista), no lo dice la web ⚠️.
   En los CSV existen `VALIDACION_DERIVACION` y los estados `EN_REVISION` y
   `RECHAZADA`.
5. **El día de la consulta**, el paciente lleva el parte de interconsulta y sus
   informes. Los profesionales pueden ver la historia de Primaria con el visor
   **HORUS**.
6. **Las revisiones las gestiona el propio hospital.** En algunos hospitales el
   paciente puede cambiarlas por internet. Si pierde la revisión, en la práctica
   vuelve a su médico de cabecera para una derivación nueva. Eso es lo que
   reflejan los CSV con «Reanudar seguimiento» ⚠️.
7. **Lo urgente no va por aquí**, sino por Urgencias. Y en varias comunidades
   existe la **interconsulta no presencial** (e‑consulta): el médico de cabecera
   pregunta por escrito y el especialista responde sin ver al paciente.

**Lo que significa para Jano:**

- **Entre la cita y la primera consulta pasan semanas.** Esa espera es el hueco
  para hacer las pruebas antes.
- **Jano lee el parte de interconsulta cuando la cita llega al hospital.** Hay
  que preguntar a Informática cómo llega: si electrónico, por HORUS o en papel ⚠️.
- **Con la libre elección, cada hospital solo ve a quien lo ha elegido**, y las
  pruebas se hacen allí mismo. Encaja: Jano funciona en el hospital que recibe.
- **La vuelta directa tras perder una revisión depende solo del hospital**, que
  es quien gestiona las revisiones. No hay que cambiar nada en Primaria.

## Lo que ya existe y qué aporta Jano

**Comprobado el 23‑oct.** Ninguna pieza de Jano es nueva por separado. Lo que no
hemos encontrado es **todo junto**: en el hospital, local y para cualquier
hospital. *No decir nunca «no existe nada parecido».*

### El precedente más cercano: Infanta Elena (Valdemoro), 2018

Póster de la AEG, *Gastroenterol Hepatol* 2021;44(Espec Congr):183. Lo firma
Aparato Digestivo del H. Infanta Elena, un hospital de la red pública madrileña
gestionado por Quirónsalud
([PDF](https://www.elsevier.es/es-revista-gastroenterologia-hepatologia-14-congresos-24-reunion-anual-asociacion-espanola-gastroenterologia-134-sesion-neurogastroenterologia-y-motilidad-6428-comunicacion-puede-la-inteligencia-artificial-mejorar-77422-pdf)).

- **Qué hacía:** desde julio de 2018, «un algoritmo de IA» leía los partes de
  derivación de Primaria, sacaba los motivos más frecuentes y les asignaba **un
  protocolo de pruebas antes de la primera consulta**. Es justo el núcleo de
  Jano.
- **Resultados:** primer semestre de 2019 frente al primero de 2018.

  | Indicador | Con el algoritmo (2019) | 2018 | Vía normal (2019) |
  |---|---|---|---|
  | Sucesivas por primera | **1,72** | 2,25 | 1,93 (p = 0,07) |
  | Analíticas en la primera visita | 49,4 % | 62,1 % | — |
  | TAC en la primera visita | 4,1 % | 8,2 % | — |
  | Endoscopias | 46,7 % | 54,5 % | 52,5 % |
  | Altas en la primera consulta | 11,4 % | 8,2 % | 10,8 % |

- **Su límite:** **solo llegó al 31,4 % de los pacientes** del área. Los mismos
  autores dicen que el uso fue bajo. El póster habla de pacientes «derivados
  usando el algoritmo» frente a «por vía normal», así que parece que dependía
  de que se eligiera esa vía ⚠️.

**Por qué nos ayuda en vez de quitarnos la idea:**

1. **Es evidencia en Madrid y en Digestivo**, el mismo servicio de nuestro
   piloto. Las sucesivas por primera bajaron un 24 %.
2. **Su problema fue el alcance**, y Jano lo resuelve: lee **el 100 %** de las
   derivaciones al llegar al hospital, sin que nadie cambie su forma de derivar.
3. **Era de 2018, de un servicio y de un hospital.** Jano usa IA de 2026, cubre
   todo el circuito (antes, durante y después), se conecta por HL7 y se instala
   en cualquier hospital.
4. **El director de La Princesa puede conocerlo.** Mejor citarlo nosotros que
   que nos lo saquen.

### El resto del mercado

| Pieza | ¿Existe? | Quién | Qué aporta Jano |
|---|---|---|---|
| Leer derivaciones y adelantar pruebas | Sí, en un servicio | Infanta Elena (2018); en Asturias, IA para priorizar dermatología en Primaria (Legit.Health) | El 100 % de las derivaciones, varios servicios, local e instalable |
| Leer cartas y avisar solo de lo que requiere acción | Sí, en Primaria (Reino Unido) | Docman10X (Advanced + U. de Manchester) | Lo mismo, pero del lado del hospital |
| Escriba en consulta | Sí, mucho | Dragon Copilot, Abridge, Nabla, Scribe de Quirón, el contrato de Madrid (nube) | Local, y conectado a lo que pasa antes y después |
| Chat con la historia clínica | Sí | Epic, Dragon Copilot | Local, con la fuente de cada dato y limitado a los pacientes del médico |
| Alta resolución, e‑consulta | Sí, hechas a mano | Muchos hospitales | Automatizarlo sin añadir personal |
| **Todo junto, local, por HL7 y fácil de instalar** | **No lo hemos encontrado** | — | **Jano** |

**Lo primero mañana con los mentores ⚠️:**

- ¿La Princesa tiene ya algo que lea o prepare las derivaciones?
- ¿Conocen lo del Infanta Elena? ¿Sigue funcionando?

Si La Princesa ya lo tiene, hay que saberlo antes del pitch, no durante.

## Cómo se conecta Jano al hospital: HCIS y HL7

**Dato del equipo (23‑oct):** la historia clínica de La Princesa es **HCIS**, y
se comunica con los demás sistemas mediante **HL7**.

**Jano no toca la base de datos de HCIS.** Se conecta como cualquier otro
sistema del hospital: recibe los mensajes HL7 que HCIS ya envía, por el motor de
integración del hospital, y guarda su propia copia de solo lectura. Si un día
cambia HCIS, Jano sigue funcionando.

| Tabla de los CSV | Mensaje HL7 v2 que la alimentaría |
|---|---|
| PACIENTES | `ADT^A04` / `A08` (alta y cambios del paciente, segmento `PID`) |
| DERIVACIONES_AP | `REF^I12` (derivación), o como la envíe Primaria ⚠️ |
| CITAS y MOVIMIENTOS_CITAS | `SIU^S12` nueva, `S13` reprogramada, `S14` modificada, `S15` anulada, `S26` no acudió |
| PRUEBAS | `OMG^O19` (petición) y `ORU^R01` (resultado) |
| Nota de la consulta | `MDM^T02` (documento clínico) |
| COMUNICACIONES | No suele ir por HL7: es la plataforma de SMS |

**Para escribir en HCIS** (citar, pedir una prueba) hace falta que HCIS acepte
peticiones (`SRM^S01`, `OMG^O19` o FHIR). En el piloto, Jano solo lee y
propone, y la persona confirma en HCIS. Escribir directamente es la fase 2 ⚠️.

**En la demo:** un script convierte los CSV en mensajes HL7 v2 y se los envía a
Jano por MLLP, como haría el motor de integración. Así el jurado ve que se
conecta sin tocar HCIS. Librerías: `hl7apy` o `hl7` (MIT).

**Preguntas para Informática ⚠️:**

- Qué versión de HL7 usan (2.3, 2.5) y qué motor de integración tienen.
- Qué mensajes publica ya HCIS (citas, pruebas, documentos).
- Si hay FHIR.
- Cómo llega la derivación de Primaria.

### La pieza innovadora: el escriba escribe, Jano actúa

La Consejería ya está comprando un escriba con IA para Primaria, y Quirónsalud
tiene «Scribe» en el Infanta Elena ([fuentes](#fuentes)). **No competimos con
ellos: nos conectamos a ellos.** Jano recibe la nota de cualquier escriba (HL7
`MDM^T02` o FHIR `DocumentReference`), o la que el especialista escribe a mano,
y la convierte en acciones.

| Momento | Qué hace Jano | De dónde lo saca |
|---|---|---|
| **Antes** | Ficha de la derivación y pruebas adelantadas | Derivación (`REF`) |
| **Durante** | **Pregúntale a Jano**: un agente con herramientas de solo lectura sobre el historial del paciente (derivaciones, citas, faltas, pruebas, avisos, accesibilidad). Siempre enseña las filas en las que se basa, y si un dato no está responde «no consta» | Copia HL7 |
| **Durante** | **Aviso de prueba duplicada**: el médico dice «le pido una analítica» y Jano avisa de que ya tiene una de hace tres semanas | Nota del escriba + `ORU` |
| **Al cerrar** | **Cero clics después de la consulta**: de la nota saca la lista de trámites (pruebas, próxima cita, vuelta a Primaria). El especialista la confirma con un clic | Nota del escriba (`MDM`) |
| **Después** | La vuelta a Primaria redactada, y la versión del paciente en lectura fácil, en su idioma y en audio | Modelos locales |

**La frase del pitch:** *«El escriba escribe la consulta. Jano hace todo lo que
viene antes y después.»*

**Por qué es innovador:**

- Es una capa de agentes **local y abierta** sobre los estándares que ya tiene
  el hospital (HL7).
- **Sirve con cualquier escriba**, también con el que compre la Consejería.
- **Cierra el circuito** de Primaria al hospital y de vuelta a Primaria.

**Qué no hacemos en 24 h:** el reconocimiento de voz. En la demo, la nota del
escriba es un texto preparado y se envía como `MDM^T02`. Si sobra tiempo, se
puede usar un audio grabado de antemano y pasado por `whisper-large-v3-turbo`,
**nunca en directo**.

**Regulación ⚠️:**

- Buscar y mostrar datos sin recomendar nada clínico no suele ser producto
  sanitario (guía MDCG 2019‑11).
- El aviso de prueba duplicada solo informa, pero hay que confirmarlo con los
  mentores.

## Dónde corre Jano: Mac mini M6 o NVIDIA DGX Spark

**Decidido el 23‑oct:** Jano se vende para **dos equipos**, el **Mac mini M6**
y el **NVIDIA DGX Spark**. Tiene que instalarlo **cualquiera, sin saber
informática**.

### Las dos versiones

| | **Jano Mini** · Mac mini M6 | **Jano Spark** · NVIDIA DGX Spark |
|---|---|---|
| Para quién | Un servicio pequeño, un centro de especialidades o un piloto | Un servicio grande o el hospital entero |
| Precio del equipo | Desde 899 $ (16 GB). **Recomendado 32 GB**, precio en € por confirmar ⚠️ | ~4.600–4.800 € con IVA (subió a 4.699 $ en febrero de 2026) |
| Memoria | 32 GB, 170 GB/s | 128 GB, 273 GB/s |
| Modelo de texto | Qwen3‑8B en Q4 (o Qwen3‑4B) | **Qwen3‑30B‑A3B** en FP8 (o Qwen3‑8B) |
| Transcripción | Whisper large‑v3‑turbo | Whisper large‑v3‑turbo |
| Lector de derivaciones | Modelos del BSC, en CPU y de noche | Igual |
| **Médicos a la vez con escriba + chat** | **~8–10** | **~30–50** |
| **Médicos a la vez solo con chat** | ~30–50 | ~400–500 |

Todos los modelos son de licencia libre (Apache‑2.0, MIT, CC‑BY‑4.0). **No se
paga cuota por médico ni por uso**, solo el equipo y la luz.

**Por qué el Qwen3‑30B‑A3B en el Spark:** es un modelo de mezcla de expertos. Tiene
30.000 millones de parámetros, pero en cada token solo trabajan 3.000 millones, así
que responde con la calidad de uno grande y a la velocidad de uno pequeño.

### Cuántos médicos pueden usarlo a la vez

**Peticiones a la vez no es lo mismo que médicos usándolo a la vez.** En una
consulta de 12 minutos, un médico hace unas 3 preguntas y ocupa la IA unos 45
segundos (~6 % del tiempo).

**Cada consulta pide a la IA:**

- Whisper: unos 10 minutos de audio. **Es el cuello de botella**.
- LLM: unos 2.500 tokens de entrada y 800 de salida, para la nota, los
  trámites, la vuelta a Primaria y la versión del paciente.

**Estimaciones a partir de benchmarks publicados:**

- **Spark:**
  - Con un 8B y 32 peticiones a la vez, unos 368 tok/s en total, es decir, unos
    11,5 tok/s cada una. Es más rápido de lo que se lee (LMSYS).
  - Whisper turbo: unos 20–30 s por cada 10 minutos de audio (estimado).
  - En total, **150–300 consultas por hora**.
- **M6:** unos 20 tok/s con una sola petición, y unos 40–60 s de Whisper por
  cada 10 minutos de audio. En total, **unas 40 consultas por hora**.
- **La nota está lista** unos 30 s después de la consulta en el Spark si se
  transcribe por trozos durante la consulta. Si no, alrededor de 1 minuto.

**La Princesa:**

- **Unos 558 médicos de plantilla**: 32 jefes de servicio, 49 jefes de sección
  y 477 facultativos especialistas, según la plantilla orgánica de abril de
  2024. Son plazas aprobadas; no incluye residentes ⚠️.
- **Más de 2.600 profesionales en total.**
- **Consultas:** 473.257 al año ÷ unos 250 días ≈ **1.900 al día**, unas 270 por
  hora por la mañana. Equivale a **unos 60 médicos en consulta a la vez**, el
  10–15 % de la plantilla.

| Montaje | Escriba + chat | Solo Jano (lector + chat + cierre con nota escrita a mano) |
|---|---|---|
| **1 Mac mini M6** | ~8–10 médicos: piloto pequeño | Un servicio entero |
| **1 Spark** (piloto en Digestivo) | ~30–50 médicos | **El hospital entero** |
| **3 Spark (~14.000 €)** | ~90–150 médicos: **todas las consultas externas** | Cualquiera de los ~558 |

⚠️ **Son estimaciones.** Antes del piloto hay que hacer una prueba de carga con el
modelo elegido, con el benchmark de vLLM o llama.cpp. Esa cifra es la que vale
para Informática.

⚠️ **El Spark es un equipo de escritorio:** no va en rack y no tiene fuente
redundante. Para producción, Informática quizá prefiera un servidor de rack con
GPU. El software es el mismo.

### Dónde se ejecuta el escriba

- **Los escribas comerciales** (Dragon Copilot, Abridge, Nabla…) mandan el audio
  de la consulta **a la nube**. El que licita la Consejería **también va en la nube
  pública** (ver más abajo). Del de Quirónsalud no sabemos dónde corre ⚠️.
- **El escriba de Jano** corre en el mismo equipo de Jano:
  1. el micrófono de la consulta envía el audio por la red interna;
  2. Whisper lo transcribe y Qwen3 redacta la nota;
  3. la nota se envía a HCIS como `MDM^T02`.

  **No sale nada del hospital.**
- **No se ejecuta en el PC de la consulta**, porque no tiene GPU.
- **Si el hospital ya tiene un escriba en la nube**, Jano recibe su nota por HL7 y
  hace igualmente los trámites.

### El escriba que compra la Comunidad de Madrid: en la nube y 1,74 M€

Es el expediente **A/SER‑000964/2026** de la Consejería de Digitalización
([BOCM 16‑mar‑2026](https://www.bocm.es/boletin/CM_Orden_BOCM/2026/03/16/BOCM-20260316-29.PDF),
[TED 154988‑2026](https://ted.europa.eu/es/notice/154988-2026/pdf)).

| | |
|---|---|
| Objeto | Transcripción de voz e informes estructurados en las consultas de los **centros de Atención Primaria**. Incluye la integración con APMadrid, automatizar trámites, indicadores, formación y soporte |
| **Dónde corre** | **En la nube pública.** El contratista tiene que suministrar «la infraestructura y las soluciones de **nube pública** necesarias» |
| **Valor estimado** | **1.441.600 €** sin IVA |
| **Presupuesto de licitación** | **1.744.336 €** con IVA |
| Duración | **3 meses**, del 1 de abril al 30 de junio de 2026, sin prórroga |
| Financiación | Fondos europeos NextGenerationEU (PRTR C11.I03.P14.S13) |
| Cómo se adjudica | **Precio, 70 puntos.** Solución técnica 14, planificación 8, transferencia 4, formación 4 |
| A quién se adjudicó | No consta en lo que hemos leído ⚠️ |
| Cuántos profesionales cubre | Está en el pliego técnico, que no hemos leído ⚠️ |

**Por qué es un punto fuerte para Jano:**

1. **Es nube, y Jano es local.** La Comunidad ha elegido que el audio de las
   consultas de Primaria vaya a la nube pública. Jano demuestra que se puede
   hacer **sin que salga nada del hospital**, que es lo que prefiere el director
   de La Princesa.
2. **No cubre el hospital.** El contrato es solo para Primaria. Las **consultas
   externas del hospital** se quedan fuera, y ese es el hueco de Jano.
3. **Se suman, no compiten.** Ese escriba automatiza las derivaciones en
   APMadrid, así que llegarán al hospital mejor redactadas. **Jano las lee
   mejor.** Primaria escribe y el hospital prepara la consulta.
4. **El coste de la nube no se acaba con el contrato.** Son 3 meses de
   implantación. Lo que cueste mantener la nube después no aparece en el anuncio
   ⚠️. Con Jano, el hardware se paga una sola vez.

**Cómo compararlo sin que nos lo tumben:**

- **No es comparable uno a uno.** Esos 1,74 M€ pagan integración, formación y
  soporte para toda la Atención Primaria de Madrid. Los 4.700 € de un Spark o
  los ~14.000 € de tres son **solo el equipo**.
- **La comparación honesta:** *«La Comunidad ha invertido 1,7 millones en un
  escriba en la nube para Primaria. Lo que falta es el hospital, y nosotros lo
  proponemos dentro del hospital, con un equipo que se compra una vez.»*
- **Nunca decir** «lo hacemos por el 1 % de lo que cuesta». Los servicios del
  piloto (la integración HL7 con Informática y la formación) también cuestan, y
  hay que estimarlos ⚠️.

### Cuánto cuesta un escriba comercial frente a Jano

**Los escribas comerciales se pagan por médico y por mes.** Casi ninguno publica
precios. Las cifras son de comparadores de terceros y del mercado de EE. UU. ⚠️:

| Producto | Precio por médico y mes |
|---|---|
| Heidi (plan Clinician, EE. UU.) | ~110 $ (publicado) |
| Nabla (Pro) | ~119 $ (estimado) |
| Abridge (empresas) | ~208 $ (estimado) |
| Ambience | ~233–267 $ (estimado) |
| Dragon Copilot (con la historia clínica integrada) | ~400–700 $, cifra antigua. Microsoft lo abarató en mayo de 2026 |
| **Rango típico para hospitales** | **~300–500 $** |

**En La Princesa,** suponiendo que **300 médicos** pasan consulta (de ~558 en
plantilla) ⚠️:

| | Al año |
|---|---|
| Escriba barato (110 $) | ~400.000 $ |
| Escriba de empresa (208 $) | ~750.000 $ |
| Escriba de gama alta (600 $) | ~2,2 M$ |
| **Jano entero**: 3 Spark amortizados en 4 años (~3.500 €), luz (~1.000 €) y una licencia anual de hospital (~50.000 €, hipótesis nuestra ⚠️) | **~55.000 €** |

- **Por médico:** unos **15 € al mes** con Jano, frente a **110–600 $** de un
  escriba.
- **Y Jano hace más que escribir:** lee las derivaciones, responde a preguntas
  sobre el historial y cierra la consulta.

**Con cuidado:**

- Los precios de los escribas son **estimaciones de terceros en EE. UU.**; en
  España se negocian por volumen.
- **Los escribas comerciales llevan años de producto y Jano es un piloto.** Si
  el jurado pregunta por la calidad, la respuesta es el mes en paralelo midiendo
  el acierto, no prometer que somos igual de buenos.
- **La integración HL7 y la formación del piloto** cuestan aparte y hay que
  estimarlas ⚠️.
- **La licencia de 50.000 € es una hipótesis** del modelo de negocio. Se paga con
  unas 700 revisiones evitadas, el 5 % del escenario del pitch.

### Aprende el vocabulario de los médicos

1. **Glosario por servicio (MVP).** Un fichero con siglas, fármacos y
   expresiones, por ejemplo *FEVI, HbA1c, EPOC*. Se usa en dos sitios:
   - en Whisper, con `hotwords` e `initial_prompt` de `faster-whisper`, o el
     prompt de `whisper.cpp`;
   - en las instrucciones del LLM.

   No hay que entrenar nada.
2. **Aprende de las correcciones.**
   - Cada corrección del médico se guarda («oyó X, era Y»).
   - Si se repite, entra en el glosario del servicio.
   - Un responsable del servicio la revisa.
   - **Aprende sin reentrenar ningún modelo**, y se puede auditar y deshacer.
3. **El estilo de cada médico.** El LLM recibe dos o tres notas anteriores suyas
   como ejemplo.
4. **Fase 2:** ajuste fino (LoRA) de Whisper con audio del hospital, con
   consentimiento y aprobación de protección de datos ⚠️.

**La frase:** *«Lo que Jano aprende se queda en el hospital.»* Con las
correcciones de vuestros médicos no se entrena el modelo de ninguna empresa.

### Sabe quién es cada médico

- **Por el login y la agenda, no por la voz.** El médico entra con su usuario o
  con el directorio del hospital, y la agenda dice quién pasa consulta en cada
  sala. En los CSV es `AGENDAS.profesional_ficticio`.
- **Para qué sirve:**
  - su glosario y su estilo de nota;
  - «Pregúntale a Jano» solo le enseña **sus pacientes del día**;
  - cada pregunta queda registrada con su nombre.
- **Separar las voces** (médico, paciente, acompañante) con
  `pyannote/speaker-diarization-3.1` (MIT) queda para la fase 2.
- **Reconocer al médico por la voz: no.** La huella de voz es un dato
  biométrico, especialmente protegido por el RGPD (art. 9), y el login ya
  resuelve lo mismo. **Con los pacientes, nunca.**

### Instalación para cualquiera

**El objetivo:** que se instale como un router. Se enchufa, se abre el
navegador y se siguen tres pasos. Nadie del hospital toca la línea de comandos.

**Cómo llega:**

- **Opción A (la buena): el equipo llega ya preparado.** Mac mini o Spark con
  Jano y los modelos ya instalados.
- **Opción B: el hospital ya tiene el equipo.** Se le pasa un instalador:
  - en el Mac, un `.pkg` con doble clic;
  - en el Spark, que trae DGX OS, basado en Ubuntu, un `.deb` o un
    `instalar.sh`.

  Los modelos van dentro del instalador, porque **no hace falta internet**
  (`HF_HUB_OFFLINE=1`) y llevan sumas de comprobación.

**El primer arranque, en `http://jano.local`:**

1. **Conectar con HCIS.** Informática escribe la IP y el puerto del motor de
   integración HL7. Hay un botón «Probar conexión» que se pone en verde.
2. **Elegir el servicio.** Se carga su protocolo y su glosario, con plantillas
   para Digestivo, Cardiología…
3. **Usuarios.** Se conecta al directorio del hospital o se crean a mano.

Al terminar, Jano se prueba solo con una derivación de ejemplo y enseña el
resultado.

**Qué hay debajo** (el instalador lo decide solo; nadie lo ve):

| | Mac mini M6 | DGX Spark |
|---|---|---|
| Modelo de texto | Ollama (Metal) | vLLM (contenedor de NVIDIA), que aguanta muchas peticiones a la vez |
| Transcripción | `whisper.cpp` (Metal) | `whisper.cpp` (CUDA) |
| Jano (API + web) | Servicio de `launchd`, arranca solo | Servicio de `systemd` o Docker, arranca solo |
| Perfil de modelos | Lo elige solo según la memoria: 16 GB → Qwen3‑4B, 32 GB → Qwen3‑8B | 128 GB → Qwen3‑30B‑A3B |

Jano habla con los dos motores por la misma API compatible con OpenAI, así que
**el código es el mismo en los dos equipos**.

**Para el día a día:**

- **Página de estado con semáforo:** verde, ámbar o rojo para la conexión con
  HCIS, los modelos, el disco y la última copia.
- **Actualizaciones** desde un fichero en un USB o desde un servidor interno, sin
  internet, con un botón para volver a la versión anterior.
- **Copia de seguridad con un botón**, del glosario, las correcciones y la
  configuración.
- **Si algo falla**, el mensaje dice qué hacer en lenguaje normal, no un error
  técnico.

### Qué entra en las 24 h

- **Que funcione igual en los dos equipos:** Jano usa la API compatible con
  OpenAI y el motor de IA se elige en un `.env`.
- **El asistente de los tres pasos** como pantallas diseñadas por UI/UX. Se puede
  enseñar en el pitch en 15 segundos.
- **Un `instalar.sh` que funcione** en el equipo de la demo.
- **Fuera:** el `.pkg` firmado, el `.deb`, las actualizaciones y la página de
  estado completa. Van a la hoja de ruta.

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
- [Comunidad de Madrid · Cita Sanitaria](https://www.comunidad.madrid/salud/cita-sanitaria) · [Libre elección](https://www.comunidad.madrid/salud/libre-eleccion-sanitaria)
- [Evaluación de la interconsulta virtual por Primaria](https://www.elsevier.es/es-revista-atencion-primaria-27-articulo-evaluacion-interconsulta-virtual-por-profesionales-S0212656723002512) · [Interconsulta digital en Cardiología (Univadis)](https://www.univadis.es/viewarticle/interconsulta-digital-resolver-m%C3%A1s-casos-sin-desplazar-2026a10010jk) · [e‑consulta en Neurología](https://www.elsevier.es/en-revista-neurology-perspectives-17-articulo-electronic-consultation-e-consultation-between-primary-S2667049623000017)
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
- Escribas con IA: [BOCM 16‑mar‑2026 · licitación del escriba de Primaria](https://www.bocm.es/boletin/CM_Orden_BOCM/2026/03/16/BOCM-20260316-29.PDF) · [Quirónsalud · Scribe, premio BiC IA (Infanta Elena)](https://www.quironsalud.com/es/comunicacion/actualidad/hospital-universitario-infanta-elena-premio-ia-bic-humaniza) · [Xataka · escribas en el Clínic y La Paz](https://www.xataka.com/medicina-y-salud/puede-resumir-50-folios-estudios-cinco-minutos-hablamos-hospitales-espanoles-que-estan-utilizando-ia)
- Hardware: [LMSYS · review del DGX Spark](https://www.lmsys.org/blog/2025-10-13-nvidia-dgx-spark) · [Foro NVIDIA · subida de precio del Spark](https://forums.developer.nvidia.com/t/dgx-spark-price-increase/361640) · [Kipmion · Spark Founders Edition EU](https://kipmion.com/producto/dgx-spark-founders-edition-eu) · [TidBITS · Mac mini M6](https://tidbits.com/2026/08/25/m6-and-m5-ultra-debut-in-updated-mac-mini-and-mac-studio/) · [PromptQuorum · Whisper en Apple Silicon](https://www.promptquorum.com/local-llms/apple-silicon-whisper-metal-benchmark)
- La Princesa: [plantilla orgánica (abr‑2024)](https://comunidad.madrid/sites/default/files/doc/sanidad/rrhh/rrhh-plantillas_organicas-estatutarios-h._la_princesa.pdf) · [70 aniversario, más de 2.600 profesionales](https://www.comunidad.madrid/file/498221/download)
- Lo que ya existe: [Infanta Elena · IA en las derivaciones de Digestivo (AEG 2021)](https://www.elsevier.es/es-revista-gastroenterologia-hepatologia-14-congresos-24-reunion-anual-asociacion-espanola-gastroenterologia-134-sesion-neurogastroenterologia-y-motilidad-6428-comunicacion-puede-la-inteligencia-artificial-mejorar-77422-pdf) · [Asturias · IA en dermatología con Legit.Health](https://elreferente.es/startups/asturias-incorpora-ia-en-atencion-primaria-con-legit-health/) · [Docman10X · lectura de cartas en Primaria (Reino Unido)](https://htworld.co.uk/?p=2297)
- Precios de escribas: [Microsoft · Dragon Copilot baja de precio (mayo 2026)](https://www.schneider.im/lu/microsoft-dragon-copilot-price-decrease-licensing-changes/) · [Commure · Nabla, precio y funciones](https://www.commure.com/blog-scribe/nabla-ai-review) · [Dupple · herramientas de documentación clínica 2026](https://dupple.com/learn/best-ai-clinical-documentation-tools) · [Plaud · Ambience](https://www.plaud.ai/blogs/articles/ambience-healthcare-review) · [Healee · cuentas de los escribas](https://blog.healee.com/the-review-tax-honest-math-on-ambient-ai-scribes-in-a-virtual-practice/)
