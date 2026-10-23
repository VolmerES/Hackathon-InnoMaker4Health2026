# Pitch · 5 minutos

**Jurado:** la dirección del Hospital Universitario de La Princesa, Wayra y dos
personas de Pfizer. Cada uno escucha una cosa distinta (ver
[CONTEXTO.md](CONTEXTO.md#qué-le-interesa-a-cada-organizador)):

| Jurado | Lo que tiene que llevarse |
|---|---|
| Dirección de La Princesa | Menos revisiones inútiles con los mismos médicos y **IA local**, que es lo que le gusta: nada sale del hospital |
| Wayra | Se puede implantar el mes que viene, cuesta casi nada y vale para cualquier hospital |
| Pfizer | Diagnóstico antes, IA responsable y centrada en el paciente |

**Nombre provisional: Jano**, el dios romano de las puertas: una cara mira a
Primaria y otra al hospital. Alternativas: *Umbral* o *Prima*. Hay que decidirlo
en equipo.

**La frase:** *Jano lee cada noche, dentro del hospital, las derivaciones de
Primaria y prepara cada caso para que la primera consulta lo resuelva.*

> ⚠️ **Antes de presentar, confirmar con los mentores** lo que va marcado con ⚠️
> en el guion. Si alguna cosa no se confirma, se quita la frase.
> - Quién valida hoy las derivaciones en La Princesa y cuánto tarda.
> - Qué parte de las primeras consultas pide una prueba.
> - Si La Princesa ya tiene algún circuito de alta resolución.
> - Si adelantar pruebas con un protocolo entra en el reglamento de productos
>   sanitarios (MDR).
> - Si la Unidad de Análisis de Datos tiene servidor con GPU.
> - El precio público vigente de una consulta sucesiva.

## Quién habla

Tres voces como mucho: cada cambio de persona cuesta segundos. El Dev 2 lleva el
portátil de la demo.

| Tramo | Quién | Tiempo |
|---|---|---|
| 1. La visita que sobra | Biomédica | 0:00 – 0:45 |
| 2. La evidencia | Biomédica | 0:45 – 1:20 |
| 3. Jano y la demo | UI/UX | 1:20 – 2:50 |
| 4. Por qué es viable | Dev 1 | 2:50 – 3:50 |
| 5. Impacto y piloto | Dev 1 | 3:50 – 4:30 |
| 6. Cierre | Biomédica | 4:30 – 5:00 |

## Diapositivas

1. **Portada:** Jano y la frase.
2. **La visita que sobra:** «294.369 consultas de revisión al año en La
   Princesa», con «muchas solo para ver un resultado» debajo.
3. **La evidencia:** 22 % → 49 % resueltos en una visita cuando la prueba ya
   está hecha.
4. **El circuito:** derivación → Jano lee (dentro del hospital) → protocolo del
   servicio → el especialista valida en bloque → prueba → primera consulta que
   resuelve. Un dibujo, sin texto.
5. **Demo** en directo, con el vídeo de respaldo preparado.
6. **Viable el mes que viene:** los seis puntos, un icono y tres palabras cada uno.
7. **Impacto y piloto:** revisiones, horas y euros; los indicadores del piloto
   en Digestivo y la licencia por hospital.
8. **Cierre:** viable · accesible · personalizada, y la petición.

## Guion

Unas 650 palabras: 5 minutos a ritmo tranquilo. **Ensayar con cronómetro**: si
pasa de 4:45, recortar del tramo 3, nunca del 4.

### 1. La visita que sobra · Biomédica · 0:00

> María lleva cinco meses con molestias de estómago. Su médica de cabecera la
> deriva a Digestivo. Tras semanas de espera, llega a su primera consulta. El
> especialista la escucha veinte minutos… y le pide una ecografía. «Vuelva con
> el resultado.»
>
> Esa segunda visita no aporta nada que no pudiéramos tener el primer día. Y no
> es un caso raro: La Princesa hace **294.000 consultas de revisión al año**,
> 1,65 por cada primera. Muchas solo sirven para ver un resultado.

### 2. La evidencia · Biomédica · 0:45

> Está medido. Cuando la prueba ya está hecha, la primera consulta resuelve el
> caso en una sola visita **en el 49 % de los pacientes**. Sin ella, en el 22 %.
> Más del doble.
>
> Y aquí al lado ya se probó. En 2018, el Digestivo del **Infanta Elena** usó un
> algoritmo para leer las derivaciones y adelantar las pruebas. Las revisiones
> por cada primera bajaron **de 2,25 a 1,72**. Pero solo llegó a **un 31 % de los
> pacientes**.
>
> ¿Por qué no se hace en todas partes? Porque alguien tendría que leer cada
> derivación antes de la cita y decidir qué prueba adelantar. Son 250 al día, en
> texto libre. Y lo que más falta en un hospital son personas.

### 3. Jano y la demo · UI/UX · 1:20

> Por eso hemos creado **Jano**. Cada noche lee las derivaciones de Primaria con
> **inteligencia artificial que funciona dentro del hospital**.
>
> *[Demo]* Esta es la derivación de María, tal como la escribe su médica. Jano
> saca la ficha: problema nuevo, cinco meses de evolución, y algo importante: la
> analítica **ya se la hicieron en Primaria**. Así que el protocolo de Digestivo
> propone solo la ecografía. No repetimos pruebas.
>
> Esta otra dice «perdió la cita de revisión». Jano la manda directa a revisión,
> sin pasar otra vez por el médico de cabecera.
>
> A esta le faltan datos: Jano se la devuelve a Primaria con la pregunta
> concreta.
>
> El especialista ve la semana entera en una pantalla, revisa y aprueba **con un
> clic**. María elige la hora de su ecografía desde el móvil, y llega a su
> primera consulta con el resultado.
>
> **Jano lee. El protocolo y el especialista deciden.**

### 4. Por qué es viable · Dev 1 · 2:50

> Se puede poner en marcha el mes que viene.
>
> **Uno:** IA local. Corre en un servidor del hospital. **Ningún dato de un
> paciente sale a la nube**, y no se paga por uso.
>
> **Dos:** IA pública española. Usamos modelos del Barcelona Supercomputing
> Center, entrenados con historias clínicas en español y con licencia libre.
> Funcionan sin GPU.
>
> **Tres:** la IA solo extrae datos. Lo clínico lo deciden los protocolos que
> aprueba cada servicio ⚠️.
>
> **Cuatro:** no añade personal. El especialista valida fichas en bloque en
> lugar de leer derivaciones una a una.
>
> **Cinco:** se prueba primero un mes en paralelo, sin decidir nada, y se mide
> cuánto acierta.
>
> **Seis:** lo puede mantener vuestra propia Unidad de Análisis de Datos.

### 5. Impacto y piloto · Dev 1 · 3:50

> Si un 30 % de las primeras consultas necesita una prueba ⚠️, y tenerla hecha
> sube la resolución del 22 al 49 %, son **unas 14.500 revisiones menos al año**
> en La Princesa. 4.800 horas de consulta: tres especialistas más sin contratar
> a nadie. **Un millón de euros al año** a precio público ⚠️. Y 14.500 viajes
> menos al hospital.
>
> Proponemos un piloto de tres meses en **Digestivo**, midiendo antes y después:
> - qué parte de las primeras se resuelve en una visita;
> - cuántas revisiones hay por cada primera;
> - los días hasta el diagnóstico;
> - las pruebas duplicadas.
>
> Y como cualquier hospital recibe derivaciones, el modelo es una licencia anual
> por hospital.

### 6. Cierre · Biomédica · 4:30

> Jano es **viable**, porque funciona dentro del hospital y con lo que ya tiene.
> Es **accesible**, porque cada viaje que ahorra pesa más para quien más le
> cuesta venir. Y es **personalizado**, porque prepara cada caso según su
> historia, sin repetir lo que ya se hizo.
>
> Solo os pedimos una cosa: **un servicio, tres meses y un equipo del tamaño de
> un libro encima de una mesa**. Gracias.

## De dónde sale cada cifra

| Frase | Cálculo y fuente |
|---|---|
| 294.000 revisiones, 1,65 por primera | Memoria 2022 de La Princesa: 294.369 sucesivas y 178.888 primeras |
| 49 % frente al 22 % | Estudio multicéntrico de 795 primeras visitas (*Rev Calid Asist*); en Digestivo, 57 % frente al 34 % ([HALLAZGOS](HALLAZGOS.md#7-la-puerta-de-entrada)) |
| 250 derivaciones al día | 35,35 % de las primeras vienen de Primaria (memoria 2022) = 63.237 al año ÷ unos 250 días laborables |
| 30 % de primeras con prueba | **Suposición**: en los CSV sintéticos es el 32 %. Hay que preguntar la cifra real ⚠️ |
| 14.500 revisiones menos | 178.888 × 30 % × (49 % − 22 %) = 14.490 |
| 4.800 horas, tres especialistas | 14.490 × 20 min = 4.830 h ÷ 1.540 h al año de consulta por especialista (7 h × 220 días; suposición) |
| Un millón de euros | 14.490 × 71 € (precio público de la sucesiva, Orden 727/2017 ⚠️). Es **valor de la actividad recuperada, no un ahorro de caja** |

Si os preguntan por los CSV: **son sintéticos**. Sirven para enseñar que Jano
funciona con la estructura de datos del hospital, no para medir el problema.
Decirlo antes de que lo pregunten da credibilidad.

## Lo que la demo tiene que enseñar

Si algo no llega a tiempo, se cambia el guion, no al revés.

- [ ] **La derivación de María**, en texto libre y con su ficha al lado: tipo,
      evolución, **analítica ya hecha en Primaria**, antecedentes.
- [ ] **El protocolo** propone solo la prueba que falta.
- [ ] **«Perdió la cita de revisión»** → directa a revisión.
- [ ] **Faltan datos** → devolución a Primaria con la pregunta.
- [ ] **La bandeja del especialista**: la semana entera, aprobar con un clic.
- [ ] **Contadores**: derivaciones leídas, pruebas adelantadas, duplicadas
      evitadas y casos enviados a una persona.
- [ ] Fichas **precalculadas** por si la IA falla en directo.

María y los demás son pacientes de los CSV sintéticos, con nombres inventados
para la demo.

## Preguntas que pueden hacer

### Dirección de La Princesa

- **«¿Y si la IA se equivoca?»** Solo extrae datos. Si no está segura, la
  derivación va a una persona. Y el especialista valida todo en bloque antes de
  que se pida ninguna prueba. Además, el primer mes funciona en paralelo, sin
  decidir nada.
- **«¿Qué servidor hace falta?»** Uno de dos equipos:
  - un **Mac mini M6**, para un servicio o un piloto pequeño (8–10 médicos con
    escriba);
  - un **NVIDIA DGX Spark, de unos 4.700 €**, para 30–50 médicos con escriba, o
    para todo el hospital si solo se usa Jano.

  Con **tres Spark, unos 14.000 € en total**, se cubren todas las consultas
  externas: unos 60 médicos a la vez, de los ~558 de plantilla. **Sin cuota por
  médico.** Son cifras estimadas, que se confirman con una prueba de carga antes
  del piloto ⚠️.
- **«¿Es difícil de instalarlo?»** Se instala como un router: se enchufa, se abre
  `jano.local` y se siguen tres pasos (conectar con HCIS, elegir el servicio y
  crear los usuarios). No necesita internet, y las actualizaciones van en un USB.
- **«¿No vamos a pedir pruebas de más?»** El protocolo solo incluye las pruebas
  que ese servicio pide casi siempre en ese motivo. Y Jano quita las que ya hizo
  Primaria, así que en total se piden menos. Las pruebas que no se usen se miden
  en el piloto.
- **«¿Cómo se integra con nuestro sistema?»** Por **HL7**, como cualquier otro
  sistema del hospital. Recibe los mensajes que HCIS ya envía (citas, pruebas,
  derivaciones, notas) y **no toca la base de datos de HCIS**. En el piloto solo
  lee y propone; escribir en HCIS es la fase 2 ⚠️.
- **«Ya vamos a tener un escriba.»** Perfecto: Jano recibe su nota por HL7 y
  hace todo lo que viene antes y después. Y si el escriba manda el audio a la
  nube, Jano trae uno que no saca nada del hospital.
- **«La Comunidad ya ha comprado un escriba.»** Sí, a T‑Systems, por 1,13 M€
  con IVA (BOCM, 7‑ago‑2026). Es **solo para Primaria** y va **en nube
  pública**: solo la nube estaba presupuestada en 400.000 € para tres meses.
  Ese escriba **escribe** la derivación en Primaria; **Jano la lee** en el
  hospital y prepara la consulta. Somos el paso siguiente, dentro de casa y con
  un equipo que se compra una vez. *No decir «lo mismo por menos»: casi 1 M€ de
  ese contrato son servicios de integración y despliegue.*
- **«¿Y la protección de datos?»** No sale nada del hospital. Además, un modelo
  del BSC anonimiza el texto antes de guardar ningún registro.

### Wayra

- **«¿Quién paga y cuánto?»** El hospital, con una licencia anual pequeña. Por
  ejemplo, 50.000 € al año se pagarían con unas 700 revisiones evitadas, el 5 %
  del escenario.
  - **Con el equipo incluido:** unos 55.000 € al año para todo el hospital,
    unos **15 € por médico al mes**.
  - **Un escriba comercial** cuesta entre **110 y 600 $ por médico al mes**,
    según estimaciones de terceros en EE. UU. ⚠️.
  - **Y Jano no solo escribe.**
- **«¿Competencia?» / «¿Esto no existe ya?»** Cada pieza existe por separado:
  - escribas en la nube;
  - chat con la historia clínica;
  - consultas de alta resolución hechas a mano;
  - el algoritmo del Infanta Elena de 2018.

  **Lo que no hemos encontrado es todo junto:** que lea el 100 % de las
  derivaciones, prepare la consulta y cierre la vuelta a Primaria, dentro del
  hospital, por HL7 y en un equipo que se instala solo. El Infanta Elena
  demostró que funciona (de 2,25 a 1,72 revisiones por primera), pero llegó a un
  31 % de los pacientes. *No decir nunca «no existe nada parecido»; decir «no lo
  hemos encontrado junto».*
- **«¿Cómo escala?»** Cada servicio carga su protocolo, y los modelos se
  reajustan con las derivaciones que el propio hospital ya etiqueta.
- **«¿Vais a seguir con esto?»** La respuesta tiene que ser sincera y estar
  pactada en el equipo antes de subir.

### Pfizer

- **«¿Qué tiene de medicina personalizada?»** Cada caso se prepara con su
  historia: lo que ya se hizo en Primaria, sus antecedentes y el motivo concreto.
  No es una cola igual para todos.
- **«¿Y los sesgos?»** La IA solo extrae datos del texto, y el piloto mide su
  acierto por edad y sexo. Si falla más en algún grupo, esos casos van a una
  persona.
- **«¿Por qué no un modelo grande en la nube?»** Por privacidad y por coste. La
  dirección del hospital lo quiere local, y los modelos clínicos españoles
  pequeños bastan para esta tarea.
