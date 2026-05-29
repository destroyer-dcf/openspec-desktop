## Why

Cuando un cambio está completado y la UI ya bloquea modificación real, mantener visibles botones de editar y guardar crea una expectativa falsa y confunde el flujo. Desactivar explícitamente esas acciones mejora claridad de estado y evita intentos inválidos.

## What Changes

- Desactivar controles de editar/guardar en visores/modales markdown cuando el cambio está en estado completo (no editable).
- Mantener visibles los botones solo si aportan contexto, pero con estado disabled y feedback visual claro.
- Evitar cualquier intento de guardado desde UI en cambios completos.

## Capabilities

### New Capabilities
- `bloqueo-acciones-edicion-cambio-completo`: Estado de UI que desactiva acciones de edición/guardado para cambios no modificables.

### Modified Capabilities
- `editor-markdown`: Ajusta comportamiento de acciones de edición/guardado según `canModify`.
- `dashboard-estado`: Refuerza señal visual de no edición en cambios completos al abrir modal de activos.

## Impact

- Componentes de modal/visor markdown de cambios activos.
- Estados de botones de editar/guardar y validación de handlers de guardado.
- Estilos disabled y mensajes de apoyo para no editable.
