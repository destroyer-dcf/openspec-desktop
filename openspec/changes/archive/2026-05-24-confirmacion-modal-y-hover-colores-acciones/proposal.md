## Why

El flujo de borrado actualmente no exige confirmación en modal y las acciones visuales no diferencian claramente intención destructiva/positiva mediante hover. Ajustar estas reglas mejora seguridad y claridad de UX.

## What Changes

- Solicitar borrado de propuestas seleccionadas mediante modal de confirmación (no `confirm()` nativo).
- Mantener botón de borrado en estilo base neutro y aplicar color rojo solo en hover.
- Mantener botón de archivado en estilo base neutro y aplicar color verde solo en hover.
- Conservar patrón icon-only + tooltip para ambos botones.

## Capabilities

### New Capabilities
- `confirmacion-modal-borrado-propuestas`: Confirmación explícita por modal para acciones destructivas en propuestas.

### Modified Capabilities
- `ux-panel-propuestas`: Estados visuales hover para acciones masivas (archivar/borrar) con semántica de color.

## Impact

- Frontend del panel de propuestas y estilos CSS de acciones.
- Lógica de confirmación previa para eliminación.
- Validación visual/funcional de estados hover y confirmación.
