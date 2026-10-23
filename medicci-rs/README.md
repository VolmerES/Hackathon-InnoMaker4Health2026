# medicci-rs · el lector, en Rust

Binario único, sin runtime ni intérprete. Una **burbuja con un médico** vive
abajo a la derecha, encima del HCE. Al pulsarla se despliega el **chat**:
arriba se escribe el número o el ID del paciente (`417` o `PAC-00417`) y abajo
se pregunta. **RESUMEN** abre una ventana flotante, en mitad de la pantalla,
con lo que el lector ha sacado de la historia, cada dato con su fuente.

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

1. `winget install Rustlang.Rustup` y `winget install Casey.Just`
2. Las herramientas de C++ de Visual Studio (las pide Rust para enlazar):
   `winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`
3. Abrir otra PowerShell en la carpeta del repositorio y `just`.

Sale `medicci.exe`, que abre sin consola. La burbuja se coloca por encima de
la barra de tareas.

### Ubuntu

`just`. Si falta algo, `just revisa` dice qué instalar. En Wayland una ventana
no puede colocarse sola en la esquina, así que Medicci arranca sobre XWayland
por su cuenta.

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
