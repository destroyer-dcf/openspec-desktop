## Why

Las tarjetas de cambios activos tienen detalles visuales y de información mal ubicados respecto al diseño esperado. Además, la jerarquía tipográfica de headers markdown usa color azul y debería ser negro en negrita para mayor claridad.

## What Changes

- Reubicar botones de acción de tarjetas de cambios activos debajo de la fila de iconos de artifacts.
- Ajustar visual de tasks para mostrar solo icono (sin texto X/X junto al icono).
- Mantener progreso por porcentaje circular sin volver a barra lineal.
- Cambiar estilo de headers markdown en visualización/edición a negro con negrita (sin azul).

## Capabilities

### New Capabilities
- `ajuste-fino-tarjetas-activos`: normalización de micro-layout de tarjetas activas (orden de iconos y botones).

### Modified Capabilities
- `dashboard-estado`: ajuste de posición de acciones y presentación de tasks en tarjetas activas.
- `editor-markdown`: actualización de estilo de encabezados markdown a negro bold.

## Impact

- Componentes frontend de dashboard y modal/editor markdown.
- CSS de tarjetas de cambios activos y estilos de contenido markdown.
- Sin cambios en backend ni modelo de datos.
