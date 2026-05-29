## Why

La visualización actual de cambios archivados no facilita exploración rápida ni consulta estructurada de su documentación histórica. Se necesita una vista más escaneable y una consulta segura en modo solo lectura para evitar modificaciones accidentales.

## What Changes

- Mostrar los cambios archivados en formato grid dentro del dashboard.
- Incluir campo de fecha de aplicación/archivo visible por cada tarjeta de cambio archivado.
- Añadir botón de consulta por elemento archivado.
- Abrir modal con listado de documentos del cambio archivado y visualización en modo solo lectura.
- Bloquear edición en la consulta de documentos archivados.

## Capabilities

### New Capabilities
- `consulta-cambios-archivados`: Consulta en modal de cambios archivados con navegación de documentos en modo solo lectura.

### Modified Capabilities
- `dashboard-estado`: La sección de cambios archivados pasa de listado lineal a grid con fecha de aplicación y acción de consulta.

## Impact

- UI del dashboard en sección de archivados (layout, tarjetas, acciones).
- UI modal de documentos archivados (listado y visor readonly).
- Lógica de carga de metadatos de archivado (fecha) y lectura de archivos desde `changes/archive`.
