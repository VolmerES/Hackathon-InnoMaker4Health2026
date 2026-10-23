"""Hace de HCIS y del motor de integración: manda mensajes HL7 v2 a Medici por MLLP.

    python mvp/hl7_emisor.py                          # la historia de los 8 pacientes de la agenda
    python mvp/hl7_emisor.py --ver                    # y enseña cada mensaje
    python mvp/hl7_emisor.py --comprobar              # después compara la tarjeta con la de los CSV
    python mvp/hl7_emisor.py --evento anular          # en directo: anula la próxima cita de María
    python mvp/hl7_emisor.py --evento resultado       # en directo: llega el resultado de su prueba

Medici tiene que estar arrancado con --datos hl7 (receptor en el puerto 2575).
"""

import argparse
import json
import sys
import time
import urllib.request
from datetime import datetime, timedelta

import hl7
from agenda import elegir_agenda
from alertas import tarjeta
from paciente import cargar, datos_del_paciente

TRIGGER = {"CREACION": "S12", "REPROGRAMACION": "S13", "ANULACION": "S15", "ATENCION": "S14"}
ORC = {"PROGRAMADA": "SC", "REALIZADA": "CM", "NO_REALIZADA": "DC"}


def _hueco(con, id_hueco):
    return con.execute(
        "SELECT h.fecha, h.hora_inicio, h.id_agenda, a.especialidad FROM huecos_agenda h "
        "LEFT JOIN agendas a USING (id_agenda) WHERE h.id_hueco = ?", [id_hueco]).fetchone()


def _siu(con, nhc, cita, mov_id, cuando, tipo_mov, origen, motivo, estado, id_hueco) -> str:
    h = _hueco(con, id_hueco) if id_hueco else None
    inicio = datetime.combine(h[0], h[1]) if h else None
    trigger = TRIGGER.get(tipo_mov) or ("S26" if estado == "NO_REALIZADA" else "S14")
    return hl7.mensaje("SIU", trigger, "SIU_S12", mov_id, cuando,
        hl7.segmento("SCH", cita["id_cita"], id_hueco or "", "", cita["id_derivacion"] or "", "",
                     motivo or "", cita["tipo_visita"], cita["prioridad"], "", "",
                     ["", "", "", hl7.ts(inicio)], *[""] * 13, estado),
        hl7.segmento("EVN", trigger, hl7.ts(cuando), "", tipo_mov, origen or ""),
        hl7.segmento("PID", "1", "", [nhc, "", "", "HUP", "MR"]),
        hl7.segmento("RGS", "1"),
        hl7.segmento("AIL", "1", "", [h[2], "", "", h[3]] if h else ""))


def mensajes_de(con, nhc: str) -> list[tuple[datetime, str]]:
    """Toda la historia del paciente como mensajes HL7, por orden de fecha."""
    out = []
    p = con.execute("SELECT edad, sexo, preferencias_contacto, necesidades_accesibilidad "
                    "FROM pacientes WHERE id_paciente = ?", [nhc]).fetchone()
    alta = datetime(2024, 1, 1)
    out.append((alta, hl7.mensaje("ADT", "A04", "ADT_A01", f"ADT-{nhc}", alta,
        hl7.segmento("EVN", "A04", hl7.ts(alta)),
        hl7.segmento("PID", "1", "", [nhc, "", "", "HUP", "MR"], "", ["SINTETICO", "PACIENTE"], "",
                     f"{2025 - p[0]}0101", p[1]),
        hl7.segmento("OBX", "1", "NM", "EDAD", "", p[0]),
        hl7.segmento("OBX", "2", "ST", "CONTACTO", "", p[2]),
        hl7.segmento("OBX", "3", "ST", "ACCESIBILIDAD", "", p[3]))))

    for d in con.execute(
            "SELECT id_derivacion, fecha_solicitud, especialidad_destino, prioridad, motivo_derivacion, "
            "observaciones_medico, estado_solicitud FROM derivaciones_ap WHERE id_paciente = ?", [nhc]).fetchall():
        out.append((d[1], hl7.mensaje("REF", "I12", "REF_I12", d[0], d[1],
            hl7.segmento("RF1", d[6], d[3], d[2], "", "", d[0], hl7.ts(d[1]), "", "", d[4]),
            hl7.segmento("PRD", "RP", ["MEDICO", "ATENCION PRIMARIA"]),
            hl7.segmento("PID", "1", "", [nhc, "", "", "HUP", "MR"]),
            hl7.segmento("NTE", "1", "", d[5]))))

    citas = con.execute("SELECT id_cita, id_derivacion, id_hueco, tipo_visita, prioridad FROM citas "
                        "WHERE id_paciente = ?", [nhc]).fetchall()
    for c in citas:
        cita = dict(zip(["id_cita", "id_derivacion", "id_hueco", "tipo_visita", "prioridad"], c))
        movs = con.execute(
            "SELECT id_movimiento, fecha_hora_movimiento, tipo_movimiento, origen_movimiento, motivo, "
            "estado_nuevo, id_hueco_nuevo FROM movimientos_citas WHERE id_cita = ? "
            "ORDER BY fecha_hora_movimiento", [cita["id_cita"]]).fetchall()
        hueco = cita["id_hueco"]
        for mv in movs:
            hueco = mv[6] or hueco
            out.append((mv[1], _siu(con, nhc, cita, mv[0], mv[1], mv[2], mv[3], mv[4], mv[5], hueco)))

    for r in con.execute(
            "SELECT id_prueba, tipo_prueba, fecha_solicitud, fecha_programada, fecha_realizacion, "
            "estado_prueba, resultado_disponible FROM pruebas WHERE id_paciente = ?", [nhc]).fetchall():
        def prueba(tipo, evento, estructura, control, cuando, estado):
            return hl7.mensaje(tipo, evento, estructura, control, cuando,
                hl7.segmento("PID", "1", "", [nhc, "", "", "HUP", "MR"]),
                hl7.segmento("ORC", "NW" if estado == "PROGRAMADA" else "SC", r[0], "", "", ORC[estado]),
                hl7.segmento("TQ1", "1", "", "", "", "", "", hl7.ts(r[3])),
                hl7.segmento("OBR", "1", r[0], "", r[1], "", hl7.ts(r[2]),
                             hl7.ts(r[4]) if estado == "REALIZADA" else "", *[""] * 17,
                             "F" if (estado == "REALIZADA" and r[6]) else ""),
                *([hl7.segmento("OBX", "1", "ST", "RESULTADO", "", "Disponible en HCIS (sin valor en los datos sintéticos)")]
                  if estado == "REALIZADA" and r[6] else []))
        out.append((r[2], prueba("OMG", "O19", "OMG_O19", f"{r[0]}-P", r[2], "PROGRAMADA")))
        if r[5] == "REALIZADA":
            out.append((r[4] or r[3], prueba("ORU", "R01", "ORU_R01", f"{r[0]}-R", r[4] or r[3], "REALIZADA")))
        elif r[5] == "NO_REALIZADA":
            out.append((r[3] or r[2], prueba("OMG", "O19", "OMG_O19", f"{r[0]}-N", r[3] or r[2], "NO_REALIZADA")))

    return sorted(out, key=lambda x: x[0] or alta)


def evento(con, nhc: str, cual: str) -> str:
    """Un mensaje nuevo, de ahora mismo, para enseñarlo en directo."""
    ahora = datetime.now()
    if cual == "anular":
        c = con.execute(
            "SELECT c.id_cita, c.id_derivacion, c.id_hueco, c.tipo_visita, c.prioridad FROM citas c "
            "LEFT JOIN huecos_agenda h USING (id_hueco) WHERE c.id_paciente = ? AND c.estado_actual <> 'ANULADA' "
            "ORDER BY c.estado_actual IN ('NO_REALIZADA', 'REALIZADA'), h.fecha DESC LIMIT 1", [nhc]).fetchone()
        if not c:
            raise SystemExit(f"{nhc} no tiene citas que anular")
        cita = dict(zip(["id_cita", "id_derivacion", "id_hueco", "tipo_visita", "prioridad"], c))
        return _siu(con, nhc, cita, f"EV{ahora:%H%M%S}", ahora, "ANULACION", "PACIENTE",
                    "Paciente cancela", "ANULADA", cita["id_hueco"])
    if cual == "resultado":
        r = con.execute("SELECT id_prueba, tipo_prueba, fecha_solicitud, fecha_programada FROM pruebas "
                        "WHERE id_paciente = ? AND estado_prueba = 'PROGRAMADA' LIMIT 1", [nhc]).fetchone()
        if not r:
            raise SystemExit(f"{nhc} no tiene pruebas pendientes")
        return hl7.mensaje("ORU", "R01", "ORU_R01", f"EV{ahora:%H%M%S}", ahora,
            hl7.segmento("PID", "1", "", [nhc, "", "", "HUP", "MR"]),
            hl7.segmento("ORC", "SC", r[0], "", "", "CM"),
            hl7.segmento("TQ1", "1", "", "", "", "", "", hl7.ts(r[3])),
            hl7.segmento("OBR", "1", r[0], "", r[1], "", hl7.ts(r[2]), hl7.ts(ahora), *[""] * 17, "F"),
            hl7.segmento("OBX", "1", "ST", "RESULTADO", "", "Disponible en HCIS (sin valor en los datos sintéticos)"))
    raise SystemExit(f"Evento desconocido: {cual}")


def main() -> None:
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--host", default="localhost")
    ap.add_argument("--puerto", type=int, default=2575)
    ap.add_argument("--evento", choices=["anular", "resultado"])
    ap.add_argument("--nhc", help="por defecto, la primera de la agenda (María)")
    ap.add_argument("--ver", action="store_true", help="enseña cada mensaje entero")
    ap.add_argument("--pausa", type=float, default=0.0, help="segundos entre mensajes")
    ap.add_argument("--comprobar", action="store_true", help="compara después la tarjeta de Medici con la de los CSV")
    ap.add_argument("--api", default="http://localhost:8000/api")
    args = ap.parse_args()

    con = cargar()
    agenda = elegir_agenda(con, lambda n: datos_del_paciente(con, n))
    emisor = hl7.Emisor(args.host, args.puerto)

    def mandar(texto):
        r = emisor.mandar(texto)
        m = hl7.Mensaje(texto)
        estado = r.campo("MSA", 1)
        print(f"  {m.tipo:8} {m.control:14} → {estado}{'  ' + r.campo('MSA', 3) if estado != 'AA' else ''}")
        if args.ver:
            print("    " + texto.replace("\r", "\n    "))
        time.sleep(args.pausa)
        return estado == "AA"

    if args.evento:
        nhc = args.nhc or agenda[0]["nhc"]
        print(f"Evento «{args.evento}» de {nhc}:")
        mandar(evento(con, nhc, args.evento))
        return

    total = ok = 0
    for p in agenda:
        print(f"{p['hora']} {p['nombre']} ({p['nhc']})")
        for _, texto in mensajes_de(con, p["nhc"]):
            total += 1
            ok += mandar(texto)
    emisor.cerrar()
    print(f"\n{ok}/{total} mensajes aceptados (ACK AA).")

    if args.comprobar:
        print("\nTarjeta de Medici (datos por HL7) frente a la de los CSV:")
        iguales = 0
        for p in agenda:
            pet = urllib.request.Request(f"{args.api}/sesion", data=json.dumps({"nhc": p["nhc"]}).encode(),
                                         headers={"Content-Type": "application/json"})
            sesion = json.load(urllib.request.urlopen(pet))["sesion"]
            de_hl7 = [a["texto"] for a in json.load(urllib.request.urlopen(f"{args.api}/sesion/{sesion}/tarjeta"))["alertas"]]
            de_csv = [a["texto"] for a in tarjeta(datos_del_paciente(con, p["nhc"]))["alertas"]]
            igual = de_hl7 == de_csv
            iguales += igual
            print(f"  {'✅' if igual else '❌'} {p['nombre']}" + ("" if igual else f"\n     HL7: {de_hl7}\n     CSV: {de_csv}"))
        print(f"{iguales}/{len(agenda)} tarjetas iguales.")


if __name__ == "__main__":
    main()
