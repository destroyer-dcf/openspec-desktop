## Why

El rosco de porcentaje en las tarjetas de cambios activos ocupa espacio y añade ruido visual en densidades altas. Un indicador textual grande y claro mejora legibilidad, reduce carga visual y mantiene la misma información de progreso.

## What Changes

- Quitar el indicador circular de porcentaje dentro de cada tarjeta de cambio activo.
- Mostrar el progreso como texto destacado (porcentaje en negrita, tamaño mayor) a la derecha de los botones de acción.
- Evaluar adorno visual ligero (icono de progreso) junto al texto para mayor claridad sin recargar.
- Mantener el cálculo existente de progreso y coherencia con `X/Y tareas`.

## Capabilities

### New Capabilities
- `indicador-textual-progreso-tarjeta`: Nuevo patrón visual de progreso textual en tarjetas activas.

### Modified Capabilities
- `dashboard-estado`: Cambia el componente visual del progreso por tarjeta (de rosco a texto destacado).
- `progreso-proyecto`: Mantiene cálculo de progreso, cambiando solo su presentación en tarjetas.

## Impact

- `Dashboard.svelte` (layout de cabecera y zona de acciones por tarjeta activa).
- Posible ajuste en uso de `CircularProgress` dentro de tarjeta activa (ya no necesario ahí).
- CSS de tarjetas para jerarquía tipográfica y alineación derecha.
