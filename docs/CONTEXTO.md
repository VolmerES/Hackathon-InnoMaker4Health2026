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

## Fuentes

- [Fundación Pfizer · Innomakers4Health](https://www.fundacionpfizer.org/tecnolog%C3%ADa/innomakers4health)
- [Wayra · anuncio del reto](https://wayra.com/news/wayra-pfizer-and-the-pfizer-foundation-launch-a-challenge-to-optimise-efficiency-in-hospital-outpatient-clinics)
- [La Ecuación Digital](https://www.laecuaciondigital.com/empresas/healthtech/wayra-pfizer-reto-consultas-externas-hospitalarias/)
