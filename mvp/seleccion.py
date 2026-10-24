"""Qué filas del paciente se le mandan al modelo: las que hacen falta para la
pregunta, y que quepan.

Un crónico de años tiene cientos de citas: no caben en la memoria de un modelo
pequeño (Ollama corta en silencio lo que sobra, y con ello las instrucciones).
Así que, en vez de todo:

1. por tema: si preguntan por pruebas, las pruebas; por citas, las citas y sus
   cambios; por avisos, los avisos…;
2. lo importante primero: lo pendiente y lo más reciente, hasta el presupuesto;
3. siempre la ficha y un resumen de toda la historia hecho con SQL ([R1]), para
   que «¿cuántas…?» salga bien aunque no vayan todas las filas;
4. si algo se queda fuera, se dice ([R2]).
"""

import os
import unicodedata
from collections import Counter
from datetime import date, datetime

from paciente import ESPECIALIDADES, Fila

# Memoria del modelo, en tokens (Ollama corta en ~2.048 si no se le dice otra
# cosa). Del total se reservan unos 700 para instrucciones, pregunta y respuesta.
CONTEXTO = int(os.environ.get("MEDICCI_CONTEXTO", "2048"))
CARACTERES_POR_TOKEN = 3

TEMAS = {
    "PRUEBAS": ["prueba", "analit", "ecograf", "radiograf", "resonanc", "tac", "ecg", "endoscop",
                "espirometr", "resultado", "pendiente"],
    "CITAS": ["cita", "consulta", "falt", "acud", "atend", "anul", "cambi", "reprogram", "cerr", "visto",
              "estado", "ha tenido", "ha estado", "servicio", "proxima", "siguiente", "pendiente"]
             + [normal for e in ESPECIALIDADES.values() for normal in [e.lower()[:6]]],
    "MOVIMIENTOS_CITAS": ["falt", "acud", "atend", "anul", "cambi", "reprogram", "cerr", "por que", "motivo"],
    "COMUNICACIONES": ["aviso", "sms", "llam", "email", "correo", "carta", "app", "contact", "notific", "llego"],
    "DERIVACIONES_AP": ["deriv", "cabecera", "primaria", "motivo", "por que", "manda", "envia"],
}


def _normal(t: str) -> str:
    return unicodedata.normalize("NFKD", t).encode("ascii", "ignore").decode().lower()


def _momento(f: Fila) -> datetime:
    v = f.campos.get("fecha") or f.campos.get("realizada") or f.campos.get("programada")
    if isinstance(v, datetime):
        return v
    if isinstance(v, date):
        return datetime(v.year, v.month, v.day)
    return datetime.min


def _resumen(filas: list[Fila]) -> Fila:
    """Toda la historia en una línea, contada con los datos, no por el modelo."""
    reales = [f for f in filas if not f.campos.get("vacia")]
    de = lambda t: [f for f in reales if f.tabla == t]  # noqa: E731
    citas, movs, pruebas, avisos = de("CITAS"), de("MOVIMIENTOS_CITAS"), de("PRUEBAS"), de("COMUNICACIONES")
    estados = Counter(f.campos.get("estado") for f in citas)
    servicios = Counter(ESPECIALIDADES.get(f.campos.get("especialidad"), "otro") for f in citas)
    años = sorted(_momento(f).year for f in citas if _momento(f) != datetime.min)
    pr = Counter(f.campos.get("estado") for f in pruebas)
    pendientes = sorted((f for f in pruebas if f.campos.get("estado") == "PROGRAMADA"), key=_momento)
    cuales = "; pendientes: " + ", ".join(
        f"[{f.ref}] {str(f.campos.get('tipo', '')).lower()} el {_momento(f):%d/%m/%Y}" for f in pendientes[:6]) \
        if pendientes else ""
    partes = [
        f"{len(citas)} citas" + (f" entre {años[0]} y {años[-1]}" if años else "")
        + (f" ({', '.join(f'{n} en {s}' for s, n in servicios.most_common())})" if servicios else ""),
        f"estado de las citas: {', '.join(f'{n} {e}' for e, n in estados.most_common())}" if estados else "",
        f"{sum(f.campos.get('tipo') == 'NO_REALIZADA' for f in movs)} cerradas sin atenderle, "
        f"{sum(f.campos.get('tipo') == 'REPROGRAMACION' for f in movs)} reprogramadas, "
        f"{sum(f.campos.get('tipo') == 'ANULACION' for f in movs)} anuladas",
        f"{len(pruebas)} pruebas ({pr['PROGRAMADA']} pendientes, {pr['REALIZADA']} hechas, {pr['NO_REALIZADA']} sin hacer){cuales}",
        f"{len(avisos)} avisos ({sum(f.campos.get('resultado') == 'FALLIDO' for f in avisos)} no le llegaron)",
        f"{len(de('DERIVACIONES_AP'))} derivaciones de Primaria",
    ]
    return Fila("R1", "RESUMEN", "", "Resumen de toda su historia, contado con los datos: "
                + "; ".join(p for p in partes if p) + ".")


def seleccionar(filas: list[Fila], pregunta: str, contexto: int = CONTEXTO) -> list[Fila]:
    """Las filas que van al modelo para esta pregunta, dentro del presupuesto."""
    presupuesto = max(1500, (contexto - 700) * CARACTERES_POR_TOKEN)
    total = sum(len(f.texto) + 8 for f in filas)
    if total <= presupuesto:
        return filas                                        # cabe todo: como siempre

    q = _normal(pregunta)
    temas = [t for t, claves in TEMAS.items() if any(c in q for c in claves)] or list(TEMAS)
    servicio = [e for e in ESPECIALIDADES.values() if _normal(e)[:6] in q]

    def orden(f: Fila):
        e = f.campos.get("estado")
        primero = (f.tabla == "PRUEBAS" and e == "PROGRAMADA") or (f.tabla == "CITAS" and e == "CITADA")
        del_servicio = bool(servicio) and ESPECIALIDADES.get(f.campos.get("especialidad")) in servicio
        return (not primero, not del_servicio, -_momento(f).timestamp() if _momento(f) != datetime.min else 0)

    elegidas = [f for f in filas if f.tabla == "PACIENTES"] + [_resumen(filas)]
    usado = sum(len(f.texto) + 8 for f in elegidas)
    candidatas = sorted((f for f in filas if f.tabla in temas), key=orden)
    fuera = Counter()
    for f in candidatas:
        if usado + len(f.texto) + 8 <= presupuesto:
            elegidas.append(f)
            usado += len(f.texto) + 8
        else:
            fuera[f.tabla] += 1
    fuera.update(f.tabla for f in filas if f.tabla not in temas and f.tabla != "PACIENTES")
    if fuera:
        nombres = {"CITAS": "citas", "MOVIMIENTOS_CITAS": "cambios de cita", "PRUEBAS": "pruebas",
                   "COMUNICACIONES": "avisos", "DERIVACIONES_AP": "derivaciones"}
        elegidas.append(Fila("R2", "RESUMEN", "",
            "Para que quepa, no van aquí: " + ", ".join(f"{n} filas de {nombres.get(t, t)}" for t, n in fuera.items())
            + ". Lo que va es lo pendiente y lo más reciente. Si preguntan por algo anterior o por otro tema, "
            "dilo y responde solo con lo que hay."))
    return elegidas
