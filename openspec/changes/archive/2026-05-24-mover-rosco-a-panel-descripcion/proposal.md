## Why

El indicador circular de resumen global está separado del panel de descripción del proyecto y divide innecesariamente la lectura del estado principal. Además, los datos del proyecto en una sola línea reducen legibilidad.

## What Changes

- Mover el rosco de resumen global al panel de descripción del proyecto, alineado a la derecha.
- Reorganizar los datos de proyecto para mostrarlos línea a línea, no en una sola fila horizontal.
- Mantener consistencia visual del panel con layout de dos zonas: datos (izquierda) y rosco (derecha).
- Ajustar responsive desktop del panel para evitar solapes con el resto del dashboard.

## Capabilities

### New Capabilities
- `layout-resumen-proyecto-integrado`: integración del resumen global circular dentro del panel de descripción de proyecto.

### Modified Capabilities
- `dashboard-estado`: cambio de distribución del bloque de resumen global y presentación multilinea de metadatos de proyecto.
- `panel-descripcion-proyecto`: actualización de estructura visual para mostrar campos en líneas separadas y reservar columna del rosco.

## Impact

- Componentes frontend de dashboard y panel de descripción.
- Estilos CSS del bloque de contexto/resumen del proyecto.
- Sin cambios en backend ni formato de datos.
