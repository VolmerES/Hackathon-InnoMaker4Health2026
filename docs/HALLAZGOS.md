# Hallazgos en los datos

Primera exploración de los 11 CSV, hecha el 23‑oct. Los números salen de
[`analisis/`](../analisis/): `perfil.py` (columnas y valores) y `hallazgos.py` y
`hallazgos2.py` (las cifras de abajo).

**Periodo:** citas del 1‑ene al 30‑jun de 2025, es decir, **6 meses** en 6
especialidades (CAR, DER, DIG, NRL, OFT, TRA) y 140 agendas. Los datos son
sintéticos: las cifras sirven para dimensionar el problema, no son de La
Princesa.

## Resumen

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

- **El mayor robo de tiempo clínico son los huecos vacíos**: las faltas y los
  huecos liberados que nadie reocupa. Son 15.000 horas de consulta en 6 meses y
  se pueden anticipar con una regla.
- **«Revisiones sin resultado» se descarta como núcleo**, porque los datos no la
  respaldan.
- **El producto:** el día antes de la cita, la bandeja enseña las citas sin
  confirmar y los huecos liberados. Propone un segundo contacto y, si no hay
  respuesta, ofrecer el hueco a quien espera, por prioridad clínica.
- **Escenario de impacto (hipótesis, para el pitch):** si se recupera la mitad de
  las faltas sin respuesta (16.000 en 6 meses), salen **unas 5.400 horas de
  consulta** cada 6 meses y menos gestión de no realizadas. Si se recupera una
  cuarta parte, unas 2.700 horas.
