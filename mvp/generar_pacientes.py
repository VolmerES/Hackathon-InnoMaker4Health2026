"""Genera pacientes sintéticos para probar Medicci con crónicos: años de
seguimiento y cientos de citas, con sus pruebas, avisos, cambios y faltas.

Escribe los 11 CSV con el mismo formato que los del reto en otra carpeta, para
no mezclarlos. Para usarlos: MEDICCI_CSV=data/generados.

    python mvp/generar_pacientes.py                       # 200 pacientes en data/generados
    python mvp/generar_pacientes.py --cronicos 100 --semilla 7

Perfiles (citas por paciente, aproximadas):
    normal         1-10 citas, menos de un año
    crónico        40-150 citas, 3-8 años, 1-3 servicios
    muy crónico    200-400 citas, 8-15 años, 2-4 servicios
    extremo        unas 1.000 citas, 15 años, 4-6 servicios
"""

import argparse
import csv
import random
import shutil
import sys
from datetime import datetime, timedelta
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
ORIGINAL = RAIZ / "data"

PERFILES = {          # años, servicios, meses entre citas (mín, máx)
    "normal": ((0.2, 1.0), (1, 1), (1.0, 4.0)),
    "cronico": ((3, 8), (1, 3), (2.0, 6.0)),
    "muy_cronico": ((8, 15), (2, 4), (1.0, 2.5)),
    "extremo": ((15, 15), (4, 6), (0.4, 0.7)),
}
PRUEBAS = {"CAR": ["ECG", "ANALITICA", "ECOGRAFIA"], "DIG": ["ANALITICA", "ENDOSCOPIA", "ECOGRAFIA"],
           "TRA": ["RADIOGRAFIA", "RESONANCIA", "TAC"], "NRL": ["RESONANCIA", "TAC", "ANALITICA"],
           "OFT": ["ANALITICA", "TAC"], "DER": ["ANALITICA", "ECOGRAFIA"]}
MOTIVOS_DER = ["Valoración especializada", "Revisión", "Reanudar seguimiento"]
OBSERVACIONES = [
    "SEGUIMIENTO: Paciente crónico en seguimiento por vuestro servicio. Solicito revisión. Un saludo cordial.",
    "VALORACIÓN Paciente con {s} de {m} meses de evolución sin mejoría con tto en AP. Derivo para valoración.",
    "REVISIÓN: empeoramiento de {s} previamente valorado. Ruego revisión. Gracias.",
    "Pcte con {s}, analítica básica sin hallazgos. Valorar. Un saludo.",
]
SINTOMAS = ["dolor persistente", "molestias inespecíficas", "alteración analítica", "disnea de esfuerzo",
            "lesiones cutáneas", "pérdida de visión", "cefalea", "dolor articular"]
MOTIVO_CAMBIO = ["Reorganización de agenda", "Ausencia del profesional", "Petición del paciente",
                 "Incompatibilidad horaria", "Error de citación", "Duplicidad detectada"]
MOTIVO_ANULA = ["Paciente cancela", "Cita ya no necesaria", "Reorganización de agenda"]
HOY = datetime(2026, 10, 24, 9, 0)


def ts(d: datetime) -> str:
    return d.strftime("%Y-%m-%d %H:%M")


class Generador:
    def __init__(self, semilla: int):
        self.r = random.Random(semilla)
        self.n = {k: 0 for k in ["P", "D", "C", "H", "M", "PR", "CO"]}
        self.filas = {t: [] for t in ["PACIENTES", "DERIVACIONES_AP", "HUECOS_AGENDA", "CITAS",
                                      "MOVIMIENTOS_CITAS", "PRUEBAS", "COMUNICACIONES"]}
        with open(ORIGINAL / "AGENDAS.csv", encoding="utf-8-sig") as f:
            self.agendas: dict[str, list[str]] = {}
            for fila in csv.DictReader(f):
                self.agendas.setdefault(fila["especialidad"], []).append(fila["id_agenda"])

    def id(self, k: str, prefijo: str, ancho: int) -> str:
        self.n[k] += 1
        return f"{prefijo}{self.n[k]:0{ancho}d}"

    def hueco(self, especialidad: str, cuando: datetime) -> str:
        h = self.id("H", "GH", 8)
        fin = cuando + timedelta(minutes=20)
        self.filas["HUECOS_AGENDA"].append([h, self.r.choice(self.agendas[especialidad]), cuando.strftime("%Y-%m-%d"),
                                            cuando.strftime("%H:%M"), fin.strftime("%H:%M"), "OCUPADO", "SUCESIVA"])
        return h

    def aviso(self, cita, pid, canal, motivo, cuando, fallo=0.05):
        if cuando > HOY:
            return
        falla = self.r.random() < fallo
        respuesta = "" if falla else self.r.choice(["", "", "CONFIRMA", "SIN_RESPUESTA"])
        if motivo == "AVISO_CAMBIO_CITA" and not falla:
            respuesta = "ACEPTA"
        self.filas["COMUNICACIONES"].append([self.id("CO", "GCO", 8), cita, pid, "", canal, motivo, ts(cuando),
                                             respuesta, "FALLIDO" if falla else "ENTREGADO"])

    def paciente(self, perfil: str) -> int:
        r = self.r
        (a0, a1), (s0, s1), (m0, m1) = PERFILES[perfil]
        pid = self.id("P", "G", 7)
        cronico = perfil != "normal"
        edad = r.randint(55, 90) if cronico else r.randint(18, 90)
        canal = r.choices(["SMS", "LLAMADA", "EMAIL", "APP", "CARTA"], [40, 20, 18, 17, 5])[0]
        apoyo = r.choices(["NINGUNA", "MOVILIDAD_REDUCIDA", "INTERPRETE", "ACOMPANANTE", "AUDITIVA", "VISUAL"],
                          [60, 14, 6, 8, 7, 5] if cronico else [82, 6, 4, 3, 3, 2])[0]
        self.filas["PACIENTES"].append([pid, edad, r.choice("FM"), canal, apoyo])

        años = r.uniform(a0, a1)
        inicio = HOY - timedelta(days=365 * años)
        servicios = r.sample(sorted(self.agendas), r.randint(s0, s1))
        citas = 0
        for esp in servicios:
            # la derivación de Primaria que abre el seguimiento en ese servicio
            alta = inicio + timedelta(days=r.randint(0, 90))
            der = self.id("D", "GD", 8)
            plantilla = r.choice(OBSERVACIONES).format(s=r.choice(SINTOMAS), m=r.randint(2, 14))
            self.filas["DERIVACIONES_AP"].append([der, pid, ts(alta), esp,
                                                  r.choices(["ORDINARIA", "PREFERENTE", "URGENTE"], [82, 15, 3])[0],
                                                  r.choice(MOTIVOS_DER), plantilla, "CITADA"])
            cuando = alta + timedelta(days=r.randint(20, 60))
            primera = True
            while cuando < HOY + timedelta(days=120):
                cuando = cuando.replace(hour=r.randint(8, 14), minute=r.choice([0, 20, 40]))
                citas += self.cita(pid, der if primera else "", esp, cuando, canal, primera)
                primera = False
                cuando += timedelta(days=30 * r.uniform(m0, m1))
                if cuando > HOY and r.random() < 0.6:
                    break                                   # no todas tienen cita futura
        return citas

    def cita(self, pid, der, esp, cuando, canal, primera) -> int:
        r = self.r
        cid = self.id("C", "GC", 8)
        asignada = cuando - timedelta(days=r.randint(15, 90), hours=r.randint(0, 9))
        h = self.hueco(esp, cuando)
        tipo = "P" if primera else r.choices(["S", "T"], [85, 15])[0]
        prioridad = r.choices(["ORDINARIA", "URGENTE"], [93, 7])[0]
        movs = [(asignada, "CREACION", "DERIVACION_AP" if primera else "SERVICIO", "Asignación inicial",
                 "", "CITADA", "", h)]
        reprogramada = anulada = 0
        self.aviso(cid, pid, canal, "CONFIRMACION_CITA", asignada + timedelta(minutes=5))
        if r.random() < 0.15:                               # se cambia de fecha
            reprogramada = 1
            nueva = cuando + timedelta(days=r.randint(3, 40))
            h2 = self.hueco(esp, nueva)
            cambio = asignada + (cuando - asignada) / 2
            movs.append((cambio, "REPROGRAMACION", r.choice(["SERVICIO", "PACIENTE", "ADMISION"]),
                         r.choice(MOTIVO_CAMBIO), "CITADA", "CITADA", h, h2))
            self.aviso(cid, pid, canal, "AVISO_CAMBIO_CITA", cambio)
            cuando, h = nueva, h2
        if cuando > HOY:
            estado = "CITADA"
        else:
            estado = r.choices(["REALIZADA", "NO_REALIZADA", "ANULADA", "URGENTE"], [80, 10, 5, 5])[0]
            if estado != "ANULADA":
                self.aviso(cid, pid, canal, "RECORDATORIO", cuando - timedelta(days=2, hours=r.randint(0, 6)))
            if estado == "REALIZADA":
                movs.append((cuando, "ATENCION", "SISTEMA", "Cita atendida", "CITADA", "REALIZADA", h, h))
            elif estado == "NO_REALIZADA":
                movs.append((cuando, "CIERRE", "SISTEMA", "Cita no realizada", "CITADA", "NO_REALIZADA", h, h))
            elif estado == "URGENTE":
                movs.append((cuando, "CIERRE", "SISTEMA", "Cita urgente", "CITADA", "URGENTE", h, h))
            else:
                anulada = 1
                quita = cuando - timedelta(days=r.randint(1, 10))
                movs.append((quita, "ANULACION", r.choice(["PACIENTE", "SERVICIO"]), r.choice(MOTIVO_ANULA),
                             "CITADA", "ANULADA", h, ""))
        self.filas["CITAS"].append([cid, pid, der, h, ts(asignada), tipo, estado, prioridad, estado,
                                    reprogramada, anulada, ""])
        for m in movs:
            self.filas["MOVIMIENTOS_CITAS"].append([self.id("M", "GM", 8), cid, ts(m[0]), *m[1:]])
        if estado == "REALIZADA" and r.random() < 0.3:      # se pide una prueba
            self.prueba(pid, cid, esp, cuando)
        return 1

    def prueba(self, pid, cid, esp, cuando):
        r = self.r
        programada = (cuando + timedelta(days=r.randint(7, 45))).replace(hour=r.randint(8, 13), minute=0)
        if programada > HOY:
            estado, hecha, resultado = "PROGRAMADA", "", 0
        elif r.random() < 0.08:
            estado, hecha, resultado = "NO_REALIZADA", "", 0
        else:
            estado, hecha, resultado = "REALIZADA", ts(programada), 1 if r.random() < 0.95 else 0
        self.filas["PRUEBAS"].append([self.id("PR", "GPR", 8), pid, cid, "", r.choice(PRUEBAS.get(esp, ["ANALITICA"])),
                                      ts(cuando), ts(programada), hecha, estado, resultado])

    def escribir(self, carpeta: Path):
        carpeta.mkdir(parents=True, exist_ok=True)
        shutil.copy(ORIGINAL / "AGENDAS.csv", carpeta / "AGENDAS.csv")
        for tabla in ["PACIENTES", "DERIVACIONES_AP", "HUECOS_AGENDA", "CITAS", "MOVIMIENTOS_CITAS",
                      "PRUEBAS", "COMUNICACIONES", "OCUPACIONES_HUECO", "TAREAS_ADMINISTRATIVAS",
                      "CIRCUITO_CONSULTA"]:
            with open(ORIGINAL / f"{tabla}.csv", encoding="utf-8-sig") as f:
                cabecera = next(csv.reader(f))
            with open(carpeta / f"{tabla}.csv", "w", newline="", encoding="utf-8") as f:
                w = csv.writer(f)
                w.writerow(cabecera)
                w.writerows(self.filas.get(tabla, []))         # las tres que Medicci no usa van vacías


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--carpeta", default=str(ORIGINAL / "generados"))
    ap.add_argument("--semilla", type=int, default=24)
    ap.add_argument("--normales", type=int, default=120)
    ap.add_argument("--cronicos", type=int, default=60)
    ap.add_argument("--muy-cronicos", type=int, default=18)
    ap.add_argument("--extremos", type=int, default=2)
    args = ap.parse_args()

    g = Generador(args.semilla)
    citas = {}
    for perfil, n in [("normal", args.normales), ("cronico", args.cronicos),
                      ("muy_cronico", args.muy_cronicos), ("extremo", args.extremos)]:
        citas[perfil] = [g.paciente(perfil) for _ in range(n)]
    g.escribir(Path(args.carpeta))

    print(f"Pacientes sintéticos en {args.carpeta}:")
    for perfil, lista in citas.items():
        if lista:
            print(f"  {perfil:12} {len(lista):4} pacientes · citas por paciente: "
                  f"mín {min(lista)}, mediana {sorted(lista)[len(lista) // 2]}, máx {max(lista)}")
    print("  filas:", ", ".join(f"{t.lower()} {len(f)}" for t, f in g.filas.items()))
    print(f"\nPara usarlos: MEDICCI_CSV={args.carpeta}")


if __name__ == "__main__":
    main()
