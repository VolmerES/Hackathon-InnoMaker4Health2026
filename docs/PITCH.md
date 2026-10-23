# Pitch · 5 minutos

**Jurado:** la dirección del Hospital Universitario de La Princesa, Wayra y dos
personas de Pfizer. Cada uno escucha una cosa distinta (ver
[CONTEXTO.md](CONTEXTO.md#qué-le-interesa-a-cada-organizador)):

| Jurado | Lo que tiene que llevarse |
|---|---|
| Dirección de La Princesa | Más consultas con los mismos médicos **y menos trabajo para admisión**, casi sin coste y sin tocar sus sistemas |
| Wayra | Se puede implantar el mes que viene, cuesta casi nada y vale para cualquier hospital |
| Pfizer | Centrado en el paciente, IA responsable cuando llegue y llega a todos |

**Nombre provisional: ReCita.** Alternativas: *Kairos* (en griego, «el momento
oportuno», en la línea de Hera, Thea o Aeris), *Nadie Falta* o *Hueco Vivo*.
Hay que decidirlo en equipo.

**La frase:** *ReCita detecta cada noche las citas que se van a perder y ofrece
ese hueco a quien lo espera, sin que nadie tenga que coger el teléfono.*

**La idea de fondo:** lo que más falta en un hospital son personas y
presupuesto. ReCita no pide ninguna de las dos cosas: lo rutinario va solo y
solo llega a una persona lo que de verdad la necesita.

> ⚠️ **Antes de presentar, confirmar con los mentores** lo que va marcado con ⚠️
> en el guion. Si alguna cosa no se confirma, se quita la frase.
> - Que no es producto sanitario (MDR).
> - La tasa real de faltas de La Princesa y la duración media de las citas.
> - Cómo se reaprovechan hoy los huecos libres.
> - Qué HIS usan y si exporta la agenda.
> - Que ya envían recordatorios por SMS, y con qué proveedor.
> - El precio público vigente de una consulta (la orden de 2023 actualizó los
>   115 y 71 € de 2017).

## Quién habla

Tres voces como mucho: cada cambio de persona cuesta segundos. El Dev 2 lleva el
portátil de la demo.

| Tramo | Quién | Tiempo |
|---|---|---|
| 1. El hueco | Biomédica | 0:00 – 0:40 |
| 2. Por qué pasa | Biomédica | 0:40 – 1:20 |
| 3. La solución y la demo | UI/UX | 1:20 – 2:50 |
| 4. Por qué es viable | Dev 1 | 2:50 – 3:50 |
| 5. Impacto y piloto | Dev 1 | 3:50 – 4:30 |
| 6. Cierre | Biomédica | 4:30 – 5:00 |

## Diapositivas

1. **Portada:** ReCita y la frase.
2. **El hueco:** «57.000–71.000 citas vacías al año en La Princesa», con
   «= 12–15 especialistas con la consulta vacía todo el año» debajo.
3. **Por qué:** las tres causas administrativas (16 % · 16 % · 15 %) y «un
   recordatorio solo: +10‑14 %».
4. **El circuito cerrado:** detectar → contactar → liberar → reofrecer, todo
   automático, y una flecha que se desvía a «una persona» solo para las
   excepciones. Un dibujo, sin texto.
5. **Demo** en directo, con el vídeo de respaldo preparado.
6. **Viable el mes que viene:** los seis puntos, un icono y tres palabras cada uno.
7. **Impacto y piloto:** citas, horas y euros recuperados; minutos de admisión
   evitados; los indicadores del piloto y la licencia por hospital.
8. **Cierre:** viable · accesible · personalizada, y la petición.

## Guion

Unas 700 palabras: 5 minutos a ritmo tranquilo. **Ensayar con cronómetro**: si
pasa de 4:45, recortar del tramo 3, nunca del 4.

### 1. El hueco · Biomédica · 0:00

> Mañana por la mañana, en una consulta de La Princesa, un especialista va a
> esperar a un paciente que no va a venir.
>
> Y no será uno. Según los estudios publicados en España, entre el 12 y el 15 %
> de las citas de consultas externas se quedan vacías ⚠️. La Princesa atiende
> 473.000 consultas al año. Eso son **entre 57.000 y 71.000 huecos perdidos**.
>
> Es como si entre 12 y 15 especialistas pasaran el año entero con la consulta
> vacía. Mientras, en la lista de espera, hay pacientes que llevan meses
> esperando ese hueco.

### 2. Por qué pasa · Biomédica · 0:40

> Y lo más importante: **casi nunca es culpa del paciente**.
>
> En el hospital de Albacete preguntaron, uno a uno, a los que no fueron. Casi la
> mitad tenía una causa administrativa: no sabía que tenía cita, tenía la cita
> duplicada, o había cancelado y el sistema no lo registró.
>
> El SMS de recordatorio, que ya se envía, mejora la asistencia solo un 10 %.
> El problema no es avisar. **Es que nadie cierra el circuito.**

### 3. La solución · UI/UX · 1:20

> Por eso hemos creado **ReCita**. Y lo hemos diseñado pensando en lo que más
> falta en un hospital: **personas y presupuesto**. ReCita no pide ninguna de
> las dos.
>
> *[Demo]* Este es Antonio. Recibe el recordatorio de siempre, pero ahora con un
> enlace: confirmar, cancelar o cambiar. Antonio cancela, y **el hueco queda
> libre al momento**, sin que nadie lo toque.
>
> Ese hueco no se pierde. ReCita lo ofrece a quien espera en Dermatología:
> **primero por prioridad clínica**, y solo a quien de verdad puede venir. Lucía
> lo acepta desde el móvil. Ha adelantado su cita dos meses, y nadie ha cogido
> el teléfono.
>
> Lo mismo con las citas duplicadas y con quien no contesta: segundo aviso
> automático, por otro canal.
>
> ¿Y quién no puede hacerlo desde el móvil? Esta paciente tiene una
> discapacidad visual. **Esa es la única que llega a la bandeja de admisión**,
> para que la llame una persona.
>
> **Lo rutinario va solo. El tiempo del personal, para quien lo necesita.**

### 4. Por qué es viable · Dev 1 · 2:50

> Sabemos que lo que importa es si esto se puede poner en marcha. Se puede, y
> pronto.
>
> **Uno:** no necesita personal nuevo, sino que lo libera.
>
> **Dos:** cuesta casi nada. Software libre, sin hardware, en un servidor que ya
> tenéis y con el SMS que ya enviáis ⚠️. El coste nuevo son céntimos por
> mensaje.
>
> **Tres:** no toca vuestros sistemas. Lee una exportación diaria de la agenda,
> y las reglas las decide cada servicio. No hay caja negra.
>
> **Cuatro:** los datos no salen del hospital. Funciona dentro de vuestra red.
>
> **Cinco:** no diagnostica ni trata. No es producto sanitario y no necesita
> marcado CE ⚠️.
>
> **Seis:** lo puede mantener vuestra propia Unidad de Análisis de Datos.
>
> Lo hemos construido en 24 horas sobre la misma estructura de datos que nos
> habéis dado.

### 5. Impacto y piloto · Dev 1 · 3:50

> Si ReCita recupera solo una cuarta parte de esas citas perdidas, son **entre
> 14.000 y 18.000 citas más al año** en La Princesa. Entre 4.700 y 5.900 horas
> de consulta: tres o cuatro especialistas más sin contratar a nadie. A precio
> público, **más de 1,2 millones de euros al año** de actividad recuperada ⚠️.
>
> Y admisión deja de hacer más de mil horas al año de gestión de faltas, además
> de las llamadas que ahora hace el sistema.
>
> Proponemos un **piloto de tres meses en un servicio**, midiendo antes y
> después:
> - las faltas;
> - los huecos reaprovechados;
> - los minutos de admisión;
> - los días de espera.
>
> Y como cualquier hospital con consultas externas tiene este problema, el
> modelo es una **licencia anual por hospital**.

### 6. Cierre · Biomédica · 4:30

> ReCita es **viable**, porque se monta sobre lo que ya existe, sin personas ni
> presupuesto nuevos. Es **accesible**, porque el tiempo que libera es para
> quien no puede hacerlo solo. Y es **personalizada**, porque reparte cada hueco
> según la necesidad clínica de quien espera.
>
> Solo os pedimos una cosa: **un servicio de La Princesa y tres meses para
> demostrarlo**. Gracias.

## De dónde sale cada cifra

| Frase | Cálculo y fuente |
|---|---|
| 12‑15 % de faltas | Albacete 14,6 %, Costa del Sol 14,2 %, Zaragoza 12,5 % ([HALLAZGOS](HALLAZGOS.md#cifras-reales-para-el-pitch)) |
| 473.000 consultas | Memoria 2022 de La Princesa: 178.888 primeras + 294.369 sucesivas |
| 57.000‑71.000 huecos | 473.257 × 12 % y × 15 % |
| 12‑15 especialistas | 18.900‑23.700 h (con 20 min por cita ⚠️) ÷ 1.540 h al año de consulta por especialista (7 h × 220 días, una suposición) |
| Casi la mitad, causa administrativa | Albacete: 16,1 % + 15,6 % + 15,1 % = 46,8 % |
| El SMS mejora un 10 % | Cochrane: RR 1,10‑1,14 |
| 14.000‑18.000 citas, 4.700‑5.900 h | Una cuarta parte de 57.000‑71.000, con 20 min por cita |
| Tres o cuatro especialistas | 4.700‑5.900 h ÷ 1.540 h |
| Más de 1,2 M€ | 14.200‑17.700 citas × 88 € (precio público medio: 115 € la primera, 71 € la sucesiva, con el reparto de La Princesa ⚠️). Es el **valor de la actividad recuperada, no un ahorro de caja**: decirlo así si lo preguntan |
| Más de mil horas de gestión de faltas | 14.200‑17.700 faltas evitadas × 4,2 min (estimación de los datos de la organización) = 990‑1.240 h |

Si os preguntan por los CSV: **son sintéticos**. Sirven para enseñar que la
herramienta funciona con la estructura de datos del hospital, no para medir el
problema. Decirlo antes de que lo pregunten da credibilidad.

## Lo que la demo tiene que enseñar

La demo cuenta una historia con dos móviles simulados en pantalla y la bandeja
al lado. Si algo no llega a tiempo, se cambia el guion, no al revés.

- [ ] **Antonio cancela** desde el enlace del recordatorio → el hueco se libera
      solo.
- [ ] **El hueco se ofrece en cascada**: candidatos de la lista de espera
      ordenados por prioridad clínica y antigüedad, filtrados por
      accesibilidad. **Lucía acepta** desde el móvil.
- [ ] **Cita doble** detectada y resuelta sola (el paciente elige cuál conserva).
- [ ] **Sin respuesta → segundo aviso automático** por otro canal.
- [ ] **La bandeja de excepciones** solo enseña a quien necesita una persona (la
      paciente con discapacidad visual).
- [ ] **Contadores**: resuelto solo frente a resuelto por una persona, huecos
      reocupados y minutos de admisión evitados.

Antonio y Lucía son pacientes de los CSV sintéticos, con nombres inventados
para la demo.

## Preguntas que pueden hacer

### Dirección de La Princesa

- **«¿Esto no lo hace ya admisión?»** Lo hace a mano: llama a quien puede,
  cuando puede. ReCita hace solo lo rutinario y le deja a admisión los casos que
  necesitan a una persona. No sustituye a nadie: le devuelve tiempo.
- **«¿Cuánto nos cuesta?»** Casi nada: software libre en un servidor que ya
  tenéis, el SMS que ya enviáis y céntimos por mensaje nuevo. Lo mantiene
  vuestra Unidad de Análisis de Datos.
- **«¿Y si el paciente no tiene móvil o no sabe usarlo?»** Pasa a la bandeja y
  le llama una persona, como ahora. La diferencia es que admisión tiene tiempo
  para hacerlo, porque lo demás va solo.
- **«¿Cómo se integra con nuestro sistema?»** Solo lectura. Para el piloto
  basta una exportación diaria de la agenda. Después, FHIR. Lo concretamos con
  Informática.
- **«¿Y la protección de datos?»** No sale nada del hospital. No crea datos
  nuevos: usa los que ya están para el mismo fin, que es gestionar la
  asistencia. Se haría la evaluación de impacto como con cualquier sistema.
- **«¿Y si ofrecemos el hueco y luego aparece el paciente original?»** Solo se
  ofrecen huecos liberados de verdad (cancelación o duplicado) o cuando el
  segundo contacto ha fallado y ha pasado la hora límite. Esa hora la decide
  cada servicio. La sobrecitación (que el estudio de Albacete propone alrededor
  del 10 %) sería opcional y la controla el servicio.
- **«¿Más trabajo para admisión?»** Menos: lo rutinario lo hace el sistema y
  admisión deja de gestionar faltas que se podían evitar. En el piloto, los
  cambios se aplican en el HIS en bloque, ya preparados. Después, el sistema
  escribe directamente en la agenda, cuando Informática lo apruebe.

### Wayra

- **«¿Quién paga y cuánto?»** El hospital o el servicio de salud, con una
  licencia anual pequeña. Por ejemplo, 50.000 € al año se pagarían con unas 570
  citas recuperadas, el 4 % del escenario bajo. Como referencia, el absentismo
  le cuesta más de 3 M€ al año a un solo hospital, el Costa del Sol.
- **«¿Competencia?»** Hay proveedores de recordatorios por SMS, pero solo
  recuerdan. Y los módulos de lista de espera del HIS no cierran el circuito.
  Lo nuestro es detectar, liberar y reofrecer por prioridad y accesibilidad.
  *No decir que no existe nada parecido sin haberlo comprobado.*
- **«¿Cómo escala?»** Las reglas se configuran por hospital y por servicio, y
  la entrada de datos es estándar (FHIR).
- **«¿Vais a seguir con esto?»** La respuesta tiene que ser sincera y estar
  pactada en el equipo antes de subir.

### Pfizer

- **«¿Dónde está la IA?»** En la primera versión, a propósito, no la hay: las
  reglas se pueden auditar y no hay que validar ningún modelo para pilotar. La
  fase 2 es un modelo de predicción de faltas con datos reales, como el
  Nostradamus que La Princesa ya tiene en Urgencias.
- **«¿Qué tiene de medicina personalizada?»** Que el circuito se adapta a cada
  paciente: el canal que puede usar, sus necesidades y su prioridad clínica. La
  lista de espera deja de ser una cola igual para todos.
- **«¿Y los sesgos?»** La predicción solo servirá para dar más apoyo (otro
  aviso, otro canal), nunca para quitar una cita o bajar una prioridad.
