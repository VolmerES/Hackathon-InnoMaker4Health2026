#!/usr/bin/env bash
# Medicci con un comando. Solo hace falta Docker y los CSV en data/.
# Levanta el modelo y el servidor, manda la historia por HL7 y abre el navegador.
set -euo pipefail
cd "$(dirname "$0")"
echo "MEDICCI · arrancando"

docker info >/dev/null 2>&1 || { echo "Docker no está en marcha: ábrelo y vuelve a lanzar ./arrancar.sh"; exit 1; }
[ -f data/PACIENTES.csv ] || { echo "Faltan los CSV del reto en data/ (no van en GitHub)."; exit 1; }

compose=(-f mvp/docker-compose.yml)
if ! command -v nvidia-smi >/dev/null 2>&1; then
  echo "Sin GPU de NVIDIA: el modelo irá por el procesador, más lento."
  compose+=(-f mvp/docker-compose.sin-gpu.yml)
fi
export MEDICCI_DATOS=hl7

echo "1/4  Modelo y servidor..."
docker compose "${compose[@]}" up -d --build --force-recreate ollama medicci
echo "2/4  Modelo de IA..."
docker exec medicci-ollama ollama list | grep -q "qwen3:8b" || {
  echo "     Descargando qwen3:8b, unos 5 GB, solo la primera vez..."
  docker exec medicci-ollama ollama pull qwen3:8b
}
echo "3/4  Esperando al servidor..."
until curl -s http://localhost:8000/api/estado >/dev/null; do sleep 2; done
echo "4/4  HCIS manda la historia por HL7..."
docker compose "${compose[@]}" run --rm --no-deps emisor --comprobar

url="http://localhost:8000/?paciente=primera"
(xdg-open "$url" || open "$url") >/dev/null 2>&1 || true
echo
echo "Listo: http://localhost:8000   ·   mensajes HL7: http://localhost:8000/hl7.html"
echo "En directo: docker compose -f mvp/docker-compose.yml run --rm --no-deps emisor --evento resultado"
echo "Para pararlo: docker compose -f mvp/docker-compose.yml down"
