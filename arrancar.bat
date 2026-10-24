@echo off
rem Medicci con un doble clic. Solo hace falta Docker Desktop y los CSV en data\.
rem Levanta el modelo y el servidor, manda la historia por HL7 y abre el navegador.
chcp 65001 >nul
cd /d "%~dp0"
title Medicci
echo.
echo   MEDICCI · arrancando
echo.

docker info >nul 2>&1
if errorlevel 1 (
  echo   Docker no está abierto. Lo abro: cuando la ballena esté quieta, vuelve a pulsar arrancar.
  start "" "C:\Program Files\Docker\Docker\Docker Desktop.exe"
  pause
  exit /b 1
)
if not exist "data\PACIENTES.csv" (
  echo   Faltan los CSV del reto en la carpeta data\ ^(no van en GitHub^).
  pause
  exit /b 1
)

set "COMPOSE=-f mvp\docker-compose.yml"
where nvidia-smi >nul 2>&1
if errorlevel 1 (
  echo   Sin GPU de NVIDIA: el modelo irá por el procesador, más lento.
  set "COMPOSE=-f mvp\docker-compose.yml -f mvp\docker-compose.sin-gpu.yml"
)
set "MEDICCI_DATOS=hl7"

echo   1/4  Modelo y servidor...
docker compose %COMPOSE% up -d --build --force-recreate ollama medicci
if errorlevel 1 ( pause & exit /b 1 )

echo   2/4  Modelo de IA...
docker exec medicci-ollama ollama list | findstr /c:"qwen3:8b" >nul
if errorlevel 1 (
  echo        Descargando qwen3:8b, unos 5 GB, solo la primera vez...
  docker exec medicci-ollama ollama pull qwen3:8b
)

echo   3/4  Esperando al servidor...
:espera
curl -s http://localhost:8000/api/estado >nul 2>&1
if errorlevel 1 ( timeout /t 2 /nobreak >nul & goto espera )

echo   4/4  HCIS manda la historia por HL7...
docker compose %COMPOSE% run --rm --no-deps emisor --comprobar

start "" "http://localhost:8000/?paciente=primera"
echo.
echo   Listo: http://localhost:8000   ·   mensajes HL7: http://localhost:8000/hl7.html
echo          registro de accesos: http://localhost:8000/registro.html
echo   En directo: docker compose -f mvp\docker-compose.yml run --rm --no-deps emisor --evento resultado
echo   Para pararlo: docker compose -f mvp\docker-compose.yml down
echo.
pause
