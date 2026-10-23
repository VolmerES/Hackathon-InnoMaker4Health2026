"""La tarjeta «3 cosas antes de que entre»: lo que el médico no puede pasar por alto.

Las alertas las deciden reglas sobre los datos, no la IA: son siempre ciertas y
cada una lleva las filas de las que sale ([M2], [P1]…). Se ordenan por
importancia y se enseñan las tres primeras.

    python mvp/alertas.py P0102733          # la tarjeta de un paciente
    python mvp/alertas.py --buscar-demo     # pacientes con una tarjeta completa, para la demo
"""

import argparse
import json
import sys
from collections import Counter
from dataclasses import asdict, dataclass
from datetime import datetime, timedelta

from paciente import ESPECIALIDADES, Fila, cargar, datos_del_paciente

MAXIMO = 3
RECIENTE = timedelta(hours=48)

ACCESIBILIDAD = {
    "INTERPRETE": "Necesita intérprete",
    "MOVILIDAD_REDUCIDA": "Tiene movilidad reducida",
    "ACOMPANANTE": "Necesita acompañante",
    "AUDITIVA": "Tiene discapacidad auditiva",
    "VISUAL": "Tiene discapacidad visual",
}
PRUEBA = {
    "ANALITICA": "analítica", "ECOGRAFIA": "ecografía", "RADIOGRAFIA": "radiografía",
    "ECG": "ECG", "TAC": "TAC", "RESONANCIA": "resonancia",
    "ESPIROMETRIA": "espirometría", "ENDOSCOPIA": "endoscopia",
}
CANAL = {"SMS": "SMS", "LLAMADA": "llamada", "EMAIL": "email", "APP": "la app", "CARTA": "carta"}


@dataclass
class Alerta:
    tipo: str
    prioridad: int          # más alto, más arriba
    texto: str
    fuentes: list[str]


def _fecha(valor) -> str:
    return valor.strftime("%d/%m/%Y") if valor else "sin fecha"


def _veces(n: int, una: str, varias: str) -> str:
    return una if n == 1 else varias.format(n=n)


def _lista(partes: list[str]) -> str:
    return partes[0] if len(partes) == 1 else ", ".join(partes[:-1]) + " y " + partes[-1]


def alertas(filas: list[Fila]) -> list[Alerta]:
    """Todas las alertas del paciente, de más a menos importante."""
    de = {t: [f for f in filas if f.tabla == t and not f.campos.get("vacia")] for t in
          ("PACIENTES", "DERIVACIONES_AP", "CITAS", "MOVIMIENTOS_CITAS", "PRUEBAS", "COMUNICACIONES")}
    servicio_de = {f.ref: ESPECIALIDADES.get(f.campos["especialidad"], "otro servicio") for f in de["CITAS"]}
    lista: list[Alerta] = []

    for f in de["PACIENTES"]:
        if f.campos["accesibilidad"] in ACCESIBILIDAD:
            lista.append(Alerta("accesibilidad", 90, f"{ACCESIBILIDAD[f.campos['accesibilidad']]}.", [f.ref]))

    for f in de["DERIVACIONES_AP"]:
        if f.campos["prioridad"] != "ORDINARIA" and f.campos["estado"] != "RECHAZADA":
            lista.append(Alerta("derivacion_prioritaria", 72,
                f"Derivación {f.campos['prioridad'].lower()} de Primaria a "
                f"{ESPECIALIDADES.get(f.campos['especialidad'], f.campos['especialidad'])} "
                f"({_fecha(f.campos['fecha'])}): {f.campos['motivo'].lower()}.", [f.ref]))

    no_realizadas = [f for f in de["MOVIMIENTOS_CITAS"] if f.campos["tipo"] == "NO_REALIZADA"]
    if no_realizadas:
        ultima = no_realizadas[-1]
        n = len(no_realizadas)
        servicio = servicio_de.get(ultima.campos["cita"], "otro servicio")
        cuando = _fecha(ultima.campos["fecha"])
        lista.append(Alerta("citas_no_realizadas", 80 if n >= 2 else 65,
            f"1 cita se cerró sin atenderle ({servicio}, {cuando})." if n == 1 else
            f"{n} citas se cerraron sin atenderle; la última, en {servicio} el {cuando}.",
            [f.ref for f in no_realizadas]))

    sin_hacer = [f for f in de["PRUEBAS"] if f.campos["estado"] == "NO_REALIZADA"]
    if sin_hacer:
        lista.append(Alerta("prueba_no_realizada", 78,
            f"No se llegó a hacer: "
            + _lista([f"{PRUEBA.get(f.campos['tipo'], f.campos['tipo'])} "
                      f"(programada el {_fecha(f.campos['programada'])})" for f in sin_hacer]) + ".",
            [f.ref for f in sin_hacer]))

    pendientes = [f for f in de["PRUEBAS"] if f.campos["estado"] == "PROGRAMADA"]
    if pendientes:
        lista.append(Alerta("prueba_pendiente", 75,
            _veces(len(pendientes), "Prueba pendiente: ", "Pruebas pendientes: ")
            + _lista([f"{PRUEBA.get(f.campos['tipo'], f.campos['tipo'])} "
                      f"(programada el {_fecha(f.campos['programada'])})" for f in pendientes]) + ".",
            [f.ref for f in pendientes]))

    fallidos = [f for f in de["COMUNICACIONES"] if f.campos["resultado"] == "FALLIDO"]
    if fallidos:
        canal, _ = Counter(f.campos["canal"] for f in fallidos).most_common(1)[0]
        n = len(fallidos)
        lista.append(Alerta("avisos_fallidos", 70 if n >= 2 else 55,
            f"1 aviso por {CANAL.get(canal, canal)} no le llegó ({_fecha(fallidos[0].campos['fecha'])})."
            if n == 1 else
            f"{n} avisos no le llegaron (sobre todo por {CANAL.get(canal, canal)}); el último, el "
            f"{_fecha(fallidos[-1].campos['fecha'])}.", [f.ref for f in fallidos]))

    con_resultado = [f for f in de["PRUEBAS"] if f.campos["estado"] == "REALIZADA" and f.campos["resultado"]]
    if con_resultado:
        lista.append(Alerta("resultado_disponible", 60,
            _veces(len(con_resultado), "Ya tiene hecha la ", "Ya tiene hechas: ")
            + _lista([f"{PRUEBA.get(f.campos['tipo'], f.campos['tipo'])} "
                      f"({_fecha(f.campos['realizada'])})" for f in con_resultado])
            + ", con resultado disponible.", [f.ref for f in con_resultado]))

    cambios = [f for f in de["MOVIMIENTOS_CITAS"] if f.campos["tipo"] in ("REPROGRAMACION", "ANULACION")]
    if cambios:
        motivos = list(dict.fromkeys(f.campos["motivo"].lower() for f in cambios))
        cuenta = Counter(f.campos["tipo"] for f in cambios)
        partes = [_veces(cuenta[t], f"1 cita {uno}", f"{{n}} citas {varios}").format(n=cuenta[t])
                  for t, uno, varios in [("REPROGRAMACION", "reprogramada", "reprogramadas"),
                                         ("ANULACION", "anulada", "anuladas")] if cuenta[t]]
        lista.append(Alerta("citas_cambiadas", 50 if len(cambios) >= 2 else 35,
            f"{_lista(partes).capitalize()} ({_lista(motivos)}).", [f.ref for f in cambios]))

    # Lo que ha pasado en las últimas 48 horas (una cita anulada, un resultado que
    # acaba de llegar por HL7) sube arriba del todo: es lo que el médico no sabe.
    por_ref = {f.ref: f for f in filas}
    hace_poco = datetime.now() - RECIENTE
    for a in lista:
        fechas = [por_ref[r].campos.get(c) for r in a.fuentes if r in por_ref for c in ("fecha", "realizada")]
        if any(isinstance(f, datetime) and f >= hace_poco for f in fechas):
            a.prioridad += 100
            a.texto = "Nuevo: " + a.texto[0].lower() + a.texto[1:]
    return sorted(lista, key=lambda a: -a.prioridad)


def tarjeta(filas: list[Fila]) -> dict:
    todas = alertas(filas)
    return {"alertas": [asdict(a) for a in todas[:MAXIMO]], "mas": max(0, len(todas) - MAXIMO)}


def imprimir(id_paciente: str, filas: list[Fila]) -> None:
    t = tarjeta(filas)
    print(f"\n── {id_paciente} · antes de que entre ──")
    if not t["alertas"]:
        print("   Nada que destacar en sus datos.")
    for a in t["alertas"]:
        print(f" • {a['texto']}  [{', '.join(a['fuentes'])}]")
    if t["mas"]:
        print(f"   (+{t['mas']} más)")


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("pacientes", nargs="*")
    ap.add_argument("--buscar-demo", action="store_true", help="pacientes con 3 alertas de tipos distintos")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    con = cargar()
    if args.buscar_demo:
        candidatos = con.execute(
            "SELECT p.id_paciente FROM pacientes p WHERE p.necesidades_accesibilidad <> 'NINGUNA' "
            "AND EXISTS (SELECT 1 FROM pruebas r WHERE r.id_paciente = p.id_paciente "
            "AND r.estado_prueba = 'PROGRAMADA') ORDER BY hash(p.id_paciente) LIMIT 400").fetchall()
        encontrados = 0
        for (pid,) in candidatos:
            filas = datos_del_paciente(con, pid)
            if len({a.tipo for a in alertas(filas)[:MAXIMO]}) == MAXIMO and len(filas) <= 20:
                imprimir(pid, filas)
                encontrados += 1
                if encontrados == 5:
                    break
        return
    for pid in args.pacientes or ["P0102733"]:
        filas = datos_del_paciente(con, pid)
        if args.json:
            print(json.dumps({"paciente": pid, **tarjeta(filas)}, ensure_ascii=False, indent=2))
        else:
            imprimir(pid, filas)


if __name__ == "__main__":
    main()
