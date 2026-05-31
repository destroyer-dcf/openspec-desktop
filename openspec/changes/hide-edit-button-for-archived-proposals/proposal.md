## Why

Las propuestas archivadas no deben comportarse como elementos editables, pero actualmente se mantiene visible la acción de editar en el panel de propuestas. Esto genera una UX inconsistente y provoca intentos de edición sobre contenido que debería ser solo de consulta.

## What Changes

- Ocultar la acción de editar para propuestas cuyo estado sea archivado en el panel de propuestas.
- Mantener visible la acción de editar únicamente para propuestas activas/no archivadas.
- Ajustar reglas de render de acciones por estado de propuesta para evitar ambigüedad visual.

## Capabilities

### New Capabilities
- (none)

### Modified Capabilities
- `ux-acciones-propuestas`: La disponibilidad del botón de edición pasa a depender del estado archivado/no archivado de la propuesta.

## Impact

- Afecta componentes UI del panel de propuestas y su lógica condicional de acciones.
- No cambia APIs externas ni estructura de datos persistida.
- Impacto esperado limitado a comportamiento visual y de interacción en tarjetas/listado de propuestas.
