# Contexto del reto

Transcripción del material impreso que se entregó el 23‑oct‑2026 (fotos en
[`briefing/`](briefing/)) y de la información pública del evento.

## Estado en un minuto (24‑oct)

**Medici es el copiloto del médico en consultas externas:** el médico abre al
paciente y Medici le dice lo que no puede pasar por alto y le contesta lo que
pregunte, con la fuente de cada dato y con IA local. Nada sale del hospital.

| | Qué | Dónde |
|---|---|---|
| ✅ **Construido y medido** | **Chat sin agente:** 6 consultas SQL fijas del paciente y una sola llamada al modelo, que solo ordena y redacta citando filas. Con Qwen3‑8B en un portátil (RTX 5060, 8 GB): **29/30** con pacientes nuevos, 6/6 «no consta», 0 fuentes inventadas, 0,7 s por respuesta | [`mvp/`](../mvp/README.md) · [Cómo construirlo](#cómo-construirlo-propuesta-pendiente-del-equipo) |
| ✅ **Construido y medido** | **Tarjeta «3 cosas antes de que entre»:** alertas por reglas, sin IA, cada una con su fuente. Contrastadas con SQL directo en 2.000 pacientes: 0 errores | [La tarjeta](#la-tarjeta-3-cosas-antes-de-que-entre-construida-el-24oct) |
| ✅ **Construido** (Daniel) | Réplicas de HCIS y SELENE con dos historias sintéticas y un agente con respuestas **precalculadas** | [`demo/`](../demo/README.md) |
| 🟡 **Propuesto, pendiente del equipo** | Medici como **pestaña dentro de HCIS**; infraestructura (1 Mac mini M6 para el piloto, 2 × DGX Spark para todo el hospital); datos por HL7 | [Dentro de HCIS](#dentro-de-hcis-la-pestaña-medici-propuesta-pendiente-del-equipo) · [Infraestructura](#infraestructura-para-todo-el-hospital-propuesta-de-claude-pendiente-del-equipo) |
| ⬜ **Falta para la demo** | El servidor (FastAPI, 3 llamadas) y la pestaña con la tarjeta y el chat sobre `demo/web/hcis.html` | [El MVP en 24 h](#cómo-construirlo-propuesta-pendiente-del-equipo) |
| 📍 **Hoja de ruta** | Preparar la consulta antes, escriba, escribir en HCIS, texto libre de las historias… | [Hoja de ruta](#hoja-de-ruta-ideas-no-decididas) |

**Preguntar a los mentores hoy ⚠️:**

- Cuánto tarda un especialista en revisar la historia antes de cada paciente.
  Si se puede, cronometrarlo con la réplica de HCIS y con Medici.
- Si La Princesa ya tiene algo parecido.
- Cómo se abre HCIS: Chrome o Edge, modo IE o Java, Citrix.
- Si un chat que solo busca y muestra datos entra en el reglamento de productos
  sanitarios.

**Decidir en el equipo ⚠️:**

- Si la demo usa las tablas de los CSV (lo medido) o el texto libre de las
  historias de Daniel (precalculado).
- El precio de la licencia, antes de subir al escenario.

Las secciones de abajo guardan el recorrido completo. Lo que está escrito para
ideas anteriores lo dice al principio de cada sección.

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

### La idea decidida: Medici, el copiloto del médico

**Decidido por el equipo el 23‑oct por la noche:** *Medici es un chatbot copiloto.
El médico le pregunta en el ordenador por el paciente y Medici le contesta.*

**También decidido:**

- La IA es **local**: los datos no salen del hospital.
- Se vende para **Mac mini M6** o **NVIDIA DGX Spark**.
- Tiene que ser **muy fácil de instalar**.

**Nombre: Medici** (decidido el 23‑oct). Como los Medici de Florencia, que
impulsaron el Renacimiento. Y en italiano *medici* significa «médicos»: es
una herramienta para ellos.

> ⚠️ Ya existe una app estadounidense llamada **Medici** (Austin, Texas):
> mensajería entre pacientes y médicos, con unos 22–24 M$ de financiación
> entre 2018 y 2020. Para el hackathon no importa. Si Wayra quiere seguir
> adelante, hay que mirar la marca en la OEPM y la EUIPO antes de registrar
> nada.

**Qué hay de cada cosa** (para no vender lo que no existe):

| Estado | Qué |
|---|---|
| **Construido y medido** | En `mvp/`: el chat sin agente (29/30 con Qwen3‑8B) y la tarjeta «3 cosas antes de que entre» (0 errores en 2.000 pacientes). En `demo/` (Daniel): réplicas de HCIS y SELENE con historias sintéticas y respuestas precalculadas. En `analisis/`: los scripts de exploración (`lector_ia.py` es de la idea anterior y nunca se ha ejecutado) |
| **Propuesto por el equipo (24‑oct)** | MVP **sin agente**: SQL fijo y el modelo solo ordena. Medici como **pestaña dentro de HCIS**. La tarjeta «3 cosas» |
| **Decidido por el equipo** | Chatbot copiloto, IA local, Mac mini M6 o DGX Spark, instalación fácil |
| **Propuesto por Claude para construirlo** (falta que el equipo lo apruebe) | Todo lo de «Cómo construirlo», más abajo |
| **Hoja de ruta** (ideas, no decididas) | Lo de la sección [Hoja de ruta](#hoja-de-ruta-ideas-no-decididas) |

**El problema que ataca:** el tiempo que pasa el médico **buscando datos del
paciente** en la historia clínica.

- En EE. UU., cada consulta supone **16 minutos de historia clínica** y **un
  tercio, 5 min 22 s, es solo revisarla** (Overhage y McCallie, *Annals of
  Internal Medicine*, 2020; 100 millones de consultas). El estudio lo hizo
  Cerner, que vende historias clínicas.
- **En España no hemos encontrado una cifra** ⚠️. Hay que preguntar a los
  mentores:
  - cuántos sistemas abre un especialista de La Princesa antes de cada paciente
    (HCIS, HORUS, Primaria…);
  - cuánto tarda en hacerlo.
- La Princesa tiene **473.257 consultas externas al año** (memoria de 2022).

**¿Es innovador?** *Con honestidad: el chat con la historia clínica ya existe*
(Epic, Dragon Copilot de Microsoft, Navina en EE. UU.). Medici solo se puede
diferenciar si lo demuestra:

- **Local**, en un equipo de unos 4.700 €, sin nube y sin cuota por médico.
  Epic y Microsoft van en la nube.
- **Enseña de dónde sale cada dato** y dice «no consta» cuando no lo sabe.
- **Para la sanidad pública española:** en español, para HCIS y para consultas
  externas.
- **Se instala solo**, sin un proyecto de integración a medida.

### Cómo construirlo (propuesta, pendiente del equipo)

**Sin agente en el MVP** (propuesto por el equipo el 24‑oct). El servidor saca
con SQL fijo todo lo que hay del paciente y el modelo solo lo ordena y
contesta. Funciona porque cada paciente tiene pocas filas en los CSV: **de
mediana 7 y como mucho 37** entre todas las tablas.

```
El médico escribe: «¿Ha estado antes en Digestivo?»
      ↓
Servidor: 6 consultas SQL fijas, siempre del paciente abierto
  · ficha        [F1]  edad, sexo, contacto preferido, accesibilidad
  · derivaciones [D1…] motivo, observaciones del médico de cabecera, fecha
  · citas        [C1…] servicio, fecha, tipo de visita, estado, prioridad
  · cambios      [M1…] reprogramaciones, anulaciones, faltas y motivos
  · pruebas      [P1…] solicitadas, programadas, hechas, con resultado disponible
  · avisos       [A1…] mensajes enviados y si respondió
      ↓  filas numeradas + la pregunta, en UNA llamada
Modelo local (Qwen3‑8B; Qwen3‑4B si el equipo va justo)
      ↓  JSON: {consta, respuesta, fuentes: ["C3", …]}
Servidor: comprueba que cada fuente existe; sin fuentes válidas → «no consta»
      ↓
Respuesta en español + las filas en las que se basa, debajo
```

- **El modelo no consulta nada.** Solo recibe filas de un paciente que ya ha
  filtrado el servidor, así que no puede pedir datos de otro.
- **Menos cálculo y más fiable que un agente:** una llamada por pregunta, y un
  modelo pequeño no tiene que acertar qué consultar.
- **Lo que haya que contar o calcular** (cuántas faltas, hace cuánto) se
  calcula en SQL. El modelo solo lo redacta.
- **Botones para las preguntas habituales**, respondidos con SQL directo y sin
  IA. La IA queda para el resumen y las preguntas escritas a mano.
- **Cada pregunta queda registrada** con el médico, el paciente y las filas
  citadas.
- **El modelo funciona sin internet** y habla por una API compatible con
  OpenAI: el mismo código sirve para Ollama (M6) y vLLM (Spark).
- **El límite:** con historias reales de años de notas no cabría todo en una
  llamada. Habría que volver a elegir qué filas mandar (por fecha, por servicio
  o con búsqueda), o un agente. Es hoja de ruta.

**Construido y medido el 24‑oct:** [`mvp/`](../mvp/)

- `paciente.py`: las 6 consultas fijas.
- `responder.py`: la llamada al modelo y la comprobación de las fuentes.
- `prueba_acierto.py`: 30 preguntas con la respuesta correcta sacada de los
  CSV, también las trampas de «no consta». Contestando siempre «no consta» se
  sacaría 11/30: es el suelo.
- `docker-compose.yml`: Ollama con la GPU y la prueba, todo en Docker.

**Resultado con Qwen3‑8B (Q4_K_M) en un portátil con una RTX 5060 de 8 GB:**

| | |
|---|---|
| Acierto en 30 pacientes **que no se usaron para ajustar nada** (semilla 2026) | **29/30**. El fallo: dice que la cita se reprogramó, pero no da el motivo |
| «No consta» cuando toca (alergias, medicación, diagnóstico…) | **6/6** |
| Filas inventadas citadas | **0** |
| Tiempo por respuesta | **0,7 s** de mediana, 1,5 s como mucho |

- En las otras dos tandas (semillas 42 y 7) sale 30/30, pero **no cuentan**: con
  ellas se corrigieron la redacción de las filas y el corrector.
- **Lo que se arregló por el camino:**
  - Ollama 0.40 ignora `/no_think`, así que Qwen3 gastaba la respuesta
    razonando por dentro y la devolvía vacía. Ahora se pide con
    `reasoning_effort: "none"`.
  - El modelo confundía una cita «no realizada» con una «anulada». Se arregló
    escribiendo la fila más clara, sin tocar el modelo.
- **Son pocas preguntas y los datos son sintéticos.** Sirve para decir que
  funciona, no para dar un porcentaje de precisión clínica.
- Cómo se ejecuta, en [`mvp/README.md`](../mvp/README.md).

**Qué se puede preguntar con los CSV:**

| Pregunta | La responde |
|---|---|
| «¿Ha estado antes en este servicio?» | ✅ CITAS + AGENDAS |
| «¿Por qué lo manda su médico de cabecera?» | ✅ DERIVACIONES_AP |
| «¿Qué pruebas tiene hechas o pendientes?» | ✅ PRUEBAS |
| «¿Ha faltado o le hemos cambiado la cita? ¿Por qué?» | ✅ MOVIMIENTOS_CITAS |
| «¿Le llegaron los avisos?» | ✅ COMUNICACIONES |
| «¿Necesita intérprete o apoyo de accesibilidad?» | ✅ PACIENTES. Sirve para el pilar de **accesibilidad** |
| «¿Tiene alergias? ¿Qué diagnóstico tiene?» | ❌ No está en los CSV: tiene que responder **«no consta»**. Es la pregunta de la demo que enseña que, si no lo sabe, lo dice |

**El MVP en 24 h:**

1. Las 6 consultas fijas sobre los CSV con DuckDB. ✅ `mvp/paciente.py`.
2. La prueba de acierto con Ollama (`qwen3:8b`, y `qwen3:4b` para comparar) en
   el portátil de la demo. **La nota real es la que va al pitch.**
3. Un FastAPI con 3 llamadas: abrir sesión con un paciente, resumen y pregunta.
4. La pantalla: la **pestaña Medici sobre la réplica de HCIS** de Daniel
   (`demo/web/hcis.html`), con el resumen arriba, el chat y las filas de origen
   debajo de cada respuesta. Ver
   [Dentro de HCIS](#dentro-de-hcis-la-pestaña-medici-propuesta-pendiente-del-equipo).
5. **Por si falla en directo:** las respuestas de la demo, precalculadas, y un
   vídeo.

> ⚠️ **Decidir en equipo:** ¿Medici lee las tablas de los CSV (lo de arriba) o
> el texto libre de las historias de Daniel (`demo/`)? Con texto libre (18
> documentos con abreviaturas y contradicciones) un modelo pequeño acierta
> bastante menos. Además, las respuestas de `demo/web/datos.js` están
> precalculadas: no se pueden presentar como si las diera el modelo en directo.

**Fuera del MVP:**

- La conexión real con HCIS.
- El asistente de instalación (en el pitch van pantallas diseñadas, no
  funcionando).
- Todo lo de la hoja de ruta.

### La tarjeta «3 cosas antes de que entre» (construida el 24‑oct)

**Por qué:** el chat espera a que el médico pregunte. La tarjeta **se adelanta**
(como 5 de los 7 ganadores anteriores): al abrir al paciente le dice lo que no
puede pasar por alto. Además responde a «eso ya lo veo en HCIS»: HCIS te
enseña todo; Medici te dice lo que importa.

**Cómo:** reglas sobre los datos, **sin IA**, así que son siempre ciertas. Cada
alerta lleva las filas de las que sale. Se ordenan por importancia y se
enseñan las tres primeras.

| Alerta | Importancia | Ejemplo |
|---|---|---|
| Necesidad de accesibilidad | 90 | «Necesita intérprete.» |
| Citas cerradas sin atenderle | 80 si son 2 o más; 65 si es una | «2 citas se cerraron sin atenderle; la última, en Oftalmología el 16/05/2025.» |
| Prueba que no se llegó a hacer | 78 | «No se llegó a hacer: ecografía (programada el …).» |
| Pruebas pendientes | 75 | «Pruebas pendientes: ecografía (programada el 02/06/2025) y analítica (…).» |
| Derivación preferente o urgente | 72 | «Derivación preferente de Primaria a Neurología (08/05/2025): valoración especializada.» |
| Avisos que no le llegaron | 70 si son 2 o más; 55 si es uno | «1 aviso por SMS no le llegó (26/01/2025).» |
| Pruebas hechas con resultado | 60 | «Ya tiene hecha la analítica (…), con resultado disponible.» |
| Citas reprogramadas o anuladas | 50 si son 2 o más; 35 si es una | «1 cita anulada (paciente cancela).» |

Ejemplo de tarjeta completa (un paciente de los CSV sintéticos):

```
── antes de que entre ──
 • Necesita intérprete.                                                [F1]
 • Prueba pendiente: endoscopia (programada el 13/05/2025).            [P1]
 • 1 cita se cerró sin atenderle (Oftalmología, 11/02/2025).          [M1]
```

**Prueba** (`mvp/prueba_alertas.py`, 2.000 pacientes al azar):

- Cada alerta se contrasta con una consulta SQL distinta de la que la generó,
  directamente sobre las tablas. **No cuadró ninguna mal** (0 de 2.000).
- **Qué sale:**
  - A un 35 % de los pacientes no les sale nada. Ver «Nada que destacar» también
    es útil.
  - Un 39 % tiene una cosa, un 19 % dos, y un 6,5 % tres o más.
- **Lo más frecuente:**

  | Alerta | Pacientes |
  |---|---|
  | Citas cerradas sin atenderle | 23 % |
  | Citas cambiadas | 23 % |
  | Accesibilidad | 18 % |
  | Avisos que no llegaron | 13 % |
  | Pruebas pendientes | 7 % |

- `python mvp/alertas.py --buscar-demo` encuentra pacientes con una tarjeta
  completa de tres cosas distintas, para elegir a «María».

**Lo que falta:** enseñarla en la pestaña de la réplica de HCIS, arriba del
chat. Los valores de importancia son una propuesta: que los revise la
biomédica con los mentores ⚠️.

### El equipo (propuesta)

| Quién | Papel |
|---|---|
| Dev 1 (software y robótica) | **El servidor:** FastAPI con las 3 llamadas sobre `mvp/` (sesión, tarjeta y resumen, pregunta) y Ollama. Cuenta la viabilidad en el pitch |
| Dev 2 (software) | **La pestaña** sobre `demo/web/hcis.html`: la tarjeta arriba, el chat y las fuentes. Lleva el portátil de la demo |
| UI/UX | **Cómo se ve una respuesta con su fuente** en 5 segundos. Las pantallas de la instalación en tres pasos, las diapositivas y **presentar la demo** |
| Biomédica | **Las preguntas reales** que hace un especialista antes de ver a un paciente, validadas con los mentores (cambiarlas en `mvp/prueba_acierto.py`). El orden de las alertas de la tarjeta, lo regulatorio y las cifras. **Abre el pitch** |

El reparto lo decide el equipo.

### Reparto del tiempo (orientativo)

| Bloque | Devs | UI/UX | Biomédica |
|---|---|---|---|
| 1 | ✅ Modelo servido, 6 consultas fijas, prueba de acierto y tarjeta | Boceto: pestaña con tarjeta, chat y fuentes | Mentores: ¿qué preguntan los especialistas? ¿Cuántos sistemas abren? ¿La Princesa tiene ya algo así? |
| 2 | Servidor (FastAPI) y pestaña sobre la réplica de HCIS con datos reales | Diseño final y pantallas de instalación | Las 30 preguntas con su respuesta correcta |
| 3 | Elegir a «María» (`alertas.py --buscar-demo`). Precalcular la demo | Diapositivas | Cifras y guion |
| 4 | Congelar el código. Grabar un vídeo de la demo | Ensayar | Ensayar |

## Hoja de ruta (ideas, no decididas)

Son ideas que salieron el 23‑oct y **no forman parte de Medici** mientras el
equipo no lo decida. En el pitch, como mucho, van en una diapositiva de «lo
siguiente».

- **Preparar la consulta antes:** leer las derivaciones al llegar y proponer las
  pruebas del protocolo, para que estén hechas en la primera consulta. Hay
  evidencia: se resuelve en una visita el 49 % frente al 22 %, y el Infanta
  Elena bajó de 2,25 a 1,72 revisiones por primera. El diseño completo está en
  git (commit `8146b89`).
- **Ficha previa** de cada paciente de la agenda del día.
- **Cerrar la consulta:** los trámites, la vuelta a Primaria y la versión para el
  paciente. También conectarse a un escriba.
- **El paciente:** instrucciones en lectura fácil, en su idioma y en audio.
- **El glosario de cada servicio**, que aprende de las correcciones.
- **Escribir en HCIS:** citar o pedir pruebas.
- Las del cuadro de usos de la IA local que sigue.

### Modelos de Hugging Face

**Para el chat del MVP basta el generativo** (Qwen3‑8B o 4B, que saben llamar a
funciones). Los del BSC son para la hoja de ruta (leer derivaciones).

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
Si no hay presupuesto para GPU, **Medici funciona sin esta capa**.

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

### Más usos de la IA local (hoja de ruta)

Ideas para sumar a Medici, ordenadas de más a menos recomendable para estas 24
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

### Aparcado: los huecos vacíos

El diseño anterior era un circuito automático contra las faltas: recordatorio
con respuesta, citas dobles, segundo aviso y reofrecer el hueco, con una bandeja
solo de excepciones. Se aparca porque media sala va a ir a por eso. Está en el
historial de git (commit `8726248`) y sus cifras reales siguen en
[`HALLAZGOS.md`](HALLAZGOS.md#cifras-reales-para-el-pitch). Si sobra tiempo, el
enlace al paciente para elegir hora es la misma pieza.

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

**Lo que significa para Medici:**

- **Entre la cita y la primera consulta pasan semanas.** Esa espera es el hueco
  para hacer las pruebas antes.
- **Medici lee el parte de interconsulta cuando la cita llega al hospital.** Hay
  que preguntar a Informática cómo llega: si electrónico, por HORUS o en papel ⚠️.
- **Con la libre elección, cada hospital solo ve a quien lo ha elegido**, y las
  pruebas se hacen allí mismo. Encaja: Medici funciona en el hospital que recibe.
- **La vuelta directa tras perder una revisión depende solo del hospital**, que
  es quien gestiona las revisiones. No hay que cambiar nada en Primaria.

## Lo que ya existe y qué aporta Medici

> Escrito para la idea anterior (preparar la consulta y adelantar pruebas). Para
> el chatbot, la competencia directa es Epic, Dragon Copilot y Navina (ver
> [La idea decidida](#la-idea-decidida-medici-el-copiloto-del-médico)).

**Comprobado el 23‑oct.** Ninguna pieza de Medici es nueva por separado. Lo que no
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
  Medici.
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
2. **Su problema fue el alcance**, y Medici lo resuelve: lee **el 100 %** de las
   derivaciones al llegar al hospital, sin que nadie cambie su forma de derivar.
3. **Era de 2018, de un servicio y de un hospital.** Medici usa IA de 2026, cubre
   todo el circuito (antes, durante y después), se conecta por HL7 y se instala
   en cualquier hospital.
4. **El director de La Princesa puede conocerlo.** Mejor citarlo nosotros que
   que nos lo saquen.

### El resto del mercado

| Pieza | ¿Existe? | Quién | Qué aporta Medici |
|---|---|---|---|
| Leer derivaciones y adelantar pruebas | Sí, en un servicio | Infanta Elena (2018); en Asturias, IA para priorizar dermatología en Primaria (Legit.Health) | El 100 % de las derivaciones, varios servicios, local e instalable |
| Leer cartas y avisar solo de lo que requiere acción | Sí, en Primaria (Reino Unido) | Docman10X (Advanced + U. de Manchester) | Lo mismo, pero del lado del hospital |
| Escriba en consulta | Sí, mucho | Dragon Copilot, Abridge, Nabla, Scribe de Quirón, el contrato de Madrid (nube) | Local, y conectado a lo que pasa antes y después |
| Chat con la historia clínica | Sí | Epic, Dragon Copilot | Local, con la fuente de cada dato y limitado a los pacientes del médico |
| Alta resolución, e‑consulta | Sí, hechas a mano | Muchos hospitales | Automatizarlo sin añadir personal |
| **Todo junto, local, por HL7 y fácil de instalar** | **No lo hemos encontrado** | — | **Medici** |

**Lo primero mañana con los mentores ⚠️:**

- ¿La Princesa tiene ya algo que lea o prepare las derivaciones?
- ¿Conocen lo del Infanta Elena? ¿Sigue funcionando?

Si La Princesa ya lo tiene, hay que saberlo antes del pitch, no durante.

## HORUS: el visor que junta la historia del paciente

**Qué es:** el visor web de historia clínica del SERMAS, para profesionales.
Sirve para ver la información de un paciente **de cualquier centro de la
Comunidad**, sin importar dónde esté guardada.

- **Qué enseña:**
  - la historia de Primaria (AP‑Madrid);
  - los informes de los distintos especialistas y hospitales;
  - las citas pendientes y las solicitudes de interconsulta;
  - según una fuente de 2013, también episodios, diagnósticos, medicación y
    analíticas.
- **Cómo funciona:** **no guarda una copia.** Se conecta en línea a los sistemas
  donde está cada historia (AP‑Madrid, el HCIS de cada hospital…) y lo muestra
  junto, sin duplicar datos. Identifica al paciente por su código regional.
- **No es** Mi Carpeta de Salud, que es el portal del paciente.
- Las fuentes son oficiales pero algunas son antiguas: hay que confirmar con los
  mentores qué enseña hoy ⚠️.

**Qué significa para Medici:**

- **HORUS es justo lo que el médico tiene que leer.** Junta todos los informes,
  pero **hay que abrirlos y leerlos uno a uno**. Son los minutos que Medici quiere
  ahorrar.
- **La frase:** *«HORUS te enseña todos los papeles del paciente; Medici te los
  lee y te contesta.»*
- **Sería la mejor fuente de datos para Medici**, porque ya junta Primaria y
  hospitales. Pero es de la Consejería, no del hospital: conectarse a él
  requiere el permiso de la Dirección General de Salud Digital, y eso es más
  lento que hacerlo con Informática de La Princesa.
- **Propuesta:** en el piloto, solo los datos del propio hospital (HCIS). HORUS,
  en la hoja de ruta.
- **Preguntas para los mentores ⚠️:**
  - ¿Los especialistas abren HORUS antes de cada paciente?
  - ¿Cuánto tardan?
  - ¿Qué buscan?

## Cómo se conecta Medici al hospital: HCIS y HL7

> **Propuesta, pendiente de Informática.** El chatbot necesita leer los datos del
> paciente; esta es la forma que proponemos. No está construida.

**Dato del equipo (23‑oct):** la historia clínica de La Princesa es **HCIS**, y
se comunica con los demás sistemas mediante **HL7**.

**Medici no toca la base de datos de HCIS.** Se conecta como cualquier otro
sistema del hospital: recibe los mensajes HL7 que HCIS ya envía, por el motor de
integración del hospital, y guarda su propia copia de solo lectura. Si un día
cambia HCIS, Medici sigue funcionando.

| Tabla de los CSV | Mensaje HL7 v2 que la alimentaría |
|---|---|
| PACIENTES | `ADT^A04` / `A08` (alta y cambios del paciente, segmento `PID`) |
| DERIVACIONES_AP | `REF^I12` (derivación), o como la envíe Primaria ⚠️ |
| CITAS y MOVIMIENTOS_CITAS | `SIU^S12` nueva, `S13` reprogramada, `S14` modificada, `S15` anulada, `S26` no acudió |
| PRUEBAS | `OMG^O19` (petición) y `ORU^R01` (resultado) |
| Nota de la consulta | `MDM^T02` (documento clínico) |
| COMUNICACIONES | No suele ir por HL7: es la plataforma de SMS |

**Para escribir en HCIS** (citar, pedir una prueba) hace falta que HCIS acepte
peticiones (`SRM^S01`, `OMG^O19` o FHIR). En el piloto, Medici solo lee y
propone, y la persona confirma en HCIS. Escribir directamente es la fase 2 ⚠️.

**Hoy no se usa HL7 en ningún sitio.** El MVP lee los CSV con DuckDB y consulta
su copia con SQL. HL7 no es un lenguaje de base de datos: son los mensajes que
se mandan los sistemas del hospital. Medici los recibiría y los guardaría en su
copia, y luego seguiría consultando esa copia con SQL.

**Si sobra tiempo en la demo (no está hecho):** un script que convierta los CSV
en mensajes HL7 v2 y se los envíe a Medici por MLLP, como haría el motor de
integración. Así el jurado vería que se conecta sin tocar HCIS. Librerías:
`hl7apy` o `hl7` (MIT).

**Preguntas para Informática ⚠️:**

- Qué versión de HL7 usan (2.3, 2.5) y qué motor de integración tienen.
- Qué mensajes publica ya HCIS (citas, pruebas, documentos).
- Si hay FHIR.
- Cómo llega la derivación de Primaria.

### La pieza innovadora: el escriba escribe, Medici actúa

> **Hoja de ruta, no decidido.** El chatbot no incluye nada del escriba.

La Consejería ya está comprando un escriba con IA para Primaria, y Quirónsalud
tiene «Scribe» en el Infanta Elena ([fuentes](#fuentes)). **No competimos con
ellos: nos conectamos a ellos.** Medici recibe la nota de cualquier escriba (HL7
`MDM^T02` o FHIR `DocumentReference`), o la que el especialista escribe a mano,
y la convierte en acciones.

| Momento | Qué hace Medici | De dónde lo saca |
|---|---|---|
| **Antes** | Ficha de la derivación y pruebas adelantadas | Derivación (`REF`) |
| **Durante** | **Pregúntale a Medici** (construido): consultas fijas de solo lectura sobre el historial del paciente y el modelo solo ordena. Siempre enseña las filas en las que se basa, y si un dato no está responde «no consta» | Copia HL7 |
| **Durante** | **Aviso de prueba duplicada**: el médico dice «le pido una analítica» y Medici avisa de que ya tiene una de hace tres semanas | Nota del escriba + `ORU` |
| **Al cerrar** | **Cero clics después de la consulta**: de la nota saca la lista de trámites (pruebas, próxima cita, vuelta a Primaria). El especialista la confirma con un clic | Nota del escriba (`MDM`) |
| **Después** | La vuelta a Primaria redactada, y la versión del paciente en lectura fácil, en su idioma y en audio | Modelos locales |

**La frase del pitch:** *«El escriba escribe la consulta. Medici hace todo lo que
viene antes y después.»*

**Por qué es innovador:**

- Es una capa **local y abierta** sobre los estándares que ya tiene el
  hospital (HL7).
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

## Infraestructura para todo el hospital (propuesta de Claude, pendiente del equipo)

**Lo que se pide:** conectar a todo el hospital a Medici, con un Spark (o
similar) o con Mac mini M6. Solo el chatbot, que es lo decidido.

### Lo que no cambia en ninguna de las dos vías

- **Los médicos no instalan nada.** Abren `https://medici` en el navegador del
  ordenador de la consulta, con su usuario del hospital.
- **Todo dentro de la red del hospital, en el CPD.** Nada en las consultas y
  sin salida a internet.
- **Los equipos de IA no guardan datos de pacientes.** Reciben la pregunta y
  los datos de ese momento, contestan y lo olvidan. Los datos y el registro de
  preguntas están en un único sitio.
- **El mismo software** para M6 y para Spark. Solo cambia el motor de IA (Ollama
  en el Mac, vLLM en el Spark), detrás de la misma API compatible con OpenAI.

```
 Ordenadores de las consultas (navegador)
            │  HTTPS dentro de la red del hospital, usuario del Directorio Activo
            ▼
 ┌──────────────── CPD del hospital, red aislada (VLAN), sin internet ────────────────┐
 │                                                                                    │
 │  Medici · aplicación         ← proxy HTTPS (Caddy) + API (FastAPI) + la web          │
 │     │   · consultas fijas de solo lectura, siempre de UN paciente                  │
 │     │   · registro: quién preguntó qué y cuándo                                    │
 │     │                                                                              │
 │     ├──► Datos de Medici (PostgreSQL): copia de solo lectura con lo justo           │
 │     │        ▲  citas, derivaciones, pruebas, avisos, accesibilidad                │
 │     │        └── motor de integración del hospital → mensajes HL7 desde HCIS ⚠️    │
 │     │            (o una exportación nocturna si es más fácil al principio)         │
 │     │                                                                              │
 │     └──► Reparto de peticiones (LiteLLM, MIT) ──► Equipos de IA                    │
 │                                                  · Vía A: 2 × DGX Spark            │
 │                                                  · Vía B: 3 × Mac mini M6          │
 └────────────────────────────────────────────────────────────────────────────────────┘
```

### Cuánto hay que aguantar en La Princesa

- **~558 médicos de plantilla**, y **~60 en consulta a la vez**.
- **Con el chat, cada médico ocupa la IA unos segundos por pregunta**, ~6 % del
  tiempo de la consulta. Con 60 médicos son **unas 4–8 preguntas a la vez en la
  punta** de la mañana.
- **Lo que aguanta cada equipo** (estimado ⚠️):
  - **un Spark:** unas **30** preguntas a la vez;
  - **un M6:** unas **2–4**.

### Vía A: centralizado con DGX Spark (o similar)

| | |
|---|---|
| Equipos | **2 × DGX Spark** en el CPD: uno trabaja y el otro está de reserva, o los dos repartiendo trabajo |
| Modelo | **Qwen3‑30B‑A3B** (Apache‑2.0). Cabe sobrado en 128 GB y deja más margen que un 8B para historias largas y texto libre (sin medir todavía con nuestra prueba ⚠️) |
| Capacidad | Con uno basta para todo el hospital (~30 preguntas a la vez). El segundo es por si falla el primero |
| Coste del equipo | **~9.400 €** (2 × ~4.700 € con IVA) |
| A favor | Un solo sitio que mantener, actualizar y auditar. Las respuestas más rápidas y el mejor modelo. Hay margen para crecer (el escriba, la hoja de ruta) |
| En contra | El Spark es un equipo de escritorio: no va en rack y no tiene fuente redundante. Va en una bandeja del rack, y la redundancia la da el segundo equipo |
| **«O similar»** | Para producción, Informática quizá prefiera un **servidor de rack con una GPU profesional** (fuente redundante, gestión remota). El software es el mismo; el precio, a pedir al proveedor ⚠️ |

### Vía B: Mac mini M6

| | |
|---|---|
| Equipos | **3 × Mac mini M6** con 32 GB en el CPD: 2 trabajando y 1 de reserva |
| Modelo | **Qwen3‑8B** en Q4. El de 30B no cabe con holgura en 32 GB |
| Capacidad | 2–4 preguntas a la vez cada uno. Dos cubren la punta (4–8) **por poco** |
| Coste del equipo | **Más barato**: 3 × M6 con 32 GB, precio en € por confirmar ⚠️ (desde 899 $ el de 16 GB) |
| A favor | Barato, silencioso, consume poco. Muy fácil para empezar: **un M6 basta para un piloto** de un servicio |
| En contra | **Un modelo más pequeño.** Con nuestros datos basta (Qwen3‑8B: 29/30), pero deja menos margen para historias largas. Más lento en la punta. Más equipos que mantener. No hay sitio para crecer (escriba) |
| Alternativa | El **Mac mini M5 Pro** (hasta 64 GB, 307 GB/s), con más margen por equipo |

**Lo que no recomiendo: un M6 en cada servicio o en cada planta.** Cada equipo
tendría una copia de datos de pacientes fuera del CPD (riesgo de robo y peor
para la protección de datos), habría que actualizar y vigilar N equipos, y el
día que falla uno, ese servicio se queda sin Medici. Los M6 siempre en el CPD
y repartiéndose el trabajo.

### Comparación

| | **A · 2 × Spark** | **B · 3 × M6** |
|---|---|---|
| Coste del equipo | ~9.400 € | Menos (por confirmar) |
| Calidad de las respuestas | **Mejor** (modelo de 30B) | Buena (8B) |
| Velocidad en la punta | **Sobrada** | Justa |
| Aguanta que se rompa uno | Sí | Sí |
| Crecer (más usos, escriba) | **Sí** | No |
| Piloto pequeño | Vale, pero sobra | **Ideal** |

### Recomendación

**Un camino que use las dos:**

1. **Piloto (un servicio, 3 meses):** **1 Mac mini M6** en el CPD. Cuesta muy
   poco, demuestra que funciona y que se instala solo. Si se quiere probar ya
   con el mejor modelo, 1 Spark.
2. **Todo el hospital:** **2 × DGX Spark** con el modelo de 30B. El M6 del
   piloto queda **de reserva o para pruebas**.
3. **Hospitales pequeños o centros de especialidades:** Medici con M6.

**Para vender:** *«Medici empieza con un Mac mini y crece a un Spark sin cambiar
nada, porque el software es el mismo.»*

### Lo que hace falta del hospital

| Qué | Quién | Nota |
|---|---|---|
| Un hueco en el CPD, red y una VLAN | Informática | |
| Acceso a los datos: mensajes HL7 desde el motor de integración, o una exportación nocturna para empezar | Informática | ⚠️ Es lo que más tarda |
| Usuarios del Directorio Activo y grupos por servicio | Informática | Cada médico ve solo a sus pacientes |
| Certificado HTTPS de la autoridad interna | Informática | |
| Copias de seguridad de la base de Medici en su sistema | Informática | |
| **Evaluación de impacto de protección de datos (EIPD)** | Delegado de protección de datos | ⚠️ Obligatoria al tratar datos de salud con un sistema nuevo |
| **Esquema Nacional de Seguridad (ENS)** | Seguridad | ⚠️ Los sistemas del sector público lo tienen que cumplir. Lo ayuda que no tenga internet, que use TLS, que tenga registro y que dé el mínimo acceso |
| Si entra en el reglamento de productos sanitarios o en el de IA | Mentores / regulatorio | ⚠️ Solo busca y muestra datos: probablemente no, pero hay que confirmarlo |

### Tecnologías (propuesta)

Todo es software libre con licencias que permiten venderlo.

| Capa | Producción | MVP de 24 h | Licencia |
|---|---|---|---|
| **Pantalla** | React + Vite + TypeScript. Chat con respuesta en streaming (SSE) | La misma, o lo que mejor domine el Dev 2 | MIT |
| **API** | Python 3.12 + FastAPI + Pydantic. Sin agente: consultas fijas y una llamada al modelo por la API compatible con OpenAI. Sin frameworks pesados | Igual (`mvp/`) | MIT |
| **Consultas** | 6 consultas SQL fijas de solo lectura, con el paciente siempre fijado por el servidor | Las mismas sobre DuckDB (`mvp/paciente.py`) | — |
| **Datos** | PostgreSQL: copia con lo justo y registro de preguntas | **DuckDB sobre los CSV** (ya está `analisis/datos.py`) | PostgreSQL / MIT |
| **Entrada de datos** | Receptor HL7 por MLLP con `hl7apy`, desde el motor de integración. O una exportación nocturna | No hace falta: se usan los CSV | MIT |
| **Modelo** | Spark: **Qwen3‑30B‑A3B** en FP8. M6: **Qwen3‑8B** en Q4 | Qwen3‑8B o 4B, según el equipo de la demo | Apache‑2.0 |
| **Servidor del modelo** | Spark: **vLLM**. M6: **Ollama** (o `llama.cpp`) | Ollama o `llama.cpp` | Apache‑2.0 / MIT |
| **Reparto entre equipos** | **LiteLLM** como proxy compatible con OpenAI | No hace falta | MIT |
| **HTTPS y entrada** | **Caddy**, con el certificado del hospital | No hace falta | Apache‑2.0 |
| **Usuarios** | Directorio Activo del hospital por LDAP (`ldap3`) | Selector de médico (de AGENDAS) | MIT |
| **Empaquetado** | Spark (DGX OS, que es Ubuntu): **Docker Compose**. M6: Ollama nativo (Docker en el Mac no usa la GPU) y la app como servicio de `launchd` | Un `instalar.sh` o `docker compose up` | — |
| **Calidad** | `pytest` y **30 preguntas con su respuesta correcta**, que se repiten en cada cambio | Las 30 preguntas | MIT |
| **Vigilancia** | Página de estado con semáforo y registros. Prometheus y Grafana si Informática los usa | Nada | Apache‑2.0 / AGPL |

**Por qué no meter más cosas:**

- **Sin LangChain ni agentes.** Con pocas filas por paciente basta con
  mandarlas todas en una llamada: se entiende, se audita y falla menos.
- **Sin base de datos vectorial ni RAG.** Los datos son tablas (citas, pruebas,
  derivaciones), así que se consultan con SQL. Buscar por parecido solo haría
  falta para informes en texto libre, que es la hoja de ruta.

### Cómo encaja todo

**1. Instalarlo (una vez, Informática):**

1. Se pone el equipo en el CPD, se conecta a la red y se abre
   `https://medici.local`.
2. El asistente pide tres cosas:
   - **de dónde salen los datos:** la dirección del motor HL7 o la carpeta de
     la exportación;
   - **el Directorio Activo;**
   - **los servicios** que van a usarlo.
3. Medici se prueba solo con un paciente de ejemplo y se pone en verde.

**2. Los datos se mantienen al día (solo, todo el día):**

- HCIS envía un mensaje por cada cita, cambio, prueba o derivación. El receptor
  de Medici lo guarda en su copia.
- Si al principio solo hay una exportación nocturna, se carga cada noche.

**3. El médico lo usa (cada día):**

1. Abre `https://medici` en el navegador de su consulta y entra con su usuario
   de siempre.
2. Ve **su agenda del día** y elige al paciente. O lo abre en HCIS como
   siempre y la pestaña Medici aparece con él (ver «Dentro de HCIS») ⚠️.
3. **Pregunta:** «¿Ha estado antes en Digestivo?»

**4. Lo que pasa por dentro (en 2–5 segundos):**

```
pregunta ─► API: comprueba que el paciente es de este médico
          ─► API: 6 consultas SQL fijas de ESE paciente, filas numeradas
          ─► modelo: una llamada; ordena y redacta citando [C3], [P1]…
          ─► API: comprueba que cada fila citada existe
          ─► pantalla: respuesta + filas de origen debajo
          ─► registro: médico, paciente, pregunta, filas citadas, hora
```

- Si no hay ninguna fila citada que exista, la respuesta es **«no consta en
  los datos disponibles»**.
- **El paciente lo fija el servidor y el modelo no consulta nada.** Aunque se
  equivoque, no puede ver datos de otro paciente.

**5. Mantenerlo:**

- La página de estado avisa si los datos llevan horas sin llegar o si un equipo
  de IA no responde.
- Las actualizaciones llegan como un fichero y se pueden deshacer.
- Las 30 preguntas de prueba se repiten antes de cada actualización. Si baja el
  acierto, no se instala.

### Dentro de HCIS: la pestaña Medici (propuesta, pendiente del equipo)

**La idea** (del equipo, 24‑oct): el médico no abre otra aplicación. Busca al
paciente en HCIS como siempre y en el borde de la ventana aparece una pestaña
«Medici» con ese paciente: el resumen arriba y el chat debajo. Los esquemas
están en el lienzo [Arquitectura Medici](https://claude.ai/artifact/TW3bnQYWKApTGZmtk9sB6d),
que es privado: hay que compartirlo desde la página.

**Dos caminos separados:**

- **Los datos** llegan por detrás: HCIS → motor de integración (HL7) o
  exportación nocturna → copia de Medici. No pasan por la pantalla, así que da
  igual cómo sea HCIS.
- **Qué paciente está abierto:** es lo único que se saca de la pantalla, y solo
  el NHC.

**Cómo es HCIS:** hoy es de **Dedalus**, que compró en 2021 la parte de
software sanitario de DXC. Su folleto dice que es *«un sistema multicapa basado
en web»* sobre J2EE. Lo que no sabemos es **cómo lo abren los médicos de La
Princesa**: en una pestaña normal del navegador, en modo Internet Explorer, en
Java o por Citrix ⚠️. No hay demo ni manual público.

**Cuatro formas de saber qué paciente está abierto:**

| Forma | Cuándo sirve | Cómo lee el NHC | Instalación | Riesgo |
|---|---|---|---|---|
| 1. Extensión de Chrome o Edge | HCIS en una pestaña normal | Lee la cinta del paciente en la página | Informática la reparte con una directiva (`ExtensionInstallForcelist`) | Si cambia la pantalla, se ajusta el selector |
| 2. Programa pequeño de Windows | Java, modo IE o programa nativo | API de accesibilidad de Windows (UI Automation); en Java, Java Access Bridge | Instalador con la herramienta de Informática | Si la ventana no expone el texto, no se puede ⚠️ |
| 3. Botón dentro de HCIS | Si HCIS admite enlaces externos | HCIS abre `https://medici/abrir?nhc=…` y Medici lo comprueba | Configuración en HCIS | Depende de Informática o de Dedalus ⚠️ |
| 4. Elegirlo en la agenda de Medici | Siempre | No lo lee: el médico pulsa al paciente en su agenda del día | Ninguna | Un clic más; el nombre va grande arriba |

- **Si HCIS va por Citrix**, la 2 tendría que publicarse dentro de Citrix. Si
  no se puede, la 3 o la 4.
- **Propuesta:** la 4 siempre, como plan B, más la mejor de las tres primeras
  según cómo sea HCIS.

**Las llamadas**, iguales en las cuatro formas:

1. `POST /api/sesion {nhc}`, con el usuario del médico. El servidor comprueba
   que puede ver a ese paciente y ata la sesión a él: el NHC ya no se vuelve a
   mandar.
2. `GET /api/sesion/{id}/resumen`. Si el paciente está en la agenda del día, se
   precalcula a primera hora.
3. `POST /api/sesion/{id}/pregunta {texto}`: SQL fijo y una llamada al modelo.
   Vuelve en streaming, con las fuentes.

Si el médico cambia de paciente en HCIS, la pestaña borra el chat y vuelve al
paso 1, para que nunca se mezclen dos pacientes.

**Para la demo:** la réplica `demo/web/hcis.html` hace de HCIS. Un script lee
el NHC de su cinta, como haría la extensión, y el panel de `agente.js` pasa a
ser una pestaña lateral. Frase para el pitch: *«El médico no abre nada nuevo.
Busca al paciente en HCIS como siempre, y Medici ya está ahí.»*

**Para preguntar a un mentor de La Princesa ⚠️:** ¿HCIS se abre en Chrome o
Edge con barra de direcciones? ¿Sale el icono de Internet Explorer o una
ventana de Java? ¿Se entra por Citrix?

### Mantenimiento

- **Página de estado** con semáforo: datos al día, equipos de IA y última copia.
- **Actualizaciones sin internet**, con un fichero desde un servidor interno o un
  USB, y vuelta atrás con un botón.
- **Lo mantiene la Unidad de Análisis de Datos o Informática.** No hace falta un
  equipo nuevo.

## Dónde corre Medici: Mac mini M6 o NVIDIA DGX Spark

> **Para el chatbot vale la columna «solo chat»:** un Spark aguanta unos
> 400–500 médicos conectados y un M6 unos 30–50 (estimado ⚠️). Las cifras «con
> escriba» y las subsecciones sobre escribas son contexto y hoja de ruta.

**Decidido el 23‑oct:** Medici se vende para **dos equipos**, el **Mac mini M6**
y el **NVIDIA DGX Spark**. Tiene que instalarlo **cualquiera, sin saber
informática**.

### Las dos versiones

| | **Medici Mini** · Mac mini M6 | **Medici Spark** · NVIDIA DGX Spark |
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

| Montaje | Escriba + chat | Solo Medici (lector + chat + cierre con nota escrita a mano) |
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
  de la consulta **a la nube**. El de la Consejería (T‑Systems) **también va en la nube
  pública** (ver más abajo). Del de Quirónsalud no sabemos dónde corre ⚠️.
- **El escriba de Medici** corre en el mismo equipo de Medici:
  1. el micrófono de la consulta envía el audio por la red interna;
  2. Whisper lo transcribe y Qwen3 redacta la nota;
  3. la nota se envía a HCIS como `MDM^T02`.

  **No sale nada del hospital.**
- **No se ejecuta en el PC de la consulta**, porque no tiene GPU.
- **Si el hospital ya tiene un escriba en la nube**, Medici recibe su nota por HL7 y
  hace igualmente los trámites.

### El escriba que compra la Comunidad de Madrid (T‑Systems, en la nube)

Es el expediente **A/SER‑000964/2026** de la Consejería de Digitalización y la
Dirección General de Salud Digital. Hemos leído el anuncio, la memoria
económica, el pliego técnico y la formalización.

**El contrato:**

| | |
|---|---|
| **Adjudicado a** | **T‑Systems ITC Iberia**, el 14‑jul‑2026, entre 7 ofertas |
| **Importe adjudicado** | **933.557 € + IVA = 1.129.604 €**. El presupuesto de licitación era de 1.744.336 € |
| Duración | 3 meses, sin prórroga. Se formalizó en julio, así que **seguramente está en marcha ahora** ⚠️ |
| Antecedente | Escala a toda la Primaria un **piloto anterior**. Hubo una licitación en 2025 (12 meses, 1,81 M€) que se desistió en agosto de 2025 ⚠️ |
| Financiación | Fondos europeos NextGenerationEU. El hito del plan de recuperación vence en el segundo trimestre de 2026 |
| Tamaño | **Hasta 2.500 micrófonos** en las consultas de Primaria |
| **Dónde corre** | **En la nube pública:** la herramienta LLM, el reconocimiento de voz, la base de datos y la infraestructura |

**Cómo se reparte el presupuesto de licitación** (memoria económica, sin IVA):

| Partida | Importe |
|---|---|
| **Nube durante 3 meses** | **400.000 €** |
| · Reconocimiento de voz | 184.014 € |
| · LLM | 85.200 € |
| · Aplicaciones y base de datos | 66.000 € |
| · Entornos de desarrollo y preproducción | 35.712 € |
| · Infraestructura | 29.074 € |
| Gastos generales y beneficio de la nube | 48.000 € |
| **Servicios:** 40 personas y 18.000 horas, de ellas 24 analistas‑programadores | **993.600 €** |
| **Total** | **1.441.600 €** (con IVA, 1.744.336 €) |

**Qué tiene que hacer su IA** (pliego técnico, apartados 4 y 5):

- **Transcribir en tiempo real** la consulta de Primaria.
- **Sacar un informe estructurado** con la información clínica relevante, con
  terminología médica, abreviaturas y nombres de fármacos.
- **Supervisión humana:** el médico revisa y edita el informe antes de que
  entre en la historia clínica.
- **Tenerlo listo antes de que el paciente salga** de la consulta.
- **Varios idiomas:** transcribir y traducir conversaciones, como mínimo en
  castellano.
- **Borrar** la grabación y la transcripción cuando el médico aprueba el
  informe.
- **Automatizar trámites en APMadrid:** recetas, bajas y **derivaciones**.
- **Integración nativa dentro de APMadrid**, que hace T‑Systems por su cuenta.
- **Que se pueda cambiar de LLM** sin parar el servicio.
- **Indicadores:** operatividad, tiempo de consulta y satisfacción.
- **Formación** de los profesionales y soporte.
- **Un estudio de lo que costará mantenerlo funcionando:** cómputo,
  mantenimiento, personal y consumo eléctrico. **Ese coste no está pagado** en
  este contrato.
- **Cumplir el Reglamento europeo de IA:** clasificarlo por riesgo y, si hace
  falta, el marcado CE.

**Qué hace Medici que su IA no hace:**

| | Escriba de la Comunidad (T‑Systems) | Medici |
|---|---|---|
| Dónde trabaja | **Primaria**, dentro de APMadrid | **Hospital**: consultas externas, con HCIS y HL7 |
| La derivación | **La crea** en Primaria | **La lee al llegar al hospital y prepara la consulta**: pruebas antes de la primera visita, revisión directa o devolución si falta algo |
| Preguntar al historial | No está en el pliego | «Pregúntale a Medici», con la fuente de cada dato |
| La vuelta a Primaria | No | Redacta qué hacer y qué pruebas no repetir |
| El paciente | Traduce la conversación | Le da las instrucciones de su prueba en lectura fácil, en su idioma y en audio |
| Dónde corren los datos | **Nube pública** | **Dentro del hospital** |
| Coste | Nube presupuestada en **400.000 € por 3 meses**; lo que cueste después no está pagado | Equipo de compra única: **4.700 € el Spark** |
| Dependencia | Integración a medida en APMadrid hecha por T‑Systems | HL7 estándar: vale para cualquier hospital y cualquier escriba |

**Lo que su IA hace y Medici no** (decirlo si preguntan):

- Escriba en tiempo real con 2.500 micrófonos ya desplegados.
- Recetas y bajas.
- Integración nativa en la historia de Primaria.

**Medici no compite con ella: va justo después.** Su IA escribe la derivación en
Primaria y Medici la lee en el hospital. Si sus derivaciones llegan mejor
estructuradas, Medici acierta más.

**Comparaciones que aguantan:**

- ✅ *«Solo la nube del escriba de Primaria estaba presupuestada en 400.000 €
  para tres meses, y lo que cueste mantenerla después no estaba pagado. Medici
  corre en un equipo que se compra una vez.»*
- ✅ *«El propio contrato pide un estudio de cuánto costará mantenerlo. A la
  Consejería le preocupa el coste de seguir; con Medici ese coste se conoce desde
  el primer día.»*
- ❌ *«Pagan 1,1 millones por tres meses de uso.»* Es falso: casi 1 M€ son
  servicios (40 personas para integrarlo y desplegarlo).
- ❌ *«Lo mismo por el 1 %.»* No hace lo mismo y no cubre lo mismo.

**Fuentes:**

- [Formalización (BOCM 7‑ago‑2026)](https://www.bocm.es/boletin/CM_Orden_BOCM/2026/08/07/BOCM-20260807-17.PDF)
- [Memoria económica](http://contratos-publicos.comunidad.madrid/medias/02-meconomicatranscripciondevoz19022026fdaanonimizadopdf/download)
- [Pliego técnico](http://contratos-publicos.comunidad.madrid/medias/03-ppttranscripciondevoz19022026fdoanonimizadopdf/download)
- [Ficha del contrato en el portal](https://contratos-publicos.comunidad.madrid/contrato-publico/print/pdf/node/295221)
- [Licitación (BOCM 16‑mar‑2026)](https://www.bocm.es/boletin/CM_Orden_BOCM/2026/03/16/BOCM-20260316-29.PDF)
- [TED 154988‑2026](https://ted.europa.eu/es/notice/154988-2026/pdf)

### Cuánto cuesta un escriba comercial frente a Medici

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
| **Medici entero**: 3 Spark amortizados en 4 años (~3.500 €), luz (~1.000 €) y una licencia anual de hospital (~50.000 €, hipótesis nuestra ⚠️) | **~55.000 €** |

- **Por médico:** unos **15 € al mes** con Medici, frente a **110–600 $** de un
  escriba.
- **Y Medici hace más que escribir:** lee las derivaciones, responde a preguntas
  sobre el historial y cierra la consulta.

**Con cuidado:**

- Los precios de los escribas son **estimaciones de terceros en EE. UU.**; en
  España se negocian por volumen.
- **Los escribas comerciales llevan años de producto y Medici es un piloto.** Si
  el jurado pregunta por la calidad, la respuesta es el mes en paralelo midiendo
  el acierto, no prometer que somos igual de buenos.
- **La integración HL7 y la formación del piloto** cuestan aparte y hay que
  estimarlas ⚠️.
- **La licencia de 50.000 € es una hipótesis** del modelo de negocio. Se paga con
  unas 700 revisiones evitadas, el 5 % del escenario del pitch.

### Aprende el vocabulario de los médicos

> Hoja de ruta, no decidido.

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

**La frase:** *«Lo que Medici aprende se queda en el hospital.»* Con las
correcciones de vuestros médicos no se entrena el modelo de ninguna empresa.

### Sabe quién es cada médico

> Propuesta: para el chatbot sirve el login, que limita a cada médico a sus
> pacientes y registra las preguntas.

- **Por el login y la agenda, no por la voz.** El médico entra con su usuario o
  con el directorio del hospital, y la agenda dice quién pasa consulta en cada
  sala. En los CSV es `AGENDAS.profesional_ficticio`.
- **Para qué sirve:**
  - su glosario y su estilo de nota;
  - «Pregúntale a Medici» solo le enseña **sus pacientes del día**;
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
  Medici y los modelos ya instalados.
- **Opción B: el hospital ya tiene el equipo.** Se le pasa un instalador:
  - en el Mac, un `.pkg` con doble clic;
  - en el Spark, que trae DGX OS, basado en Ubuntu, un `.deb` o un
    `instalar.sh`.

  Los modelos van dentro del instalador, porque **no hace falta internet**
  (`HF_HUB_OFFLINE=1`) y llevan sumas de comprobación.

**El primer arranque, en `http://medici.local`:**

1. **Conectar con HCIS.** Informática escribe la IP y el puerto del motor de
   integración HL7. Hay un botón «Probar conexión» que se pone en verde.
2. **Elegir el servicio.** Se carga su protocolo y su glosario, con plantillas
   para Digestivo, Cardiología…
3. **Usuarios.** Se conecta al directorio del hospital o se crean a mano.

Al terminar, Medici se prueba solo con una derivación de ejemplo y enseña el
resultado.

**Qué hay debajo** (el instalador lo decide solo; nadie lo ve):

| | Mac mini M6 | DGX Spark |
|---|---|---|
| Modelo de texto | Ollama (Metal) | vLLM (contenedor de NVIDIA), que aguanta muchas peticiones a la vez |
| Transcripción | `whisper.cpp` (Metal) | `whisper.cpp` (CUDA) |
| Medici (API + web) | Servicio de `launchd`, arranca solo | Servicio de `systemd` o Docker, arranca solo |
| Perfil de modelos | Lo elige solo según la memoria: 16 GB → Qwen3‑4B, 32 GB → Qwen3‑8B | 128 GB → Qwen3‑30B‑A3B |

Medici habla con los dos motores por la misma API compatible con OpenAI, así que
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

- **Que funcione igual en los dos equipos:** Medici usa la API compatible con
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
- Resúmenes previos a la visita: [Commure · pre-visit summaries](https://www.commure.com/blog/pre-visit-summaries-what-are-they-why-they-matter-and-how-ai-is-changing-pre-charting) · [Navina](https://tooldirectory.ai/tools/navina) · [Epic · AI Charting](https://www.healthcareitnews.com/news/epics-ai-charting-tool-now-available-ehr-clients) · [AAFP · herramientas para revisar historias](https://www.aafp.org/practice-operations/administrative-simplification/chart-review/technologies-chart-review-burden)
- Tiempo en la historia clínica: [Overhage y McCallie, Annals of Internal Medicine 2020 (resumen en Fierce Healthcare)](https://www.fiercehealthcare.com/practices/for-each-patient-visit-physicians-spend-about-16-minutes-ehrs-study-finds) · [Healio](https://www.healio.com/news/primary-care/20200113/physicians-spend-16-minutes-using-ehr-for-each-patient-encounter)
- HORUS: [Comunidad de Madrid · Libre elección (qué ve el profesional)](https://www.comunidad.madrid/salud/libre-eleccion-sanitaria) · [Plataformas informáticas del SERMAS](https://www.vence.es/auxiliar-administrativo-sermas/temario/tema-26) · [iSanidad · premio a HORUS](https://isanidad.com/32259/premio-a-la-comunidad-de-madrid-por-la-plataforma-horus-que-auna-datos-clinicos-de-los-pacientes-y-se-pueden-consultar-en-cualquier-lugar/) · [El Médico Interactivo (2013)](https://elmedicointeractivo.com/principal-ventaja-horus-posibilidad-conocer-informacion-previa-pacientes-hayan-sido-atendidos-cualquier-centro-sermas-20130617132816023000/)
- Infraestructura: [LiteLLM (MIT)](https://github.com/BerriAI/litellm) · [vLLM](https://github.com/vllm-project/vllm) · [Ollama](https://github.com/ollama/ollama) · [Caddy](https://caddyserver.com/)

- Dedalus Iberia, folleto de HCIS (J2EE, «multicapa basado en web»): https://www.dedalus.com/spain/wp-content/uploads/sites/27/2021/04/HCIS_brochure_DedalusIBERIA.pdf
- BOCM 10‑ene‑2025, mantenimiento de HIS1 y HCIS (A/SER‑017556/2024): https://bocm.es/boletin/CM_Orden_BOCM/2025/01/10/BOCM-20250110-28.PDF
