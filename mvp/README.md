# mvp/

El núcleo de Medici para la demo, **sin agente**: el servidor saca con SQL
fijo todo lo que hay del paciente y el modelo local solo lo ordena y contesta
citando las filas.

```text
paciente.py       las 6 consultas fijas: filas numeradas [F1] [D1] [C1] [M1] [P1] [A1]
responder.py      una sola llamada al modelo (API compatible con OpenAI) y comprobación de las fuentes
alertas.py        la tarjeta «3 cosas antes de que entre»: reglas, sin IA
prueba_acierto.py 30 preguntas con la respuesta correcta sacada de los CSV, y la nota
prueba_alertas.py contrasta cada alerta con SQL directo en una muestra de pacientes
resultados/       el detalle de cada prueba (no se sube: lleva filas de los CSV)
```

Solo usa la biblioteca estándar y `duckdb`. Los CSV tienen que estar en `data/`.
Los datos se leen con SQL sobre los CSV; HL7 todavía no se usa.

## La demo: HCIS con la pestaña Medici

```bash
docker compose -f mvp/docker-compose.yml up -d            # modelo + servidor
# o sin Docker, con Ollama en marcha:
.venv/Scripts/python mvp/servidor.py
```

Abrir **http://localhost:8000**. Es una réplica de HCIS con la agenda del día
(8 pacientes de los CSV con nombres inventados). Al abrir un paciente, la
pestaña Medici de la derecha:

1. lee el NHC de la cinta del paciente, como haría la extensión del navegador;
2. abre una sesión atada a ese paciente;
3. enseña la tarjeta «antes de que entre» y deja preguntar.

Cada dato lleva un botón con su fuente (`P1`, `M2`…). Al pulsarlo se ve la
fila tal cual. Si el modelo no responde, la tarjeta sigue funcionando.

```text
servidor.py       API (sesión, tarjeta, pregunta) y la réplica de HCIS; solo biblioteca estándar
web/index.html    la réplica de HCIS: no sabe nada de Medici
web/pestana.js    la pestaña: solo lee el NHC de la cinta (#c-nhc) y habla con la API
```

**María Alonso**, la primera de la agenda, es la del guion. Tiene movilidad
reducida, una radiografía pendiente y una derivación preferente de Primaria.
Cada pregunta queda registrada en `resultados/registro.jsonl`.

## Los datos por HL7

Así se conectaría Medici en el hospital: **sin tocar la base de datos de HCIS**,
recibiendo los mensajes HL7 v2 que HCIS ya manda por el motor de integración.
En la demo, `hl7_emisor.py` hace de HCIS: lee los CSV y manda cada hecho como
su mensaje. Medici arranca con la copia vacía y la va llenando.

```bash
export MEDICI_DATOS=hl7                    # la copia empieza vacía
docker compose -f mvp/docker-compose.yml up -d
docker compose -f mvp/docker-compose.yml run --rm --no-deps emisor --comprobar
# en directo, con María abierta en la pantalla:
docker compose -f mvp/docker-compose.yml run --rm --no-deps emisor --evento resultado
docker compose -f mvp/docker-compose.yml run --rm --no-deps emisor --evento anular
```

Sin Docker: `python mvp/servidor.py --datos hl7` y `python mvp/hl7_emisor.py
--comprobar`. Los mensajes se ven llegar en **http://localhost:8000/hl7.html**.

| Hecho | Mensaje | Hecho | Mensaje |
|---|---|---|---|
| Paciente | `ADT^A04` | Cita reprogramada | `SIU^S13` |
| Derivación de Primaria | `REF^I12` | Cita atendida | `SIU^S14` |
| Cita nueva | `SIU^S12` | Cita anulada | `SIU^S15` |
| Petición de prueba | `OMG^O19` | Cita no realizada | `SIU^S26` |
| Resultado | `ORU^R01` | Avisos (SMS, email) | No van por HL7: se cargan de su CSV |

**Comprobado el 24‑oct:**

- Los 43 mensajes de los 8 pacientes de la agenda llegan todos con `ACK AA`.
- Las 8 tarjetas y todas las filas son **idénticas** a las que salen de los
  CSV.
- Los dos eventos en directo salen arriba de la tarjeta con «Nuevo:».

`hl7.py` es un lector, escritor y transporte MLLP mínimo, con la biblioteca
estándar. En el hospital se usaría `hl7apy` y el perfil de mensajes que dé
Informática ⚠️. Con `docker compose run`, poned `--no-deps`: sin él, Compose
vuelve a crear el servidor en modo CSV si `MEDICI_DATOS` no está en la consola.

## La tarjeta «3 cosas antes de que entre»

```bash
.venv/Scripts/python mvp/alertas.py P0037782            # la tarjeta de un paciente
.venv/Scripts/python mvp/alertas.py P0037782 --json     # lo que recibiría la pantalla
.venv/Scripts/python mvp/alertas.py --buscar-demo       # pacientes con tres cosas distintas
.venv/Scripts/python mvp/prueba_alertas.py              # 2.000 pacientes contra SQL directo
```

Resultado del 24‑oct: **0 alertas que no cuadren** en 2.000 pacientes. Lo
ocurrido en las últimas 48 horas sube arriba con «Nuevo:», por ejemplo una cita
que se acaba de anular o un resultado que acaba de llegar. Qué sale
y por qué, en
[`docs/CONTEXTO.md`](../docs/CONTEXTO.md#la-tarjeta-3-cosas-antes-de-que-entre-construida-el-24oct).

## Medir el acierto

### Con Docker (lo que se usó el 24‑oct)

Hace falta Docker Desktop con la GPU de NVIDIA disponible:

```bash
docker compose -f mvp/docker-compose.yml up -d ollama
docker exec medici-ollama ollama pull qwen3:8b
docker compose -f mvp/docker-compose.yml run --rm prueba --semilla 2026
docker compose -f mvp/docker-compose.yml run --rm prueba --modelo qwen3:4b
```

En Git Bash para Windows, poner `MSYS_NO_PATHCONV=1` delante de
`docker compose`.

**Resultado con `qwen3:8b` en una RTX 5060 de 8 GB:** 29/30 con pacientes que no
se usaron para ajustar nada (semilla 2026). Además, 6/6 «no consta», 0 filas
inventadas y 0,7 s de mediana por respuesta. Detalle en
[`docs/CONTEXTO.md`](../docs/CONTEXTO.md#cómo-construirlo-propuesta-pendiente-del-equipo).

### Sin Docker

En el portátil de la demo, con [Ollama](https://ollama.com) instalado:

```bash
ollama pull qwen3:8b                                  # Q4_K_M, unos 5 GB
.venv/Scripts/python mvp/prueba_acierto.py --solo-preguntas   # ver las 30 sin modelo
.venv/Scripts/python mvp/prueba_acierto.py                    # qwen3:8b en localhost:11434
.venv/Scripts/python mvp/prueba_acierto.py --modelo qwen3:4b  # comparar con el pequeño
```

Para otro servidor (vLLM en un Spark, `llama.cpp`…): `--url http://equipo:8000/v1
--modelo <nombre>`, o las variables `MEDICI_URL` y `MEDICI_MODELO`.

Al final sale la nota por tipo de pregunta, cuántas veces dice «no consta»
cuando toca, cuántas filas inexistentes ha citado y los segundos por respuesta.

## Qué preguntas hace

| Tipo | Cuántas | Se da por buena si… |
|---|---|---|
| ¿Ha tenido citas en {servicio}? (sí) | 3 | cita una cita de ese servicio |
| ¿Ha tenido citas en {servicio}? (no) | 2 | dice que no y no se inventa ninguna |
| ¿Qué pruebas tiene pendientes? | 5 | nombra y cita todas las programadas (una sin ninguna) |
| ¿Necesita algún apoyo para la consulta? | 4 | da la necesidad y cita la ficha (una sin ninguna) |
| ¿Por qué lo manda su médico de cabecera? | 4 | cita la derivación y da el motivo |
| ¿Le hemos cambiado o anulado alguna cita, o ha faltado? | 4 | cita el cambio y da el motivo (uno sin cambios) |
| ¿Cómo prefiere que le avisemos? | 2 | da el canal y cita la ficha |
| Trampas: alergias, medicación, diagnóstico, antecedentes, tabaco, valores de la analítica | 6 | dice «no consta» sin fuentes |

**La respuesta correcta no la escribe nadie a mano:** sale de los mismos datos.
Los pacientes salen al azar pero son siempre los mismos con la misma
`--semilla`, así se comparan modelos con las mismas preguntas.

**Referencia:** un «modelo» que contestara siempre «no consta» sacaría 11 de
30. Esa es la nota mínima que hay que superar con claridad.

La corrección es automática y estricta con las palabras: el detalle en
`resultados/` sirve para revisar a mano los fallos dudosos. Las frases de las
preguntas están arriba en `prueba_acierto.py` (`PREGUNTAS`, `TRAMPAS`) para
que la biomédica las cambie por las que de verdad hace un especialista.
