# Hallazgos en los datos

Primera exploración de los 11 CSV, hecha el 23‑oct. Los números salen de
[`analisis/`](../analisis/): `perfil.py` (columnas y valores) y `hallazgos.py` y
`hallazgos2.py` (las cifras de abajo).

**Periodo:** citas del 1‑ene al 30‑jun de 2025, es decir, **6 meses** en 6
especialidades (CAR, DER, DIG, NRL, OFT, TRA) y 140 agendas.

> **Los CSV son datos falsos (sintéticos).** Sirven para construir y enseñar la
> herramienta con la misma estructura que tendrían los datos del hospital. **No
> sirven para dimensionar el problema.** Varias de las cifras de abajo son,
> casi seguro, la forma en que se generaron los datos y no algo que pase en La
> Princesa. Para el pitch hay que usar las
> [cifras reales publicadas](#cifras-reales-para-el-pitch).

## Qué vale y qué no de estos datos

**Señales de que los datos están generados:**

- Las 140 agendas son idénticas: citas de 20 minutos, de 8:00 a 15:00 y 21
  huecos al día.
- El recordatorio sale siempre **exactamente** 2 días antes y siempre por el
  canal preferido.
- La tasa de faltas es del 28 % **en todo**: edad, sexo, accesibilidad, canal.
  En la realidad varía según el perfil del paciente.
- La prioridad no cambia la espera: un urgente de Dermatología espera lo mismo
  que uno ordinario (97 días).
- Ningún hueco liberado se reocupa nunca y ninguna oferta de adelanto se acepta.
  Lo más probable es que el generador no simule esos casos, no que el hospital
  falle en el 100 % de ellos.
- Las pruebas dejan de tener fecha de realización el 31‑mar.

**Cómo usar cada cifra:**

| Cifra | ¿Se puede usar? |
|---|---|
| 28 % de citas no realizadas | **No.** En España se publica entre el 10 y el 15 % (ver abajo) |
| No contestar al recordatorio: 46 % de faltas frente al 13 % | **Como idea, sí; como cifra, no.** La dirección es lógica, pero el tamaño lo puso el generador |
| 0 huecos liberados reocupados, 0 adelantos aceptados | **No como hallazgo.** Hay que preguntar a los mentores cómo se reaprovechan hoy los huecos en La Princesa |
| Reparto de horas de tareas administrativas | Solo como orden de magnitud entre tipos de tarea |
| Estructura (tablas, estados, canales, tipos de tarea) | **Sí.** Es lo que la herramienta tiene que leer |

## Cifras reales para el pitch

| Dato | Cifra | Fuente |
|---|---|---|
| Consultas externas de La Princesa al año | **473.257** (178.888 primeras + 294.369 sucesivas) | [Memoria 2022 del hospital](https://www.comunidad.madrid/docs/assets/2023/11/29/memoria_2022_hulp_ok.pdf) |
| Faltas a consultas externas en España | **10‑15 %**: 14,6 % en Albacete; 14,2 % en el Hospital Costa del Sol; 12,5 % en Medicina Preventiva de Zaragoza | Pereira‑Victorio et al., *Med Gen Fam* 2016; Costa del Sol y Zaragoza en *Rev Calid Asist* / *J Healthc Qual Res* |
| **Casi la mitad de las faltas tiene causa administrativa** | En Albacete: no sabía que tenía cita (16,1 %), tenía doble cita (15,6 %), había cancelado pero la cita seguía en el sistema (15,1 %). En total, **47 %**. Los autores atribuyen a la administración cerca del 60 % | [Pereira‑Victorio et al. 2016](https://www.elsevier.es/es-revista-medicina-general-familia-edicion-digital--231-articulo-absentismo-pacientes-consulta-externa-especializada-S1889543316000372) |
| Coste | Más de **3 M€ al año** en la Agencia Sanitaria Costa del Sol (13,8 % de faltas). Unos 3,6 M€ al año proyectados en Albacete | [iSanidad](https://isanidad.com/93355/no-acudir-a-las-citas-medicas-genera-perdidas-millonarias-para-la-sanidad/), Pereira‑Victorio et al. 2016 |
| Efecto de un recordatorio simple | Pequeño: **la asistencia mejora un 10‑14 %** (RR 1,10‑1,14) | [Revisión Cochrane](https://cochranelibrary.com/es/cdsr/doi/10.1002/14651858.CD007458.pub2/full) |

**Por qué esto refuerza la propuesta:**

- **El problema es administrativo, justo lo que pide el reto.** Las tres
  primeras causas en Albacete son fallos del circuito de citación, no del
  paciente: no le llegó la cita, tenía dos o su cancelación no se registró.
- **Un recordatorio solo no basta**, porque mejora poco la asistencia. Lo que
  falta es **cerrar el circuito**: que alguien vea quién no ha confirmado, que
  las cancelaciones liberen el hueco y que ese hueco se ofrezca.
- **Las propuestas del propio estudio de Albacete son lo que hace la bandeja:**
  - confirmar la cita antes de que llegue;
  - verificar anulaciones y citas dobles;
  - recoger las preferencias del paciente;
  - sobreasignar alrededor de un 10 % según el absentismo previo.

**Escenario para La Princesa** (hipótesis con cifras públicas):

| | Con un 12 % de faltas | Con un 15 % de faltas |
|---|---:|---:|
| Citas perdidas al año | 56.800 | 71.000 |
| Horas de consulta perdidas (con 20 min de media) | 18.900 | 23.700 |
| De ellas, con causa administrativa (47 %) | 26.700 citas | 33.400 citas |
| **Si la bandeja recupera una cuarta parte de las faltas** | **14.200 citas · 4.700 h** | **17.700 citas · 5.900 h** |

Los 20 minutos por cita son una suposición: hay que preguntar la duración real
de las primeras y las sucesivas. La tasa real de La Princesa la puede dar la
Unidad de Datos.

## Resumen de los CSV

Cifras de los datos sintéticos. Ver arriba qué vale de cada una.

1. **El 28 % de las citas programadas no se realiza.** Son 45.763 en 6 meses, es
   decir, **15.254 horas de consulta perdidas**, más 7.000 horas de trabajo
   administrativo para gestionarlas.
2. **Hay una señal sencilla que lo anticipa.** Si el paciente **no contesta al
   recordatorio**, falta el **45,8 %** de las veces; si confirma, el 13 %. Ahí
   está el **71 % de todas las faltas**, y se sabe **2 días antes**.
3. **Los huecos que se liberan no se vuelven a ocupar nunca.** Hay 24.928 huecos
   liberados por anulación o reprogramación y **ninguno** se reutilizó, aunque el
   92 % se liberó con 2 días o más de margen. Mientras, Dermatología espera una
   mediana de **98 días** y Oftalmología, 75.
4. **Las ofertas de adelanto actuales no funcionan.** Se enviaron 6.220 y ninguna
   se aceptó; todos esos huecos siguen libres.
5. **La idea «revisiones sin resultado» no se sostiene con estos datos.** Solo el
   1,5 % de las pruebas está enlazado a su revisión, y en todos esos casos la
   prueba ya estaba hecha.

## 1. Citas no realizadas

| Tipo de visita | Citas | No realizadas |
|---|---:|---:|
| Primera (P) | 41.434 | **33,3 %** |
| Sucesiva (S) | 98.786 | 26,9 % |
| Técnica (T) | 22.627 | 23,8 % |
| **Total** | **162.847** | **28,1 %** (45.763) |

No se cuentan las anuladas (9.354) ni las de estado `URGENTE` (67.720), que
entran por Urgencias y no pasan por la agenda programada.

**Coste en 6 meses:**

- **15.254 horas de consulta** perdidas (45.763 huecos de 20 minutos).
- **3.176 horas** de `GESTION_NO_REALIZADA`, que no es automatizable.
- **3.825 horas** de `RECONTACTO_PACIENTE` por contactos fallidos, que tampoco es
  automatizable.

## 2. La señal: la respuesta al recordatorio

El recordatorio sale siempre **2 días antes** de la cita.

| Respuesta al recordatorio | Citas | Faltan | Parte de todas las faltas |
|---|---:|---:|---:|
| Confirma | 82.078 | 13,0 % | 23,2 % |
| **Sin respuesta, entregado** | **70.601** | **45,8 %** | **70,6 %** |
| Sin respuesta, envío fallido | 8.092 | 28,9 % | 5,1 % |
| Sin recordatorio | 2.076 | 22,8 % | 1,0 % |

La tasa es la misma por cualquier canal (APP, SMS, email, llamada, carta: entre
el 45 y el 47 % sin respuesta). **No hace falta un modelo de IA**: basta una
regla que cualquiera entiende y que se puede auditar.

**Lo que no discrimina en estos datos:** la edad, el sexo, la accesibilidad, el
canal preferido y si la cita se reprogramó. Todo ronda el 28 %. El recordatorio
se envía siempre por el canal preferido del paciente. Es probable que sea
uniforme porque los datos son sintéticos: hay que preguntarlo a la Unidad de
Datos.

La demora sí influye un poco: faltan el 23,9 % con menos de 15 días desde la
asignación y el 29,6 % con más de 120.

## 3. Huecos liberados que se pierden

| Motivo | Liberados | Con 2 días o más de margen | Reocupados |
|---|---:|---:|---:|
| Anulación | 9.354 | 9.034 | **0** |
| Reprogramación | 15.574 | 13.779 | **0** |

En `OCUPACIONES_HUECO`, ningún hueco tiene más de una ocupación en toda su
historia.

En total hay **141.717 huecos libres** (el 37 %), mientras las primeras consultas
esperan:

| Especialidad | Mediana de días | P90 |
|---|---:|---:|
| Dermatología | **98** | 194 |
| Oftalmología | **75** | 150 |
| Traumatología | 50 | 99 |
| Digestivo | 44 | 88 |
| Cardiología | 36 | 71 |
| Neurología | 35 | 71 |

Además hay 2.216 derivaciones en `PENDIENTE_CITA`.

**La prioridad apenas cambia la espera.** Una derivación `URGENTE` de
Dermatología espera 97 días, igual que una ordinaria. Es otro punto de
«medicina personalizada»: ordenar la espera por necesidad clínica.

## 4. Ofertas de adelanto

Se enviaron 6.220 ofertas (`OFERTA_ADELANTO`): 3.556 se rechazaron, 2.664 no
tuvieron respuesta y **0 se aceptaron**. Todos los huecos ofertados siguen
`LIBRE`. La mayoría se envió con 7 días o más de margen, así que el problema no
es la antelación.

Los avisos de cambio de cita sí funcionan: el 95 % se acepta.

## 5. Tareas administrativas

37.893 horas en total, de las que el **77 % está marcado como automatizable**.

| Tarea | Horas | Media (min) | Automatizable |
|---|---:|---:|---|
| Citación | 14.601 | 3,7 | sí |
| Admisión | 5.075 | 2,6 | sí |
| Reprogramación | 4.259 | 6,8 | sí |
| **Recontacto con el paciente** | **3.825** | 8,8 | **no** |
| Validación de derivación | 3.206 | 4,2 | sí |
| **Gestión de la no realizada** | **3.176** | 4,2 | **no** |
| **Llamada de aviso de cambio** | **1.801** | 5,8 | **no** |
| Solicitud de prueba (enfermería) | 1.624 | 3,7 | sí |
| Anulación | 324 | 2,1 | sí |

Las tres tareas que **no** son automatizables (8.800 horas) están ligadas a las
faltas y a los cambios de cita.

## 6. El día de la consulta

Espera media de **14,7 minutos**, consulta de **20 minutos** y 3,5 minutos de la
llegada a la admisión. Aquí no hay un problema grande.

## Qué significa para la propuesta

- **Los huecos vacíos siguen siendo el problema**, pero su tamaño sale de las
  cifras reales (10‑15 % de faltas, la mitad por causas administrativas) y no
  del 28 % de los CSV.
- **Los CSV son el banco de pruebas**: la demo enseña que la bandeja funciona
  con la estructura de datos del hospital.
- **«Revisiones sin resultado» se descarta como núcleo**, porque los datos no la
  respaldan.
- **El producto:** el día antes de la cita, la bandeja enseña las citas sin
  confirmar, las posibles citas dobles y las cancelaciones que no han liberado
  su hueco. Propone un segundo contacto y, si no hay respuesta, ofrecer el hueco
  a quien espera, por prioridad clínica.
- **Las citas dobles se pueden detectar en los CSV:** hay movimientos
  «Duplicidad detectada» (1.821) y «Error de citación» (1.861). Encaja con la
  segunda causa de Albacete.
