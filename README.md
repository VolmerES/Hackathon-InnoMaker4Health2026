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
- [x] Necesidad concreta: **los huecos vacíos**. El 28 % de las citas no se realiza (15.254 h de consulta en 6 meses) y los huecos liberados no se reocupan nunca
- [ ] MVP: bandeja de trabajo (citas en riesgo y huecos que se reaprovechan)
- [ ] Medir el impacto: minutos liberados, pasos ahorrados
- [ ] Pitch

## Aviso

Todos los datos son sintéticos. Ningún dato de pacientes reales pasa por este
repositorio.
