# Pitch · 5 minutos

**Jurado:** la dirección del Hospital Universitario de La Princesa, Wayra y dos
personas de Pfizer. Cada uno escucha una cosa distinta (ver
[CONTEXTO.md](CONTEXTO.md#qué-le-interesa-a-cada-organizador)):

| Jurado | Lo que tiene que llevarse |
|---|---|
| Dirección de La Princesa | Más consultas con los mismos médicos, sin tocar sus sistemas ni cargar a admisión |
| Wayra | Se puede implantar el mes que viene y vale para cualquier hospital |
| Pfizer | Centrado en el paciente, IA responsable cuando llegue y llega a todos |

**Nombre provisional: ReCita.** Alternativas: *Kairos* (en griego, «el momento
oportuno», en la línea de Hera, Thea o Aeris), *Nadie Falta* o *Hueco Vivo*.
Hay que decidirlo en equipo.

**La frase:** *ReCita es la bandeja que cada mañana detecta las citas que se van
a perder y ofrece ese hueco a quien lo espera.*

> ⚠️ **Antes de presentar, confirmar con los mentores** lo que va marcado con ⚠️
> en el guion. Si alguna cosa no se confirma, se quita la frase.
> - Que no es producto sanitario (MDR).
> - La tasa real de faltas de La Princesa y la duración media de las citas.
> - Cómo se reaprovechan hoy los huecos libres.
> - Qué HIS usan y si exporta la agenda.

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
4. **El circuito cerrado:** detectar → contactar → liberar → ofrecer. Un dibujo,
   sin texto.
5. **Demo** en directo, con el vídeo de respaldo preparado.
6. **Viable el mes que viene:** los seis puntos, un icono y tres palabras cada uno.
7. **Impacto y piloto:** el escenario, los indicadores y la licencia por
   hospital.
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

> Por eso hemos creado **ReCita**: una bandeja para el personal de admisión que
> cada mañana enseña qué citas de los próximos días se van a perder y qué hacer
> con cada una.
>
> *[Demo]* Esta es la bandeja de mañana en Dermatología. Tres avisos: citas sin
> confirmar, citas duplicadas y cancelaciones que siguen ocupando un hueco.
>
> Esta paciente no ha contestado al SMS. ReCita propone llamarla, porque tiene
> una discapacidad visual y el SMS no le sirve.
>
> Esta cita está duplicada: un clic y el hueco se libera.
>
> Y ese hueco no se pierde. ReCita propone a quién ofrecérselo entre los que
> esperan: **primero por prioridad clínica**, y solo a quien de verdad puede
> venir, según sus necesidades y por el canal que usa.
>
> La administrativa confirma con un clic. **ReCita propone; la persona decide.**

### 4. Por qué es viable · Dev 1 · 2:50

> Sabemos que lo que importa es si esto se puede poner en marcha. Se puede, y
> pronto.
>
> **Uno:** no toca vuestros sistemas. Solo lee una exportación diaria de la
> agenda; más adelante, por FHIR ⚠️.
>
> **Dos:** no hay caja negra. Son reglas que cualquier supervisora puede leer y
> auditar.
>
> **Tres:** nunca decide sola. Siempre confirma una persona.
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
> de consulta: tres o cuatro especialistas más sin contratar a nadie, y menos
> llamadas de recontacto para admisión.
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

> ReCita es **viable**, porque se monta sobre lo que ya existe. Es
> **accesible**, porque habla a cada paciente por el canal que puede usar. Y es
> **personalizada**, porque reparte cada hueco según la necesidad clínica de
> quien espera.
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

Si os preguntan por los CSV: **son sintéticos**. Sirven para enseñar que la
herramienta funciona con la estructura de datos del hospital, no para medir el
problema. Decirlo antes de que lo pregunten da credibilidad.

## Lo que la demo tiene que enseñar

El guion depende de que la bandeja tenga estas tres listas y estas tres
acciones. Si alguna no llega, se cambia el guion, no al revés.

- [ ] **Citas sin confirmar:** recordatorio sin respuesta, con la propuesta de
      segundo contacto según el canal y la accesibilidad.
- [ ] **Citas duplicadas:** el mismo paciente con dos citas vivas en la misma
      especialidad. Liberar con un clic.
- [ ] **Cancelaciones que siguen ocupando el hueco.**
- [ ] **Hueco liberado → candidatos**: de la lista de espera, ordenados por
      prioridad clínica y después por antigüedad, filtrados por accesibilidad.
- [ ] **Confirmar con un clic** y ver cómo se mueve el contador.
- [ ] **Un paciente con necesidad de accesibilidad** preparado para la demo
      (por ejemplo, visual → llamada).

## Preguntas que pueden hacer

### Dirección de La Princesa

- **«¿Esto no lo hace ya admisión?»** Lo hace a mano y a ciegas: llama a quien
  puede, cuando puede. ReCita le da cada mañana la lista ordenada de lo que se
  va a perder y la acción ya preparada. No sustituye a nadie: le quita trabajo
  de buscar.
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
- **«¿Más trabajo para admisión?»** Menos: en lugar de llamar a todos, llama
  primero a quien hace falta, y deja de gestionar faltas que se podían evitar.

### Wayra

- **«¿Quién paga y cuánto?»** El hospital o el servicio de salud, con una
  licencia anual. Como referencia, el absentismo le cuesta más de 3 M€ al año a
  un solo hospital, el Costa del Sol.
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
