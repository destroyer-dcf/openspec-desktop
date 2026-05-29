## Why

El sidebar actual no prioriza lectura rápida de proyectos ni permite desvincular carpetas de forma directa. Mejorar su estructura a tarjetas y mover acciones al footer optimiza uso diario y claridad, especialmente con muchos proyectos.

## What Changes

- Mostrar listado de proyectos del sidebar como tarjetas.
- En cada tarjeta: nombre del proyecto en negrita y, debajo, path completo en tipografía más pequeña.
- Añadir acción de desvincular proyecto con botón `X` en cada tarjeta.
- Mantener tarjeta seleccionada con estado azul.
- Reubicar/cambiar el botón de configuración al footer del sidebar.
- Asegurar comportamiento responsive del sidebar en anchos reducidos.

## Capabilities

### New Capabilities
- `tarjetas-proyectos-sidebar`: Presentación y gestión de proyectos como tarjetas con metadatos y acción de desvincular.

### Modified Capabilities
- `sidebar-proyectos`: Se modifica layout y acciones del sidebar, incluyendo footer y responsive.

## Impact

- Frontend del componente `Sidebar` y flujo de selección/desvinculación.
- Gestión de proyectos persistidos para soportar quitar carpeta de la app.
- Estilos responsive del panel lateral.
