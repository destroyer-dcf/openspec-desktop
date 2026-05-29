## Why

La implementación actual de propuestas usa la ruta `opencode/propose`, pero el proyecto requiere que la persistencia quede bajo `openspec` para mantener consistencia con el resto de artefactos OpenSpec. Esta corrección evita dispersión de datos y facilita mantenimiento.

## What Changes

- Cambiar la ruta de almacenamiento de propuestas activas desde `opencode/propose/actives` a `openspec/propose/actives`.
- Cambiar la ruta de archivado desde `opencode/propose/archived` a `openspec/propose/archived`.
- Actualizar lectura, escritura, listado y archivado múltiple para usar exclusivamente rutas bajo `openspec`.
- Alinear textos/validaciones relacionadas con las rutas nuevas.

## Capabilities

### New Capabilities
- `ruta-propuestas-openspec`: Normalización de persistencia de propuestas dentro de `openspec/propose`.

### Modified Capabilities
- `gestion-propuestas`: Se modifican los requisitos de rutas de almacenamiento y archivado de propuestas.

## Impact

- Backend Tauri: comandos de propuestas (list/get/save/archive).
- Validaciones funcionales y documentación del flujo de propuestas.
- Estructura de carpetas persistidas en cada proyecto.
