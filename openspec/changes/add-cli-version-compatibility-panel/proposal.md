## Why

La aplicación no expone de forma explícita si la versión instalada de `opencode` CLI es compatible con la versión actual de Opencode Desktop. Esto genera incertidumbre operativa y errores difíciles de diagnosticar cuando hay desalineación de versiones.

## What Changes

- Añadir un archivo interno mantenido por desarrollo con las reglas de compatibilidad de versiones de `opencode` CLI admitidas por la aplicación.
- Soportar expresiones de rango en reglas de versión (incluyendo operadores como `<`, `<=`, `>`, `>=`).
- Evaluar en runtime la versión de `opencode` CLI instalada contra las reglas configuradas.
- Mostrar en la UI un panel fijo en la parte superior derecha con el texto `Opencode Desktop`, la versión CLI detectada y el estado de validación.
- Colorear el panel según estado:
- Verde: versión validada.
- Rojo: versión no validada.
- Amarillo: estado indeterminado (por ejemplo, versión no detectable o configuración inválida/incompleta).

## Capabilities

### New Capabilities
- `compatibilidad-version-cli`: Gestión declarativa y validación visual de compatibilidad entre Opencode Desktop y versiones de `opencode` CLI.

### Modified Capabilities
- (none)

## Impact

- Afecta configuración interna de la app (nuevo archivo de compatibilidad de versiones).
- Afecta capa de detección/evaluación de versión CLI.
- Afecta UI principal al incorporar un nuevo panel de estado en la esquina superior derecha.
- Introduce dependencia funcional en parsing/evaluación semántica de rangos de versión.
