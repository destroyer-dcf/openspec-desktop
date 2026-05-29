## Why

El panel de propuestas necesita más control visual y operativo para manejar volumen: filtrar por tipo/estado, acciones compactas y copiar contenido para uso con IA. La UX actual obliga a más clics y ocupa más espacio del necesario.

## What Changes

- Añadir dos combos horizontales y compactos junto a acciones del panel:
  - filtro por tipo de propuesta,
  - filtro por estado (activas/archivadas).
- Cambiar botones de acciones a icon-only con tooltip:
  - añadir propuesta,
  - archivar seleccionadas,
  - modificar propuesta.
- Añadir acción icon-only con tooltip para copiar al portapapeles el markdown completo de una propuesta.
- Ajustar grid para reflejar filtros combinados sin romper selección múltiple.
- Mantener formato markdown con metadatos mínimos (frontmatter) para tipo/fecha, simplificando filtrado fiable.

## Capabilities

### New Capabilities
- `ux-panel-propuestas`: Filtros compactos y acciones icon-only con tooltips para gestión eficiente del panel.

### Modified Capabilities
- `gestion-propuestas`: Se amplía con filtros por tipo/estado y acción de copiar markdown al portapapeles.

## Impact

- Frontend del dashboard y modal de propuestas.
- Estado de UI para filtros y selección.
- Integración con API de portapapeles.
