## Why

Los tres paneles del pipeline muestran elementos internos en múltiples columnas y eso rompe lectura secuencial vertical. Se necesita que cada panel tenga una sola columna interna para ordenar mejor el flujo visual.

## What Changes

- Forzar layout interno de una sola columna en panel Propuestas.
- Forzar layout interno de una sola columna en panel Cambios activos.
- Forzar layout interno de una sola columna en panel Cambios archivados.
- Ajustar estilos de tarjetas/listas para mantener legibilidad y espaciado vertical consistente.

## Capabilities

### New Capabilities
- `layout-paneles-columna-unica`: regla de diseño para que cada panel del pipeline renderice su contenido en una sola columna interna.

### Modified Capabilities
- `dashboard-estado`: cambio de disposición interna de paneles en pipeline a columna única por panel.
- `gestion-propuestas`: adaptación del grid de propuestas a lista vertical en una sola columna dentro de su panel.

## Impact

- Frontend del dashboard en estructura y CSS de paneles.
- Estilos de listas de propuestas y archivados.
- Sin cambios en backend ni modelo de datos.
