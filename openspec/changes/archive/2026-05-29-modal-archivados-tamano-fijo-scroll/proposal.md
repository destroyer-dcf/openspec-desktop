## Why

El modal de consulta de cambios archivados crece según el tamaño del markdown seleccionado y rompe la experiencia visual. Se necesita un tamaño de ventana estable y que el contenido largo se navegue con scroll interno.

## What Changes

- Fijar dimensiones estándar del modal de consulta de cambios archivados dentro de límites de viewport.
- Evitar que el contenedor del modal se expanda al cargar documentos markdown extensos.
- Habilitar scroll vertical en el área de documento/render markdown.
- Mantener header, acciones y cierre del modal siempre visibles mientras se desplaza el contenido.

## Capabilities

### New Capabilities
- `modal-consulta-archivados-scroll`: comportamiento de visualización con tamaño fijo y scroll interno para lectura de markdown archivado.

### Modified Capabilities
- `consulta-cambios-archivados`: ajuste de requisitos de UI del modal para impedir crecimiento dinámico por contenido y garantizar legibilidad.

## Impact

- Frontend Svelte del modal de cambios archivados y estilos CSS asociados.
- Posible ajuste en contenedor de vista previa markdown dentro del modal.
- Sin cambios de API backend ni dependencias externas.
