## Why

El panel de cambios activos no mantiene consistencia visual con archivados y obliga a navegación menos fluida para consultar/editar documentos. Hace falta un flujo modal unificado, progreso compacto en círculo, y utilidades rápidas (copiar nombre de tarea y marcar checkboxes desde vista).

## What Changes

- Rediseñar tarjeta de cambio activo para que sea equivalente visualmente a tarjeta de archivado.
- Sustituir acción principal por icono de ver que abre modal equivalente al de archivados.
- En modal de activos: modo preview por defecto y toggle preview/edición con icono; permitir guardar cambios.
- Sustituir barra de progreso lineal por porcentaje en círculo en esquina superior derecha de la tarjeta activa.
- Mantener iconos de artifacts actuales y añadir visual de tareas con el mismo patrón de iconografía.
- Aplicar estilo distintivo a headers markdown en editor/preview.
- Permitir marcar tareas desde visualización markdown.
- Añadir acción para copiar nombre de tarea desde la tarjeta y facilitar uso con IA.

## Capabilities

### New Capabilities
- `modal-activo-vista-edicion`: consulta y edición de documentos de cambios activos en modal con toggle preview/editar.

### Modified Capabilities
- `dashboard-estado`: tarjetas de cambios activos estilo archivados, progreso circular por tarjeta y acción de copia de nombre de tarea.
- `editor-markdown`: diferenciación visual de headers markdown y marcado de tareas desde visualización.
- `progreso-proyecto`: sustitución de barra lineal por indicador circular porcentual en tarjeta de cambio activo.

## Impact

- Componentes frontend de dashboard, modal de documentos y editor markdown.
- Estilos de tarjetas, iconos y tipografía markdown.
- Reutilización de comandos existentes de lectura/escritura, sin cambios backend nuevos.
