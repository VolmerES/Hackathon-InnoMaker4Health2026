# Medicci · atajos
#
#   just              revisa que todo está instalado, pasa las pruebas y abre la burbuja
#   just abre 417     igual, con el paciente ya cargado
#   just rapido       abre sin revisar (cuando ya se ha revisado)
#   just conectada    abre conectada al servidor de Medicci (mvp/servidor.py en marcha)
#   just compila      deja el ejecutable listo y dice dónde está
#
# Para tener just: cargo install just
# En Windows funciona desde PowerShell; en Ubuntu, desde cualquier terminal.

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]
set working-directory := "medicci-rs"

exe := if os() == "windows" { "medicci-rs\\target\\release\\medicci.exe" } else { "medicci-rs/target/release/medicci" }

default: abre

# Revisa herramientas, ficheros y pruebas; si algo falta, dice cómo instalarlo
revisa: _sistema
    cargo test --release --quiet
    @echo "Todo en orden: herramientas instaladas y pruebas pasadas."

# Revisa y abre la burbuja (opcional: número o ID del paciente)
abre paciente="": revisa
    cargo run --release --quiet -- {{ if paciente == "" { "" } else { "--paciente " + paciente } }}

# Abre sin revisar
rapido paciente="":
    cargo run --release --quiet -- {{ if paciente == "" { "" } else { "--paciente " + paciente } }}

# Abre conectada al servidor de Medicci: tarjeta e IA local (opcional: NHC, p. ej. 129798)
conectada paciente="" servidor="http://localhost:8000":
    cargo run --release --quiet -- --servidor {{ servidor }} {{ if paciente == "" { "" } else { "--paciente " + paciente } }}

# Compila el ejecutable
compila: revisa
    cargo build --release
    @echo "Ejecutable: {{ exe }}"

# Solo las pruebas
prueba:
    cargo test --release

# Borra lo compilado
limpia:
    cargo clean

[unix]
_sistema:
    #!/usr/bin/env bash
    set -uo pipefail
    falta=0
    bien() { printf '  \033[32mok\033[0m  %s\n' "$1"; }
    mal()  { printf '  \033[31mfalta\033[0m  %s\n        %s\n' "$1" "$2"; falta=1; }
    echo "Revisando lo necesario para Medicci"
    if command -v cargo >/dev/null 2>&1; then bien "$(cargo --version)"
    else mal "Rust (cargo)" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"; fi
    if command -v cc >/dev/null 2>&1; then bien "compilador de C (para SQLite)"
    else mal "compilador de C" "sudo apt install build-essential"; fi
    sueltos=""
    for f in ../bd/sql/01_cita.sql ../bd/sql/02_medicci.sql ../bd/sql/03_vistas.sql ../bd/sql/04_datos.sql \
             fuentes/NotoSans-Regular.ttf fuentes/NotoSans-Bold.ttf fuentes/NotoSansMono-Regular.ttf; do
        [ -f "$f" ] || sueltos="$sueltos ${f#../}"
    done
    if [ -z "$sueltos" ]; then bien "SQL de la base y fuentes"
    else mal "ficheros:$sueltos" "faltan en el repositorio: git pull"; fi
    if [ "$(uname)" = Linux ]; then
        if [ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]; then bien "escritorio gráfico"
        else mal "escritorio gráfico" "abre Medicci desde una sesión con escritorio"; fi
        libs=$(ldconfig -p 2>/dev/null)
        faltan=""
        for lib in libX11.so.6 libXcursor.so.1 libXrandr.so.2 libXi.so.6 libxkbcommon.so.0 libxkbcommon-x11.so.0 libGL.so.1; do
            case "$libs" in *"$lib "*) ;; *) faltan="$faltan $lib" ;; esac
        done
        if [ -z "$faltan" ]; then bien "bibliotecas de ventana y OpenGL"
        else mal "bibliotecas:$faltan" "sudo apt install libx11-6 libxcursor1 libxrandr2 libxi6 libxkbcommon0 libxkbcommon-x11-0 libgl1"; fi
        # el micrófono del dictado (cpal) se compila contra ALSA
        if pkg-config --exists alsa 2>/dev/null; then bien "audio (ALSA) para el dictado"
        else mal "audio (ALSA) para el dictado" "sudo apt install libasound2-dev pkg-config"; fi
    fi
    exit $falta

[windows]
_sistema:
    Write-Host "Revisando lo necesario para Medicci"
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { Write-Host "  falta  Rust (cargo): winget install Rustlang.Rustup   y abre otra terminal" -ForegroundColor Red; exit 1 } else { Write-Host "  ok  $(cargo --version)" -ForegroundColor Green }
    if ((rustc -vV | Out-String) -match 'host: \S*msvc') { $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"; $vs = if (Test-Path $vswhere) { & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath } else { $null }; if (-not $vs) { Write-Host "  falta  herramientas de C++ de Visual Studio: winget install Microsoft.VisualStudio.2022.BuildTools --override `"--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended`"" -ForegroundColor Red; exit 1 } else { Write-Host "  ok  herramientas de C++ de Visual Studio" -ForegroundColor Green } }
    $f = @('..\bd\sql\01_cita.sql','..\bd\sql\02_medicci.sql','..\bd\sql\03_vistas.sql','..\bd\sql\04_datos.sql','fuentes\NotoSans-Regular.ttf','fuentes\NotoSans-Bold.ttf','fuentes\NotoSansMono-Regular.ttf') | Where-Object { -not (Test-Path $_) }; if ($f) { Write-Host "  falta  $($f -join ', '): git pull" -ForegroundColor Red; exit 1 } else { Write-Host "  ok  SQL de la base y fuentes" -ForegroundColor Green }
