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
**[`docs/CONTEXTO.md`](docs/CONTEXTO.md)**. Empieza por su
[estado en un minuto](docs/CONTEXTO.md#estado-en-un-minuto-24oct).

## Arrancar Medicci con un clic

Solo hace falta **Docker Desktop** y los 11 CSV del reto en `data/` (no van en
GitHub).

- **Windows:** doble clic en **`arrancar.bat`**.
- **Linux:** `./arrancar.sh`.

Levanta el modelo de IA local y el servidor, descarga el modelo la primera vez
(unos 5 GB), manda la historia de la agenda por HL7, comprueba que las
tarjetas salen bien y abre el navegador con María. Sin GPU de NVIDIA también
funciona, pero más lento. Con el modelo ya descargado tarda **unos 30
segundos**.

**Dónde está cada cosa:** en una licitación, dónde se instala Medicci lo decide
quien licita (el CPD del hospital, el de la Comunidad, la nube del pliego).
Todas las direcciones van en **un único fichero, `medicci.conf`**: el modelo de
IA (con su clave si la pide), dónde escucha el servidor y el receptor HL7, y
adónde llaman la burbuja y el emisor. Se copia de
[`medicci.conf.ejemplo`](medicci.conf.ejemplo), se cambian las líneas y no se
toca código. Una variable de entorno con el mismo nombre manda sobre el
fichero.

La burbuja de escritorio, conectada al mismo servidor: `just conectada 129798`
(ver [`medicci-rs/`](medicci-rs/README.md)).

## Estructura

```text
docs/
  CONTEXTO.md     El reto, los datos, el modelo entidad-relación y a quién preguntar
  briefing/       Fotos del material impreso que entregó la organización
  HALLAZGOS.md    Lo que dicen los datos, con cifras
analisis/         Scripts de exploración (Python + DuckDB)
mvp/              Núcleo de Medicci: chat sin agente, tarjeta «3 cosas» y sus pruebas
demo/             Réplicas de HCIS y SELENE con historias sintéticas (Daniel)
medicci-rs/       La burbuja de escritorio: el asistente que ve el médico
marca/            La identidad de Medicci: colores, letra, isotipo y reglas (IDENTIDAD.md)
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
- [x] **Idea decidida: Medicci, el chatbot copiloto del médico.** El médico le pregunta en el ordenador por el paciente y Medicci le contesta, con la fuente de cada dato y con IA local. Ver [CONTEXTO](docs/CONTEXTO.md#la-idea-decidida-medicci-el-copiloto-del-médico)
- [x] Hardware decidido: **Mac mini M6** o **NVIDIA DGX Spark**, con una instalación muy sencilla. [Propuesta de infraestructura para todo el hospital](docs/CONTEXTO.md#infraestructura-para-todo-el-hospital-propuesta-de-claude-pendiente-del-equipo). Ver [CONTEXTO](docs/CONTEXTO.md#dónde-corre-medicci-mac-mini-m6-o-nvidia-dgx-spark)
- [x] Ideas que no entran (preparar la consulta, escriba, HL7…): en la [hoja de ruta](docs/CONTEXTO.md#hoja-de-ruta-ideas-no-decididas)
- [x] Chat sin agente medido con Qwen3‑8B: **29/30** con pacientes nuevos, 0 fuentes inventadas, 0,7 s ([`mvp/`](mvp/README.md))
- [x] Tarjeta «3 cosas antes de que entre»: alertas por reglas, 0 errores en 2.000 pacientes
- [ ] Servidor y [pestaña dentro de HCIS](docs/CONTEXTO.md#dentro-de-hcis-la-pestaña-medicci-propuesta-pendiente-del-equipo) sobre la réplica de Daniel
- [ ] Medir el impacto: minutos liberados, pasos ahorrados
- [x] [Guion del pitch de 5 minutos](docs/PITCH.md), con las preguntas probables del jurado
- [ ] Confirmar con los mentores lo marcado con ⚠️ en el guion
- [ ] Ensayar el pitch con cronómetro

## Aviso

Todos los datos son sintéticos. Ningún dato de pacientes reales pasa por este
repositorio.
