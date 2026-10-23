# Pitch · 5 minutos

> **Rehecho el 23‑oct por la noche para la idea decidida: Jano, el chatbot
> copiloto del médico.** La versión anterior, la de preparar la consulta y
> adelantar pruebas, está en git (commit `8146b89`).
>
> **Regla:** en presente solo se dice lo que enseña la demo. Lo demás va como
> plan («en el piloto», «lo siguiente»).

**Jurado:** la dirección del Hospital Universitario de La Princesa, Wayra y dos
personas de Pfizer. Cada uno escucha una cosa distinta (ver
[CONTEXTO.md](CONTEXTO.md#qué-le-interesa-a-cada-organizador)):

| Jurado | Lo que tiene que llevarse |
|---|---|
| Dirección de La Princesa | Sus médicos dejan de buscar y empiezan a preguntar, con **IA local**: nada sale del hospital |
| Wayra | Un equipo de unos 4.700 €, sin cuota por médico, que vale para cualquier hospital |
| Pfizer | IA responsable: dice de dónde saca cada dato y no se inventa nada |

**Nombre provisional: Jano.** Alternativas: *Umbral* o *Prima*. Hay que decidirlo
en equipo.

**La frase:** *Jano es el copiloto del médico: le preguntas por el paciente y te
contesta, con la fuente, sin que nada salga del hospital.*

> ⚠️ **Antes de presentar, confirmar con los mentores** lo que va marcado con ⚠️.
> Si algo no se confirma, se quita la frase.
> - Cuántos sistemas abre un especialista de La Princesa antes de cada paciente
>   (HCIS, HORUS…) y cuánto tarda.
> - Si La Princesa ya tiene un asistente o un chat sobre la historia clínica.
> - Cómo se podrían leer los datos del paciente: HL7, una réplica de solo
>   lectura… (Informática).
> - Si un chat que solo busca y muestra datos entra en el reglamento de
>   productos sanitarios (MDR).

## Quién habla

| Tramo | Quién | Tiempo |
|---|---|---|
| 1. Buscar en vez de atender | Biomédica | 0:00 – 0:45 |
| 2. Jano y la demo | UI/UX | 0:45 – 2:30 |
| 3. Por qué es viable | Dev 1 | 2:30 – 3:40 |
| 4. Impacto y piloto | Dev 1 | 3:40 – 4:30 |
| 5. Cierre | Biomédica | 4:30 – 5:00 |

El Dev 2 lleva el portátil de la demo.

## Diapositivas

1. **Portada:** Jano y la frase.
2. **Buscar en vez de atender:** «5 minutos por paciente revisando la historia»
   y, debajo, varias ventanas abiertas.
3. **Demo** en directo, con el vídeo de respaldo preparado.
4. **Cómo funciona:** pregunta → modelo local → solo lee los datos de ese
   paciente → respuesta con su fuente. Un dibujo.
5. **Viable:** un equipo encima de la mesa (Mac mini o Spark), sin nube, sin
   cuota por médico, instalación en tres pasos.
6. **Impacto y piloto:** horas de búsqueda y lo que se mide en el piloto.
7. **Cierre:** viable · accesible · personalizada, y la petición.

## Guion

Unas 550 palabras. **Ensayar con cronómetro.**

### 1. Buscar en vez de atender · Biomédica · 0:00

> Entra un paciente. Antes de mirarle a la cara, su médico tiene que averiguar
> quién es: qué le pidió su médico de cabecera, qué pruebas tiene, si ya vino
> antes, si faltó a alguna cita. Y eso está repartido en varias pantallas ⚠️.
>
> En un estudio con cien millones de consultas, el médico pasaba **cinco minutos
> por paciente solo revisando la historia clínica**. Cinco minutos que no son
> para el paciente.
>
> En La Princesa hay **casi medio millón de consultas externas al año**.

### 2. Jano y la demo · UI/UX · 0:45

> Esto es Jano. Es el copiloto del médico. En vez de buscar, pregunta.
>
> *(Demo, agenda del día; se elige a María.)*
>
> «¿Ha estado antes en Digestivo?» Jano contesta y, debajo, enseña **de dónde
> lo saca**: la cita de marzo, en este registro.
>
> «¿Qué pruebas tiene pendientes?» La ecografía, solicitada y sin hacer.
>
> «¿Necesita algún apoyo para la consulta?» Sí: intérprete. El médico lo sabe
> antes de que María entre.
>
> Y ahora una pregunta trampa: «¿Tiene alergias?» Jano contesta: **«No consta
> en los datos disponibles.»** No se inventa nada. Si no lo sabe, lo dice.
>
> **Jano solo lee. No decide nada clínico.** Y solo ve al paciente que el médico
> tiene delante.

### 3. Por qué es viable · Dev 1 · 2:30

> Todo lo que habéis visto corre **en este equipo**, sin internet. **Ningún dato
> de un paciente sale del hospital.**
>
> Usamos modelos abiertos con licencia libre: sin coste por uso y sin cuota por
> médico.
>
> Lo diseñamos para un **Mac mini** o un **NVIDIA DGX Spark** de unos 4.700
> euros. Calculamos que un Spark da para cientos de médicos conectados ⚠️; lo
> mediremos con una prueba de carga.
>
> Y queremos que se instale como un router: se enchufa, se abre en el navegador
> y en tres pasos está listo.
>
> En el piloto se conectaría a los datos del hospital por el estándar que ya
> usan sus sistemas ⚠️, y en modo solo lectura.

### 4. Impacto y piloto · Dev 1 · 3:40

> Si Jano le ahorra al médico **un solo minuto** de búsqueda por consulta, en La
> Princesa son **casi 8.000 horas al año**: como **cinco especialistas más**,
> sin contratar a nadie. Con dos minutos, diez.
>
> No lo prometemos: lo medimos. Proponemos un piloto de tres meses en un
> servicio, midiendo:
> - los minutos de búsqueda antes y después;
> - cuántas respuestas son correctas;
> - si los médicos lo siguen usando.

### 5. Cierre · Biomédica · 4:30

> Jano es **viable**, porque cabe encima de una mesa y no saca nada del
> hospital. Es **accesible**, porque el médico sabe antes de que entre el
> paciente si necesita intérprete o ayuda. Y es **personalizado**, porque
> contesta sobre ese paciente y no sobre un protocolo genérico.
>
> Os pedimos **un servicio, tres meses y un equipo encima de la mesa**.
> Gracias.

## De dónde sale cada cifra

| Frase | Cálculo y fuente |
|---|---|
| Cinco minutos por paciente revisando la historia | 5 min 22 s por consulta, el 33 % de 16 minutos de historia clínica por consulta (Overhage y McCallie, *Annals of Internal Medicine* 2020; 100 M de consultas en EE. UU.; estudio de Cerner). **No es una cifra española** |
| Casi medio millón de consultas | 473.257 consultas externas: 178.888 primeras + 294.369 sucesivas (memoria 2022 de La Princesa) |
| 8.000 horas, cinco especialistas | **Hipótesis:** 1 minuto ahorrado × 473.257 consultas = 7.888 h ÷ 1.540 h de consulta al año por especialista (7 h × 220 días; suposición) ≈ 5,1. Con 2 minutos, unos 10 |
| 4.700 € el Spark | Precio de la Founders Edition en la UE, con IVA (oct‑2026) |
| Cientos de médicos por Spark | **Estimado** a partir del benchmark de LMSYS (8B, 32 peticiones a la vez); falta una prueba de carga ⚠️ |

Si os preguntan por los CSV: **son sintéticos.** Sirven para enseñar que Jano
funciona con la estructura de datos del hospital, no para medir el problema.

## Lo que la demo tiene que enseñar

Si algo no llega a tiempo, se cambia el guion, no al revés.

- [ ] La agenda del día y María seleccionada.
- [ ] «¿Ha estado antes en Digestivo?», con la fila de la cita debajo.
- [ ] «¿Qué pruebas tiene pendientes?», con la fila de la prueba.
- [ ] «¿Necesita algún apoyo para la consulta?», sacado de PACIENTES.
- [ ] «¿Tiene alergias?» → **«No consta»**.
- [ ] Todo funcionando **sin internet** (se puede enseñar con el wifi apagado).
- [ ] Respuestas precalculadas y vídeo por si falla en directo.

María es una paciente de los CSV sintéticos, con un nombre inventado para la
demo.

## Preguntas que pueden hacer

### Dirección de La Princesa

- **«¿Y si se inventa algo?»** Solo responde con lo que encuentra en los datos
  de ese paciente, enseña la fila de la que lo saca y, si no está, dice «no
  consta». Lo hemos medido con 30 preguntas de respuesta conocida *(dar la cifra
  real de la prueba; si no se hizo, quitar la frase)*.
- **«¿Qué servidor hace falta?»** Un Mac mini para un servicio, o un DGX Spark
  de unos 4.700 € para muchos médicos. Las cifras exactas, con una prueba de
  carga antes del piloto ⚠️.
- **«¿Cómo se conecta con HCIS?»** No lo tenemos hecho. La propuesta es leer los
  mensajes HL7 que HCIS ya envía, o una réplica de solo lectura, sin tocar su
  base de datos. Lo decidiríamos con Informática ⚠️.
- **«¿Y la protección de datos?»** Nada sale del hospital. Cada médico solo ve a
  sus pacientes y cada pregunta queda registrada con su nombre.
- **«La Comunidad ya ha comprado un escriba.»** Es otra cosa: el escriba de
  T‑Systems escribe la nota en Primaria. Jano responde preguntas en el
  hospital. Se pueden complementar.

### Wayra

- **«¿Competencia?»** El chat con la historia clínica ya existe en EE. UU.
  (Epic, Dragon Copilot de Microsoft, Navina), en la nube y con cuota por
  médico. Lo nuestro:
  - corre en local, en un equipo que se compra una vez;
  - enseña de dónde sale cada dato;
  - está pensado para la sanidad pública española;
  - se instala solo.

  *No decir que no existe nada parecido.*
- **«¿Quién paga y cuánto?»** El hospital: el equipo una vez, más una licencia
  anual por hospital. El precio está por definir; hay que pactarlo en el equipo
  antes de subir.
- **«¿Y después?»** Lo siguiente sería que Jano prepare la consulta antes de
  que llegue el paciente. Hay evidencia de que cuando la prueba ya está hecha se
  resuelven más casos en una visita (49 % frente a 22 %). Es la hoja de ruta, no
  lo que enseñamos hoy.
- **«¿Vais a seguir con esto?»** La respuesta tiene que ser sincera y estar
  pactada en el equipo antes de subir.

### Pfizer

- **«¿Qué tiene de medicina personalizada?»** Responde sobre ese paciente
  concreto: su historia, sus pruebas y lo que necesita para la consulta.
- **«¿Y los sesgos?»** No decide nada: busca y muestra datos con su fuente. En
  el piloto se mide si acierta igual con todos los pacientes.
- **«¿Por qué no un modelo grande en la nube?»** Por privacidad y por coste. La
  dirección del hospital lo quiere local, y para buscar en los datos de un
  paciente basta un modelo pequeño.
