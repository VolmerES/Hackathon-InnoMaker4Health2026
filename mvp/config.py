"""Dónde está cada cosa: un único fichero medicci.conf, o variables de entorno.

En una licitación, dónde se instala Medicci lo decide quien licita (el CPD del
hospital, el de la Comunidad, la nube del pliego). Por eso todas las direcciones
salen de aquí y no del código. Se busca medicci.conf en MEDICCI_CONF, en la raíz
del repositorio y en la carpeta desde la que se arranca. Lo que esté en el
entorno manda sobre el fichero. Formato: CLAVE=valor, una por línea; # comenta.
"""

import os
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent


def _cargar() -> Path | None:
    candidatos = [os.environ.get("MEDICCI_CONF"), RAIZ / "medicci.conf", Path.cwd() / "medicci.conf"]
    for c in candidatos:
        ruta = Path(c) if c else None
        if ruta and ruta.is_file():
            for linea in ruta.read_text(encoding="utf-8").splitlines():
                linea = linea.split(" #")[0].strip()
                if linea and not linea.startswith("#") and "=" in linea:
                    clave, valor = linea.split("=", 1)
                    os.environ.setdefault(clave.strip(), valor.strip())
            return ruta
    return None


FICHERO = _cargar()


def valor(clave: str, por_defecto: str) -> str:
    return os.environ.get(clave) or por_defecto
