# Innomakers4Health 2026 · Consultas externas

Proyecto para el hackathon **Innomakers4Health 2026** (Fundación Pfizer, Pfizer y
Wayra, con el Hospital Universitario de La Princesa). Se celebra el 23 y el 24
de octubre de 2026 en 42 Madrid · Fundación Telefónica.

## El reto

> ¿Cómo podríamos **liberar tiempo clínico** reduciendo tareas administrativas y
> mejorando procesos para optimizar la experiencia de pacientes y profesionales
> en **consultas externas**?

Para trabajarlo hay un conjunto de **datos sintéticos** en 11 CSV que simula el
circuito de consultas externas. Empieza en la derivación desde Atención
Primaria y sigue con la agenda y los huecos, la cita y sus cambios, las
comunicaciones con el paciente, el día de la consulta, las pruebas y las tareas
administrativas.

## Los tres pilares

1. **Viabilidad**: se puede pilotar en un hospital real, con los datos y los
   sistemas que ya tiene.
2. **Accesibilidad**: funciona también para quien peor lo tiene para llegar a la
   consulta.
3. **Medicina personalizada**: cada paciente recorre su circuito según su caso.

Qué significa cada uno en la práctica:
[`docs/CONTEXTO.md`](docs/CONTEXTO.md#los-tres-pilares-del-proyecto).

## Contexto

El contexto completo, con el modelo de datos transcrito, está en
**[`docs/CONTEXTO.md`](docs/CONTEXTO.md)**.

## Estructura

```text
docs/
  CONTEXTO.md     El reto, los datos, el modelo entidad-relación y a quién preguntar
  briefing/       Fotos del material impreso que entregó la organización
  HALLAZGOS.md    Lo que dicen los datos, con cifras
analisis/         Scripts de exploración (Python + DuckDB)
data/             Los 11 CSV (no se suben: son material de la organización)
```

Para repetir el análisis, con los CSV en `data/`:

```bash
python -m venv .venv
.venv/Scripts/python -m pip install -r requirements.txt   # en Linux/macOS: .venv/bin/python
.venv/Scripts/python analisis/hallazgos.py
```

## Estado

- [x] Contexto del reto y modelo de datos
- [x] Ganadores de ediciones anteriores y [dos ideas candidatas](docs/CONTEXTO.md#ideas-candidatas)
- [x] Qué le interesa a cada organizador y [plan del MVP en 24 h](docs/CONTEXTO.md#el-mvp-en-24-horas)
- [x] Cargar y explorar los 11 CSV: [hallazgos](docs/HALLAZGOS.md)
- [x] ~~Los huecos vacíos~~: aparcado, porque media sala va a por las faltas. Sus cifras reales siguen en [HALLAZGOS](docs/HALLAZGOS.md)
- [x] **Idea decidida: Medici, el chatbot copiloto del médico.** El médico le pregunta en el ordenador por el paciente y Medici le contesta, con la fuente de cada dato y con IA local. Ver [CONTEXTO](docs/CONTEXTO.md#la-idea-decidida-medici-el-copiloto-del-médico)
- [x] Hardware decidido: **Mac mini M6** o **NVIDIA DGX Spark**, con una instalación muy sencilla. [Propuesta de infraestructura para todo el hospital](docs/CONTEXTO.md#infraestructura-para-todo-el-hospital-propuesta-de-claude-pendiente-del-equipo). Ver [CONTEXTO](docs/CONTEXTO.md#dónde-corre-medici-mac-mini-m6-o-nvidia-dgx-spark)
- [x] Ideas que no entran (preparar la consulta, escriba, HL7…): en la [hoja de ruta](docs/CONTEXTO.md#hoja-de-ruta-ideas-no-decididas)
- [ ] MVP: agente con herramientas de solo lectura sobre los CSV, modelo local, pantalla con agenda, chat y fuentes, y prueba de acierto
- [ ] Medir el impacto: minutos liberados, pasos ahorrados
- [x] [Guion del pitch de 5 minutos](docs/PITCH.md), con las preguntas probables del jurado
- [ ] Confirmar con los mentores lo marcado con ⚠️ en el guion
- [ ] Ensayar el pitch con cronómetro

## Aviso

Todos los datos son sintéticos. Ningún dato de pacientes reales pasa por este
repositorio.
