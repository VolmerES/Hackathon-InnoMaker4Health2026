# Pitch · 5 minutos

> **Rehecho el 24‑oct para lo que está construido:** la tarjeta «antes de que
> entre», el chat con IA local, los datos por HL7 y las dos caras (la pestaña
> web y la burbuja de escritorio de Daniel). Las versiones anteriores están en
> git.
>
> **Regla:** en presente solo se dice lo que enseña la demo. Lo demás va como
> plan («en el piloto», «lo siguiente»).

**Jurado:** la dirección del Hospital Universitario de La Princesa, Wayra y dos
personas de Pfizer. Cada uno escucha una cosa distinta (ver
[CONTEXTO.md](CONTEXTO.md#qué-le-interesa-a-cada-organizador)):

| Jurado | Lo que tiene que llevarse |
|---|---|
| Dirección de La Princesa | Medicci entra en su hospital **sin tocar HCIS** (HL7) y **sin sacar nada fuera** (IA local). Sus médicos dejan de buscar |
| Wayra | Un equipo de unos 4.700 €, sin cuota por médico, que vale para cualquier hospital con HCIS, web o no |
| Pfizer | IA responsable: cada dato con su fuente, «no consta» cuando no está, y medido: 29 de 30 |

**Nombre: Medicci** (decidido el 23‑oct). Suena a «médicos» en italiano y
recuerda a la familia de Florencia que impulsó el Renacimiento.

> ⚠️ Existe una app estadounidense con un nombre casi igual (Austin, Texas). Para el
> hackathon no importa; si Wayra quiere seguir, hay que mirar la marca en la
> OEPM y la EUIPO.

**La frase:** *Medicci le dice al médico lo que no puede pasar por alto antes de
que entre el paciente, y le contesta lo que pregunte, con la fuente y sin que
nada salga del hospital.*

> ⚠️ **Antes de presentar.** Lo que no se confirme, se quita del guion.
>
> - **Una cifra de La Princesa:** cuántos documentos tiene un paciente típico de
>   consultas externas y cuánto tarda un especialista en revisarlo. Mejor aún,
>   cronometrar a un médico con la réplica de HCIS y con Medicci.
> - **La frase de los médicos** que se llamaron, con el cargo y sin el nombre.
> - Si **La Princesa ya tiene** algo parecido.
> - Cómo se abre **HCIS** en La Princesa: navegador, Java o Citrix.
> - Si un asistente que solo busca y muestra datos entra en el reglamento de
>   productos sanitarios (MDR).
> - El **precio de la licencia**, pactado en el equipo.

## Quién habla

| Tramo | Quién | Tiempo |
|---|---|---|
| 1. Cientos de notas, unos minutos | Biomédica | 0:00 – 0:45 |
| 2. Medicci, en directo | UI/UX (habla) y Dev 2 (maneja) | 0:45 – 2:30 |
| 3. Por qué es viable | Dev 1 | 2:30 – 3:40 |
| 4. Impacto y piloto | Dev 1 | 3:40 – 4:30 |
| 5. Cierre | Biomédica | 4:30 – 5:00 |

## Diapositivas

1. **Portada:** Medicci y la frase.
2. **Cientos de notas, unos minutos:** «359 notas · 58.000 palabras · 4 horas
   de lectura» frente a «5 minutos». Debajo, «la mitad, copiada».
3. **Demo** en directo, con el vídeo de respaldo preparado.
4. **Cómo funciona:** HCIS → mensajes HL7 → Medicci en el CPD → la pestaña (HCIS
   en el navegador) o la burbuja (HCIS de escritorio). Un dibujo.
5. **Viable y barato:** un portátil con una GPU de 8 GB; 29 de 30, 0 fuentes
   inventadas, 0,7 s. Al lado, lo que cuesta: «nube, 0,7–1,4 M$ al año por
   cuota de médico» frente a «local, un equipo de 4.700 € y la luz».
6. **Impacto y piloto:** horas de búsqueda y lo que se mide en el piloto.
7. **Cierre:** viable · accesible · personalizada, y la petición.

## Guion

Unas 580 palabras. **Ensayar con cronómetro.**

### 1. Cientos de notas, unos minutos · Biomédica · 0:00

> Entra una paciente. Antes de mirarla a la cara, su médico tendría que saber
> quién es: por qué se la mandan, qué pruebas tiene pendientes, qué ha pasado
> con sus citas, si necesita algún apoyo.
>
> Todo eso está en la historia clínica. El problema es cuánto ocupa: en un
> hospital grande de Estados Unidos, un paciente llega de media con **359 notas,
> más de 58.000 palabras. Unas cuatro horas de lectura. Y la mitad, copiada** de
> notas anteriores.
>
> El médico tiene unos minutos: en un estudio con cien millones de consultas,
> unos **cinco** para revisar la historia. Así que hojea, y lo que no ve, se le
> escapa.
>
> En La Princesa hay **casi medio millón de consultas externas al año**. Lo
> hemos preguntado a médicos y a los mentores: les sería muy útil ⚠️ *(la frase
> real, con el cargo)*.

### 2. Medicci, en directo · UI/UX · 0:45

> Esto es HCIS, con la agenda de hoy. El médico abre a su paciente como siempre.
> No abre nada nuevo.
>
> *(Dev 2 abre a María Alonso.)*
>
> Y Medicci ya está aquí. Sin preguntarle nada, le dice **tres cosas antes de que
> entre**: tiene **movilidad reducida**; tiene una **radiografía pendiente**; y
> la deriva su médico de cabecera, **con prioridad preferente**, para retomar el
> seguimiento en Dermatología. Cada cosa con su fuente: si pulso, veo el dato
> tal cual está en su historia.
>
> El médico pregunta lo que necesite. «¿Por qué la manda su médico de
> cabecera?» Porque perdió la cita de revisión y hay que retomar el
> seguimiento. Con la derivación debajo.
>
> Ahora una pregunta trampa: «¿Tiene alergias?» **«No consta en los datos
> disponibles.»** No se inventa nada. Lo hemos medido: **29 de 30** respuestas
> correctas, **ninguna fuente inventada**.
>
> Y mientras María espera fuera, llega el resultado de su radiografía. HCIS
> manda el mensaje, como manda tantos al día… *(la tarjeta cambia)* …y Medicci lo
> pone arriba: **nuevo, ya tiene la radiografía hecha**.
>
> Medicci solo lee. No decide nada clínico. Y solo ve al paciente que el médico
> tiene abierto.

### 3. Por qué es viable · Dev 1 · 2:30

> Todo lo que habéis visto corre **en este portátil**, con una tarjeta gráfica
> de 8 GB y sin internet. Ningún dato sale del hospital. Responde en **menos de
> un segundo**.
>
> **No tocamos HCIS.** Medicci recibe los datos por **HL7**, el estándar con el
> que ya se hablan los sistemas del hospital. En la demo, HCIS es simulado, pero
> los mensajes son los estándar.
>
> Y funciona **sea como sea su HCIS**: si va en el navegador, Medicci es una
> pestaña; si es un programa de escritorio, una burbuja encima, que es la que ha
> hecho Daniel. El mismo cerebro detrás.
>
> Para el hospital, un **Mac mini** para un servicio o un **NVIDIA DGX Spark** de
> unos 4.700 euros. Sin nube, sin cuota por médico, con modelos abiertos.
>
> Y la IA **no cuesta por uso**. Un asistente en la nube cobra por médico: para
> los 558 de La Princesa, entre **700.000 y 1,4 millones de dólares al año**.
> Medicci, en local, es el equipo una vez y la luz: cada respuesta gasta unas
> **dos centésimas de vatio-hora**.

### 4. Impacto y piloto · Dev 1 · 3:40

> Si Medicci le ahorra al médico **un solo minuto** por consulta, en La Princesa
> son **casi 8.000 horas al año**: como **cinco especialistas más**, sin
> contratar a nadie.
>
> No lo prometemos: lo medimos. Proponemos un piloto de tres meses en un
> servicio, midiendo los minutos antes y después, cuántas respuestas acierta y
> si los médicos lo siguen usando.

### 5. Cierre · Biomédica · 4:30

> Medicci es **viable**: entra por HL7, sin tocar HCIS, y cabe encima de una
> mesa. Es **accesible**: el médico sabe antes de que entre la paciente que
> tiene movilidad reducida. Y es **personalizado**: habla de esta paciente, no
> de un protocolo.
>
> Os pedimos **un servicio, tres meses y un equipo encima de la mesa**.
> Gracias.

## La demo, paso a paso (1 min 45 s)

**Quién:** UI/UX habla; Dev 2 maneja el portátil y lanza el mensaje HL7.

| Tiempo | Dev 2 hace | Se ve | UI/UX dice |
|---|---|---|---|
| 0:00 | Pantalla en `http://localhost:8000`, agenda de hoy | La réplica de HCIS | «Esto es HCIS, con la agenda de hoy» |
| 0:08 | Pulsa **María Alonso** (09:00) | La historia en tablas y, a la derecha, Medicci con su tarjeta | «Y Medicci ya está aquí… tres cosas antes de que entre» |
| 0:30 | Pulsa la fuente `¹ P1` de la radiografía | La fila tal cual, como cita | «Cada cosa con su fuente» |
| 0:40 | Pulsa la sugerencia **«¿Por qué lo manda su médico de cabecera?»** | Respuesta con `¹ D1`, en un segundo | «Porque perdió la cita de revisión…» |
| 0:55 | Escribe **«¿Tiene alergias?»** | «No consta en los datos disponibles.» | «Pregunta trampa… 29 de 30» |
| 1:05 | **En la terminal:** `docker compose -f mvp/docker-compose.yml run --rm --no-deps emisor --evento resultado` | — | «Y mientras María espera fuera…» |
| 1:20 | Nada: espera | La tarjeta se ilumina; arriba, **«Nuevo: ya tiene hecha la radiografía…»** | «…y Medicci lo pone arriba» |
| 1:35 | (Opcional) Pasa a `/hl7.html` | El mensaje `ORU^R01` con su `ACK` | «Este es el mensaje que ha mandado HCIS» |

- **El mensaje HL7 tarda unos 15 segundos** en llegar: el emisor carga los
  datos al arrancar. Por eso se lanza al acabar la pregunta de las alergias, y
  llega mientras se habla.
- **Si algo falla**, se pasa al vídeo sin dar explicaciones. La tarjeta
  funciona aunque el modelo no responda.

**Preparar antes de subir:**

1. Arrancar limpio: doble clic en **`arrancar.bat`**. En unos 30 s levanta todo,
   manda la historia por HL7, comprueba 8/8 tarjetas y abre a María.
2. Abrir `http://localhost:8000` y, en otra pestaña, `/hl7.html`. Dejar la
   terminal con el comando del evento escrito.
3. Hacer una pregunta de calentamiento, para que el modelo esté cargado.
4. Apagar el wifi: se puede enseñar que funciona sin internet.
5. Tener a mano el vídeo de respaldo.

María es una paciente de los CSV sintéticos, con un nombre inventado.

## Lo que cuesta la IA

Es el argumento más fuerte para Wayra y para la dirección: **la IA local no
cuesta por uso, y funciona**.

| | En la nube, con cuota por médico | Medicci, en local |
|---|---|---|
| Qué se paga | Una cuota al mes por cada médico | El equipo, una vez, y la luz |
| Para los 558 médicos de La Princesa | **0,74–1,39 M$ al año**: 558 × 110–208 $/mes × 12 (precios de Heidi y Abridge, en CONTEXTO) | **4.700 €** un Spark, o **9.400 €** dos para tener reserva |
| Por respuesta | Incluido en la cuota, o por token si es una API | **≈ 0,02 Wh** de luz: 0,7 s × unos 100 W de la GPU del portátil (estimado ⚠️) |
| Todo el año en La Princesa | — | 473.257 consultas × 3 preguntas ≈ 1,4 millones de respuestas ≈ **28 kWh ≈ 4 € de luz** (estimado, solo la GPU) |
| Los datos | Salen del hospital | **No salen** |
| ¿Se puede en local? | — | **Medido:** 29/30 con un modelo de 8B en una GPU de portátil de 8 GB |

- **Las cuotas son de escribas** (Heidi, Abridge), que no hacen lo mismo que
  Medicci. Sirven como orden de magnitud de lo que cuesta la IA clínica cuando
  se paga por médico.
- **El consumo es una estimación:** no lo hemos medido con un vatímetro. Lo
  medido es el tiempo, 0,7 s. Aunque fuera diez veces más, seguirían siendo
  unas decenas de euros al año.
- **Por qué basta un modelo pequeño:** no busca ni razona sobre toda la
  historia. El servidor le da las filas de un paciente (de mediana 7) y él solo
  las ordena y redacta. Lo crítico (la tarjeta) va por reglas, sin IA.

## De dónde sale cada cifra

| Frase | Cálculo y fuente |
|---|---|
| 359 notas, más de 58.000 palabras | Mediana de notas disponibles al llegar a Urgencias en 2022, en un sistema sanitario grande de EE. UU. («Call me Dr Ishmael», [PMC11110617](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11110617/)). **No es una cifra española** |
| Unas cuatro horas de lectura | 58.662 palabras ÷ unas 240 palabras por minuto (lectura en silencio de un adulto, Brysbaert 2019) ≈ 4 h |
| La mitad, copiada | 50,1 % del texto duplicado de notas anteriores del mismo paciente (Penn Medicine, 104 M de notas, [JAMA Netw Open](https://jamanetwork.com/journals/jamanetworkopen/fullarticle/2796664)) |
| Unos cinco minutos para revisar la historia | 5 min 22 s por consulta (Overhage y McCallie, *Annals of Internal Medicine* 2020; 100 M de consultas; estudio de Cerner). En medicina de familia, observado: 2,9 min antes de entrar ([Ann Fam Med](https://www.annfammed.org/content/3/6/488)) |
| Casi medio millón de consultas | 473.257 consultas externas (memoria 2022 de La Princesa) |
| 8.000 horas, cinco especialistas | **Hipótesis:** 1 minuto × 473.257 consultas = 7.888 h ÷ 1.540 h de consulta al año por especialista (suposición) ≈ 5,1 |
| 29 de 30, ninguna fuente inventada, menos de un segundo | `mvp/prueba_acierto.py --semilla 2026` con Qwen3‑8B en un portátil con RTX 5060 de 8 GB (24‑oct): 29/30, 6/6 «no consta», 0 fuentes inventadas, 0,7 s de mediana |
| La tarjeta no se equivoca | Reglas sin IA, contrastadas con SQL directo en 2.000 pacientes: 0 errores (`mvp/prueba_alertas.py`) |
| Por HL7 | 43/43 mensajes aceptados y 8/8 tarjetas idénticas a las de los CSV (`mvp/hl7_emisor.py --comprobar`) |
| 4.700 € el Spark | Precio de la Founders Edition en la UE, con IVA (oct‑2026) |
| 700.000–1,4 M$ al año en la nube | 558 médicos de plantilla (La Princesa, abr‑2024) × 110–208 $/mes (Heidi y Abridge; fuentes en CONTEXTO) × 12. Son escribas: orden de magnitud |
| Dos centésimas de vatio-hora por respuesta | **Estimado:** 0,7 s medidos × unos 100 W de GPU de portátil (supuesto) ≈ 0,02 Wh. Ver [Lo que cuesta la IA](#lo-que-cuesta-la-ia) |

Si os preguntan por los CSV: **son sintéticos.** Sirven para enseñar que Medicci
funciona con la estructura de datos del hospital, no para medir el problema.

## Lo que la demo tiene que enseñar

- [ ] La agenda de HCIS y María abierta como siempre.
- [ ] La tarjeta sola, sin preguntar: movilidad reducida, radiografía pendiente
      y derivación preferente.
- [ ] Una fuente pulsada: la fila tal cual.
- [ ] «¿Por qué lo manda su médico de cabecera?», con `¹ D1`.
- [ ] «¿Tiene alergias?» → **«No consta»**.
- [ ] El resultado que llega por HL7 y la tarjeta que cambia.
- [ ] Todo **sin internet**.
- [ ] El vídeo de respaldo, preparado.

## Preguntas que pueden hacer

### Dirección de La Princesa

- **«¿Y si se inventa algo?»** Solo responde con los datos de ese paciente,
  enseña la fila de la que lo saca y, si no está, dice «no consta». Medido con
  pacientes que no usamos para ajustarlo: **29 de 30, 6 de 6 «no consta» y
  ninguna fuente inventada**. El fallo fue una respuesta incompleta, no
  inventada. Y la tarjeta no usa IA: son reglas, contrastadas en 2.000
  pacientes sin un error.
- **«¿Cómo se conecta con HCIS?»** Como cualquier otro sistema del hospital:
  escucha los mensajes **HL7** que HCIS ya manda por el motor de integración,
  sin tocar su base de datos. En la demo, HCIS es simulado y los mensajes son
  los estándar. En el piloto se ajusta al perfil de mensajes de Informática ⚠️.
- **«¿Y si nuestro HCIS no va en el navegador?»** Entonces Medicci es la burbuja
  de escritorio, que se pone encima de cualquier ventana. Es el mismo servidor.
- **«¿Y si el pliego dice que va en otro sitio?»** Va donde diga: en el CPD del
  hospital, en el de la Comunidad o en la nube del pliego. El modelo, el
  servidor y el receptor HL7 se apuntan en un único fichero, sin tocar código.
  Lo que hay que decir con honestidad: si el pliego pone el modelo fuera del
  hospital, los datos llegan hasta allí. Nuestra propuesta es que se quede
  dentro.
- **«¿Qué servidor hace falta?»** Un Mac mini para un servicio, o un DGX Spark
  de unos 4.700 € para muchos médicos. La capacidad exacta, con una prueba de
  carga antes del piloto ⚠️.
- **«¿Y la protección de datos?»** Nada sale del hospital. La sesión queda atada
  a un paciente, el modelo no puede consultar a otro, y cada pregunta queda
  registrada con quién la hizo.
- **«La Comunidad ya ha comprado un escriba.»** Es otra cosa: el escriba de
  T‑Systems escribe la nota en Primaria. Medicci avisa y responde en el
  hospital, antes de que entre el paciente. Se pueden complementar.

### Wayra

- **«¿Competencia?»** Hay asistentes que se ponen encima de la historia clínica
  (Heidi, Suki, Noa de Docplanner) y chats con la historia (Epic, Dragon
  Copilot, Navina). Casi todos van en la nube y cobran por médico. Lo nuestro:
  - corre en el hospital, en un equipo que se compra una vez;
  - **avisa antes de que entre el paciente**, con reglas que no se equivocan;
  - junta lo clínico y lo administrativo (citas cerradas sin atender, avisos
    que no llegan), que el médico no ve en la misma pantalla;
  - entra por HL7, sin proyecto de integración con el fabricante.

  *No decir que no existe nada parecido.*
- **«¿Quién paga y cuánto?»** El hospital: el equipo una vez, más una licencia
  anual por hospital. El precio, pactado en el equipo antes de subir ⚠️. Frente a
  0,7–1,4 M$ al año en cuotas por médico, hay mucho margen. Ver
  [Lo que cuesta la IA](#lo-que-cuesta-la-ia).
- **«¿Y después?»** Lo siguiente es leer el texto libre de los informes, con
  contradicciones y alergias. Daniel lo tiene en prototipo con dos historias.
  Y después, preparar la consulta antes de que llegue el paciente. Es la hoja
  de ruta, no lo que enseñamos hoy.
- **«¿Vais a seguir con esto?»** La respuesta tiene que ser sincera y estar
  pactada en el equipo antes de subir.

### Pfizer

- **«¿Qué tiene de medicina personalizada?»** Habla de ese paciente: sus
  pruebas, sus citas, lo que necesita para la consulta. No de un protocolo
  genérico.
- **«¿Y la accesibilidad?»** El médico sabe antes de que entre si el paciente
  necesita intérprete, tiene movilidad reducida o no le llegan los avisos por
  SMS.
- **«¿Y los sesgos?»** No decide nada: busca y muestra datos con su fuente. En
  el piloto se mide si acierta igual con todos los pacientes.
- **«¿Por qué no un modelo grande en la nube?»** Por privacidad y por coste. Un
  modelo pequeño basta: solo ordena las filas que le da el servidor, y lo hemos
  medido.
