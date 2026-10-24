# La identidad de Medicci

Copiada del sistema de diseño de la presentación del pitch
([diapositivas](https://claude.ai/artifact/KRUedGyLcgfwakKtfA3Nbg)). Los valores
exactos están en [`tokens.json`](tokens.json); para las páginas web,
[`medicci.css`](medicci.css). La burbuja (`medicci-rs/`) usa los mismos.

## El nombre

Siempre **Medicci**, con mayúscula inicial y sin tilde. En la marca va en EB
Garamond seminegrita.

## El símbolo

[`isotipo.svg`](isotipo.svg): un bocadillo en verde claustro con una «m» que se
convierte en pulso, y el punto de oro al final. El bocadillo es la pregunta del
médico; el pulso, que la respuesta sale del paciente.
[`isotipo-app.svg`](isotipo-app.svg) es el mismo símbolo dentro del cuadrado en
verde Medicci, con la esquina del 23 % (30 sobre 128): solo para el icono de la
aplicación.

## Colores

| Nombre | Claro | Oscuro | Para qué |
|---|---|---|---|
| fondo | `#F6F1E6` | `#0B2F2B` | Fondo de página. Marfil: el papel de un informe, no el blanco de un quirófano |
| fondo-alt | `#ECE4D3` | `#0E3B36` | Tarjetas, filas de fuente |
| tinta | `#16211F` | `#F6F1E6` | Texto principal (14,7:1) |
| tinta-suave | `#4B5A57` | `#BFD3CE` | Texto secundario y metadatos |
| **verde Medicci** | `#0E3B36` | `#1F6B61` | El color de la marca: símbolo, portada, barra superior. Encima, marfil u oro |
| verde claustro | `#1F6B61` | `#6FB3A6` | Acento: enlaces, botón principal, foco, la pregunta del médico |
| salvia | `#CFE0D9` | `#1A4A44` | Fondo de la respuesta de Medicci y de las cifras |
| **oro** | `#D4AF37` | `#D4AF37` | El punto de la marca. Solo como forma o sobre verde Medicci; nunca texto sobre marfil |
| oro hondo | `#7A5C0E` | `#E2C25E` | Antetítulos y el ¹ de fuente sobre fondos claros |
| terracota | `#A34A24` | `#E08A62` | Solo «No consta» y avisos, siempre con la palabra, nunca solo el color |
| línea | `#D9CFBA` | `#24524C` | Filetes de 1 px y bordes de tarjeta |

## Letra

| Uso | Familia | Muestra |
|---|---|---|
| Marca y titulares | EB Garamond | «Pregunta, no busques.» |
| Texto y respuestas | IBM Plex Sans | «Sí. Tuvo una primera consulta en Digestivo el 14 de marzo.» |
| Etiquetas | IBM Plex Sans seminegrita, mayúsculas, +0,08 em | FUENTE · CITAS |
| La fila de origen | IBM Plex Mono | `CITAS · id 48213 · 2026-03-14 · Digestivo · realizada` |

Las fuentes están en `medicci-rs/fuentes/` (licencia OFL: `LICENCIA-IBM-Plex.txt`
y `LICENCIA-EB-Garamond.txt`). El servidor las sirve en `/fuentes/`.

## Medidas

Espacios de 4, 8, 16, 24, 40 y 64 px. Esquinas: 6 px (etiquetas, filas de
fuente), 12 px (burbujas, botones, tarjetas), 20 px (paneles).

## Dónde se usa

- **La burbuja** (`medicci-rs/`): barra superior y círculo en verde Medicci con el
  isotipo, fondo marfil, avisos en terracota, Plex Sans y el nombre en Garamond.
- **El registro de accesos y los mensajes HL7** (`mvp/web/`).
- **Los rótulos de la simulación** (`mvp/web/simulacion.js`).
- **No en la copia de HCIS** (`mvp/web/index.html`): es HCIS, no Medicci.
