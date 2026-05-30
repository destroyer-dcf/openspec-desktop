## Why

En el panel de descripción del proyecto faltan datos clave de contexto técnico. Mostrar el `schema` actual desde `openspec/config.yaml` mejora visibilidad del modo de trabajo activo del proyecto.

## What Changes

- Leer el campo `schema` desde `openspec/config.yaml` del proyecto activo.
- Mostrar el valor de `schema` dentro del panel de descripción del proyecto en `content`.
- Definir comportamiento fallback cuando `schema` no exista o el archivo no sea legible.
- Mantener consistencia visual con el resto de líneas de metadatos del panel.

## Capabilities

### New Capabilities
- `schema-en-descripcion-proyecto`: Mostrar el esquema OpenSpec activo en la descripción del proyecto.

### Modified Capabilities
- `panel-descripcion-proyecto`: Extender los campos visibles del resumen para incluir `schema`.

## Impact

- Backend Rust: enriquecimiento de datos de proyecto desde `config.yaml`.
- Frontend Svelte: render de nueva línea informativa en panel de descripción.
- UX: mejora de trazabilidad del contexto OpenSpec del proyecto activo.
