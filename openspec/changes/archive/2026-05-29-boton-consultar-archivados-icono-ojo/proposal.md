## Why

El botón textual “Consultar” en cambios archivados ocupa espacio visual y rompe consistencia con otras acciones icon-only de la UI. Se busca un botón compacto de icono (ojo) con accesibilidad clara.

## What Changes

- Reemplazar botón textual “Consultar” por botón solo icono (ojo) en tarjetas de cambios archivados.
- Mantener misma acción funcional de apertura del modal de consulta.
- Añadir `aria-label` y `title/tooltip` para accesibilidad y claridad.
- Ajustar estilos para coherencia con patrón icon-only existente.

## Capabilities

### New Capabilities
- `accion-consultar-archivado-icono`: acción de consulta de cambios archivados mediante botón icon-only tipo ojo.

### Modified Capabilities
- `consulta-cambios-archivados`: actualización de interacción de entrada al modal desde botón iconográfico.
- `ui-controles-botones`: consistencia visual de controles icon-only en paneles de estado.

## Impact

- Frontend del grid de cambios archivados en dashboard.
- Estilos del botón de acción de consulta.
- Sin cambios backend ni estructura de datos.
