## Why

El modal de inicialización fija el `schema` en `spec-driven` sin permitir elección del usuario. Esto limita la flexibilidad y obliga a editar manualmente `config.yaml` tras inicializar.

## What Changes

- Añadir un combo de selección de `schema` en el modal `Inicializar proyecto OpenSpec`.
- Cargar las opciones disponibles ejecutando `openspec templates` y parseando los schemas detectados.
- Usar el valor seleccionado para escribir `schema` en `openspec/config.yaml` al ejecutar la inicialización.
- Mantener fallback seguro cuando no se puedan obtener templates (usar `spec-driven`).

## Capabilities

### New Capabilities
- `selector-schema-init-proyecto`: Permitir seleccionar schema OpenSpec durante la inicialización del proyecto.

### Modified Capabilities
- `wizard-init-proyecto`: Extender el formulario de inicialización para incluir selección de schema y persistirlo en config.

## Impact

- Frontend Svelte: nuevo campo combo en modal de inicialización.
- Backend Rust: comando para obtener schemas disponibles y aplicación del schema elegido en `config.yaml`.
- UX: flujo de inicialización más flexible sin edición manual posterior.
