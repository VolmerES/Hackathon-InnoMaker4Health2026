# demo/

La demo de **Medici** (antes «Jano») para el pitch: informes de derivación sintéticos y la
pantalla con la que el especialista los valida.

```text
derivaciones/   10 hojas de derivación de Primaria, texto libre tal como lo escribe el médico
fichas/         reservado para las fichas en JSON si se generan con la IA local
web/
  historial.html       la historia en un HCE tipo SELENE
  hcis.html            la misma historia en una pantalla tipo HCIS
  agente.js/.css       el agente flotante, compartido por las dos pantallas
  historial-datos.js   paciente 1 (María Alonso): 18 documentos, 2015-2026
  historial-datos-2.js paciente 2 (Ramón Esteve): 14 documentos, 2012-2026, y los hallazgos clave
  historial-ficha.js   la ficha de contexto de 3 capas que muestra el agente
  index.html           la bandeja de validación de derivaciones (la otra pantalla)
  datos.js             los 10 casos de la bandeja
```

## La pantalla principal: `web/historial.html`

Imita el HCE del hospital (tipo **SELENE**): barra de título, menú, barra de
herramientas, **carpeta del paciente** en árbol, cintillo azul con NHC, CIPA,
cita de hoy y alergias, pestañas de Evolutivo / Documentos / Resultados y barra
de estado.

Abajo a la derecha, **el agente**, como el chat de cualquier web: botón redondo
con el icono del médico y, al abrirlo, el lector. El HCE queda intacto detrás:
es la idea, que Medici se acople al sistema que ya usan, no que lo sustituya.

Dos pacientes en el desplegable de arriba, los dos citados hoy en Digestivo:

| | Paciente | Documentos | Lo que esconde la historia |
|---|---|---|---|
| 1 | María Alonso Vega, 58 a · dispepsia de 5 meses | 18 · 2015-2026 | Alergia a metamizol, histerectomía y AINE que la derivación niega; adenoma de 2015 con control nunca hecho; sigue tomando el IBP, así que la «falta de respuesta» no es valorable |
| 2 | Ramón Esteve Gálvez, 72 a · anemia ferropénica | 14 · 2012-2026 | Angiodisplasia gástrica sangrante en 2019 («no antecedentes digestivos», dice la derivación); anticoagulado; Adiro sin indicación; alergia al yodo; 6 kg perdidos y deposiciones oscuras anotados solo por enfermería |

Las tres vistas: **Evolutivo** (todos los documentos seguidos, del más reciente
al más antiguo), **Documentos** (rejilla arriba, documento debajo) y
**Resultados** (analíticas, endoscopias y radiología). El árbol filtra por tipo,
por año y por episodio, y el buscador resalta la palabra dentro del texto.

### El agente (botón de abajo a la derecha)

- **Resumen de entrada**: al abrirlo da el motivo de hoy, lo que le pasa, las
  contradicciones con la derivación y lo que falta, cada línea con la frase
  literal y un botón que salta al documento y lo resalta.
- **Preguntas**: las sugeridas o escritas a mano. Si la respuesta no está en la
  historia contesta «no consta» en vez de deducir.
- **Generar ficha de contexto**: produce la ficha de 3 capas (control crítico y
  contexto de vida · contexto clínico dinámico · historial profundo) con la
  fuente de cada campo y «no consta» donde la historia no lo tiene.

La ficha completa de cada paciente, en markdown, está en
[`fichas/`](fichas/): `ficha-contexto-P1.md` (114 campos revisados, 64 con dato)
y `ficha-contexto-P2.md` (92 revisados, 59 con dato). En pantalla se muestra la
versión resumida de `web/historial-ficha.js`.

Los hallazgos clave de cada paciente están en `historial-datos-2.js`
(`window.HALLAZGOS`): la lista contra la que comparar lo que saca un médico
leyendo, para cuando se monte esa parte.

## Cómo está escrito el texto

Las hojas imitan el registro real: el encabezado lo imprime el HIS (son los
campos de `DERIVACIONES_AP`) y debajo va `motivo_derivacion` y
`observaciones_medico` **tal cual los escribe un médico de familia**: minúsculas,
sin tildes, frases telegráficas, siglas (AP, AF, EF, NAMC, IBP, EC, FID, SOH,
tto, pcte, s/p, rgo), analíticas pegadas en una línea, un caso en mayúsculas
enteras y otro casi ilegible. Nada de apartados ordenados: eso es lo que hace
difícil leer 250 al día y es justo lo que Medici tiene que digerir.

El registro está tomado de lo que describe la literatura sobre historias
clínicas españolas (notas escritas deprisa y con gran número de abreviaturas;
en informes reales las siglas más frecuentes son HTA, AP y SC, y hasta un 27 %
pueden ser de uso dudoso) y de los corpus clínicos públicos del Barcelona
Supercomputing Center (SPACCC, MEDDOCAN). **Ningún texto es de un paciente
real: están escritos para la demo.**

**Todo es sintético.** Nombres, centros de salud, facultativos y cifras están
inventados a partir de la estructura de `DERIVACIONES_AP`. Cada hoja lo dice en
su encabezado. No hay ningún dato de un paciente real.

## Abrir

```bash
xdg-open demo/web/historial.html   # historia en SELENE + agente
xdg-open demo/web/hcis.html        # la misma historia en HCIS + agente
xdg-open demo/web/index.html       # bandeja de validación de derivaciones
```

Las dos pantallas llevan arriba un enlace a la otra, así que cada médico puede
mirar la que use en su hospital. Mismos pacientes, mismos documentos y el mismo
agente: lo único que cambia es el HCE de debajo.

**HCIS** está recreada de una captura real: cinta de paciente con NHC y
episodio, columna verde de Alertas / Historia Clínica / Relevante, barra de
iconos, pestañas (Consulta, Historia Clínica, Monográficos, Pedidos Recientes,
Preoperatorio, Prescripción) y la *Pantalla historia clínica de Digestivo* con
sus secciones plegables: datos generales, otras visitas, motivo de consulta,
antecedentes, historia actual, constantes, exploración, escalas clínicas, lista
de problemas, peticiones con «Mis favoritos» y pruebas complementarias
(laboratorio, imagen, otros), con los botones morados de abajo. Las secciones
vienen **rellenas con la historia del paciente**: el motivo trae la derivación
tal cual, historia actual las tres últimas notas, laboratorio la última
analítica e imagen el último informe.

**SELENE** es una reconstrucción por rasgos conocidos (no hay manual ni
capturas públicas): barra de título, menú, carpeta del paciente en árbol,
cintillo azul y pestañas de evolutivo.

Funciona en `file://`, sin red ni servidor: es lo que hay que enseñar en el
escenario. Las fichas vienen **precalculadas** en `datos.js`, así que la demo no
depende de que el modelo responda en directo.

Atajos: `j` y `k` para moverse por la lista, `a` para aprobar.

## Qué enseña cada caso

| Derivación | Paciente | Lo que demuestra |
|---|---|---|
| 04871 | María Alonso | El caso del guion: analítica y H. pylori ya hechos en Primaria, solo falta la ecografía |
| 04872 | Antonio Ruiz | «Perdió la cita de revisión» → revisión directa, sin primera consulta |
| 04873 | Lucía Fernández | Faltan datos → se devuelve a Primaria con la pregunta concreta |
| 04874 | Rosa Iglesias | Datos de alarma → colonoscopia preferente antes de la consulta |
| 04875 | Iván Beltrán | Empeoramiento de una enfermedad conocida → revisión preferente con su médico |
| 04876 | Pilar Sanchís | Ecografía ya hecha fuera del hospital: no se repite, se añade la elastografía |
| 04877 | Jesús Carvajal | Bandera roja (disfagia y 6 kg): no se encamina solo, va a una persona |
| 04878 | Nuria Peláez | Tres pruebas duplicadas evitadas; solo queda la gastroscopia |
| 04879 | Mohamed Benali | Texto telegráfico: confianza 0,41, por debajo del umbral → a una persona |
| 04880 | Carmen Dorado | Revisión de siempre + accesibilidad: silla de ruedas, un solo viaje |

Cuatro de los diez no acaban en una primera consulta nueva, y en tres se
evitan pruebas que ya estaban hechas.

## La idea de la pantalla

Dos columnas enfrentadas: a la izquierda la hoja
tal como la escribió Primaria, a la derecha la ficha. Al pasar por encima de un
campo de la ficha se resalta **la frase del texto de la que sale**. Es la forma
de dejar ver que la IA solo extrae: cada dato tiene su origen a la vista, y lo
clínico lo propone el protocolo del Servicio que firma arriba.
