## Why

Actualmente no existe un flujo interno para capturar y gestionar propuestas funcionales o bugs desde la propia UI, lo que dispersa ideas y dificulta su seguimiento. Incorporar un panel dedicado de propuestas mejora trazabilidad y prepara un backlog estructurado antes de convertirlo en cambios OpenSpec.

## What Changes

- Añadir en `content`, debajo de “Cambios Activos”, un panel “Propuestas” en formato grid.
- Mostrar por propuesta: fecha de creación, nombre y botón “Modificar”.
- Añadir botón para crear nuevas propuestas.
- Implementar modal “Propuesta” con:
  - campo nombre identificador,
  - combo tipo (`Feature`/`Bug`),
  - editor Markdown,
  - acciones guardar y descartar.
- Guardar propuestas como archivos Markdown en `opencode/propose/actives` y soportar archivado en `opencode/propose/archived`.
- Permitir selección múltiple en grid para archivar varias propuestas en bloque.
- Definir formato Markdown con header que incluya tipo y nombre.

## Capabilities

### New Capabilities
- `gestion-propuestas`: Creación, edición, listado y archivado múltiple de propuestas en markdown bajo `opencode/propose`.

### Modified Capabilities
- `dashboard-estado`: Se añade panel de propuestas debajo de cambios activos, con grid y acciones de gestión.

## Impact

- Frontend: dashboard, modal de propuestas y estado de selección múltiple.
- Backend/Tauri: lectura/escritura/movimiento de archivos en `opencode/propose/actives` y `opencode/propose/archived`.
- Modelo de datos: metadatos de propuesta (nombre, tipo, fecha, ruta, estado activo/archivado).
