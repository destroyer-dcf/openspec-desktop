## Why

El flujo de propuestas tiene acciones globales que añaden fricción visual (selección + botones de lote) para operaciones que se realizan por tarjeta. Además, mantener selector de color de botones en configuración genera inconsistencia con el estilo base del tema.

## What Changes

- Mover acciones de propuestas (archivar y borrar) desde la barra global a cada tarjeta individual.
- Eliminar el checkbox/flag de selección en tarjetas de propuestas.
- Mostrar extracto de contenido (2 líneas aprox.) debajo del título de cada propuesta.
- Eliminar del modal de configuración el selector de color de botones.
- Eliminar variantes de color verde de botones y usar colores por defecto del tema (incluyendo hover).

## Capabilities

### New Capabilities
- `resumen-propuesta-en-tarjeta`: Mostrar preview textual breve del contenido markdown en cada tarjeta de propuesta.

### Modified Capabilities
- `gestion-propuestas`: Cambia acciones de propuestas a nivel de tarjeta y elimina selección múltiple.
- `ux-acciones-propuestas`: Reordena jerarquía de acciones (archivar/borrar) por tarjeta con tooltips/iconos.
- `configuracion-densidad-y-color`: Elimina control de color de botones del modal de configuración.
- `ui-controles-botones`: Normaliza botones al estilo base de tema, sin variante verde.

## Impact

- `Dashboard.svelte` en panel de propuestas (acciones y rendering de extracto).
- `SettingsModal.svelte` y `+page.svelte` para retirar preferencias de color de botones.
- Tokens/estilos globales de botones en tema claro/oscuro.
- Lógica de archivado/borrado de propuestas para operar por tarjeta.
