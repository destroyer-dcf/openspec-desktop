## Why

Actualmente se puede archivar propuestas seleccionadas, pero no eliminarlas directamente desde el panel, lo que fuerza pasos manuales en el sistema de archivos. Añadir eliminación por selección mejora eficiencia y control del backlog.

## What Changes

- Añadir botón de “eliminar propuestas seleccionadas” en el panel de propuestas.
- Mantener estilo visual coherente con acciones existentes: icon-only + tooltip.
- Permitir eliminación en lote de propuestas seleccionadas del estado visible (activa/archivada según filtro).
- Refrescar grid y limpiar selección tras eliminar.

## Capabilities

### New Capabilities
- `eliminacion-propuestas-seleccionadas`: Eliminación múltiple de propuestas seleccionadas desde UI.

### Modified Capabilities
- `gestion-propuestas`: Se amplía el set de acciones masivas con eliminación seleccionada.

## Impact

- Frontend del panel de propuestas (nueva acción y estados).
- Backend Tauri para borrado de archivos de propuestas por lote.
- Validación de UX para evitar eliminaciones accidentales.
