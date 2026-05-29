## Context

El flujo actual permite seleccionar artifacts cuya ruta puede ser carpeta (caso `specs/`), lo que termina en intento de lectura de fichero y error `Is a directory (os error 21)`. Además, el layout en ciertos anchos/estados provoca superposición visual entre sidebar y área de contenido.

## Goals / Non-Goals

**Goals:**
- Bloquear lectura de directorios como documentos.
- Dar feedback de UI claro para rutas no editables.
- Asegurar separación física sidebar/content en dashboard y detalle.

**Non-Goals:**
- No rediseñar navegación completa.
- No cambiar modelo de datos OpenSpec.

## Decisions

- Validar tipo de ruta antes de `read_file` (fichero vs directorio).
- Introducir estado explícito de "no editable" para artifacts de carpeta.
- Ajustar grid/flex con `min-width: 0`, `overflow` y límites de columna para eliminar solape.
- Mantener estilos compactos actuales; solo corregir colisión estructural.

## Risks / Trade-offs

- [Falso positivo no-editable] → Mitigación: mensaje contextual y ruta visible.
- [Regresión responsive] → Mitigación: validar desktop/móvil con listado largo.
- [Cambios CSS cruzados] → Mitigación: tocar solo contenedores layout y paneles afectados.
