#!/usr/bin/env bash
# Crea bd/medicci.db desde cero (envoltorio de crear_bd.py).
set -euo pipefail
exec python3 "$(dirname "$0")/crear_bd.py"
