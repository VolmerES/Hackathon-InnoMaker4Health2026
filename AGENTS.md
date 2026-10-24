# Instrucciones para asistentes de código

## El nombre del producto: Medicci

El producto se llama **Medicci**, con dos ces, y es el único nombre que se usa.
Vale para todo: documentación, interfaz, comentarios, nombres de variables,
ficheros, clases CSS, tokens de marca, servicios de Docker y mensajes de commit.

| Dónde | Cómo se escribe |
|---|---|
| Texto | Medicci |
| Código, ficheros, CSS | `medicci` (`medicci.conf`, `--verde-medicci`, `medicci-rs`) |
| Variables de entorno y constantes | `MEDICCI_*` (`MEDICCI_URL`, `TABLAS_MEDICCI`) |

**No se usan** los nombres antiguos «Medici» (con una ce) ni «Jano», tampoco
para colores, tokens o comentarios («verde Medicci», no «verde Medici»).

Para comprobarlo antes de un commit, esto no tiene que sacar nada:

```bash
git grep -n -i -E "(^|[^a-z])(medici|jano)([^a-z]|$)" -- ':!AGENTS.md'
```
