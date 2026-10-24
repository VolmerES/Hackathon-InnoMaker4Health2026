# data/

Aquí van los **11 CSV sintéticos** y el **PDF** con su estructura, tal como los
entregó la organización. No se suben al repositorio (ver `.gitignore`).

Ficheros que se esperan:

```text
PACIENTES.csv            DERIVACIONES_AP.csv      AGENDAS.csv
HUECOS_AGENDA.csv        CITAS.csv                MOVIMIENTOS_CITAS.csv
OCUPACIONES_HUECO.csv    COMUNICACIONES.csv       TAREAS_ADMINISTRATIVAS.csv
CIRCUITO_CONSULTA.csv    PRUEBAS.csv
```

El modelo de datos está en [`../docs/CONTEXTO.md`](../docs/CONTEXTO.md#modelo-entidadrelación).

## Sin los CSV del reto: la base inventada

Para probar Medicci en un equipo que no los tiene:

```bash
python mvp/base_inventada.py        # escribe aquí los 11 CSV
```

Son dos pacientes inventados con un historial largo y realista, los mismos de
las réplicas de HCIS y SELENE (`demo/web`): **María Alonso Vega** (58 años,
dispepsia; la colonoscopia de control de 2020 se suspendió por la COVID y nunca
se hizo) y **Ramón Esteve Gálvez** (72 años, anemia ferropénica; anticoagulado,
oye mal y las llamadas de aviso no le llegan). Si ya hay CSV del reto, no los
pisa. Cuando lleguen, se copian encima y se borra `BASE_INVENTADA.txt`.
