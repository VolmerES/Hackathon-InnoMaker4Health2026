# medicci-rs · el lector, en Rust

Binario único, sin runtime ni intérprete. Una **burbuja con un médico** vive
abajo a la derecha, encima del HCE. Al pulsarla se despliega el **chat**:
arriba se escribe el número o el ID del paciente (`417` o `PAC-00417`) y abajo
se pregunta. **Resumen** pone en el chat una tarjeta con lo que el lector ha
sacado de la historia, cada dato con su fuente, en cuatro pestañas: Resumen
(alertas, lo que no coincide con la derivación, valores clave, pendiente y lo
que no consta), Historial, Pruebas y Medicación. **Ampliar** la abre en una
ventana grande, en mitad de la pantalla, con tres columnas, la evolución de la
hemoglobina y un botón para copiarla a la nota clínica. Los botones de abajo
(Pruebas, Medicación, ¿Qué falta?) son atajos.

## Arrancar

Desde la raíz del repositorio, con [`just`](https://github.com/casey/just):

```
just              # revisa lo instalado, pasa las pruebas y abre la burbuja
just abre 417     # igual, con el paciente ya cargado
just rapido       # abre sin revisar
just compila      # deja el ejecutable y dice dónde está
```

`just revisa` comprueba Rust, el compilador de C (para SQLite), los SQL de la
base y las fuentes; en Linux también el escritorio y las bibliotecas de
ventana. Si falta algo, dice el comando para instalarlo.

### Windows

1. Rust: `winget install Rustlang.Rustup`
2. Las herramientas de C++ de Visual Studio (las pide Rust para enlazar):
   `winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`
3. Abrir otra PowerShell e instalar just: `cargo install just`
4. En la carpeta del repositorio: `just`.

Sale `medicci.exe`, que abre sin consola. La burbuja se coloca por encima de
la barra de tareas.

### Ubuntu

`cargo install just` (una vez) y `just`. Si falta algo, `just revisa` dice
qué instalar. En Wayland una ventana
no puede colocarse sola en la esquina, así que Medicci arranca sobre XWayland
por su cuenta.

## Conectada al servidor de Medici

La burbuja es la otra cara del servidor de `mvp/`. La pestaña web va encima de
HCIS cuando HCIS está en el navegador; la burbuja, cuando no lo está, porque se
pone encima de cualquier ventana. Las dos hablan con la misma API.

```
docker compose -f mvp/docker-compose.yml up -d     # el servidor (o python mvp/servidor.py)
just conectada                                      # la burbuja, conectada
just conectada 129798                               # con María ya abierta
```

Conectada, al escribir el NHC (`129798` vale por `P0129798`):

1. abre una sesión atada a ese paciente;
2. enseña la tarjeta **«antes de que entre»**, cada dato con su fila de origen;
3. las preguntas las contesta la IA local del servidor, citando las filas.

Cada 5 segundos vuelve a pedir la tarjeta: si llega algo por HL7 (una cita
anulada, un resultado), la burbuja lo dice. **RESUMEN** vuelve a enseñar la
tarjeta. Si el servidor no responde al arrancar, la burbuja sigue sola con su
base, como siempre.

| Fichero | Qué hace |
|---|---|
| `src/servidor.rs` | El cliente de la API (`ureq`), en otro hilo para no congelar la burbuja |

**Adónde llama:** `--servidor`, la variable `MEDICI_SERVIDOR` o la línea
`MEDICI_SERVIDOR=https://…` de un `medici.conf` al lado del ejecutable, que
pone Informática. Vale `http` o `https`.

Prueba contra un servidor en marcha:
`MEDICI_SERVIDOR=http://localhost:8000 cargo test --release -- --ignored`.

## Sin just

```
cargo test --release
cargo run --release -- --paciente 417
medicci --db ruta.db --paciente PAC-00417
```

## Qué hay dentro

| Fichero | Qué hace |
|---|---|
| `src/datos.rs` | La base: pacientes, documentos, hechos, campos de ficha y el registro de accesos |
| `src/lector.rs` | El lector: valores concretos, hechos por categoría, frases literales o «no consta» |
| `src/main.rs` | La interfaz: burbuja, chat y ventana de resumen |
| `fuentes/` | Noto Sans (licencia SIL OFL), incrustada en el binario |

`eframe`/`egui` para la interfaz, `rusqlite` con SQLite **incrustado** y
`regex`. El esquema y los datos de la demo van dentro del binario con
`include_str!`: **si la base no existe, la crea él solo**.

El lector responde en tres pasos: primero un **valor** («GOT de la última
analítica» → `GOT 23` con la línea literal), luego los **hechos** de esa
categoría («¿alergias?»), y si no, **frases** del texto que contengan lo
preguntado. Si no hay nada, dice que no consta. Cada dato lleva su documento.
Las pruebas comprueban que cada cita literal está de verdad en su documento.

Todo queda registrado en la tabla `accesos`: usuario, paciente, acción y hora.
