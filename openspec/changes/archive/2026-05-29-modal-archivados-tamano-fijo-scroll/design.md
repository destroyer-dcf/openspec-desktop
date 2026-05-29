## Context

El modal de consulta de cambios archivados actualmente adapta su tamaño al contenido markdown seleccionado. Cuando el documento es grande, el contenedor del modal crece y degrada la usabilidad. La app es de escritorio, por lo que se busca una ventana modal estable con área interna desplazable para contenido largo.

## Goals / Non-Goals

**Goals:**
- Mantener tamaño estándar del modal de consulta independientemente del tamaño del documento.
- Garantizar scroll interno en el panel de visualización markdown.
- Preservar visibilidad de cabecera, selector/listado de documentos y acciones de cierre.

**Non-Goals:**
- Cambiar el backend de lectura de documentos archivados.
- Añadir edición, versionado o búsqueda full-text en documentos archivados.
- Rediseñar visual completo del modal más allá de tamaño y overflow.

## Decisions

1. Definir dimensiones fijas con límites de viewport para el contenedor del modal.
- Decisión: usar ancho/alto estándar y `max-height`/`max-width` para evitar overflow global.
- Alternativa descartada: cálculo dinámico por longitud de documento; vuelve a causar crecimiento impredecible.

2. Separar layout del modal en regiones fijas + región scrollable.
- Decisión: cabecera y barra de acciones sin scroll; cuerpo de documento con `overflow: auto`.
- Alternativa descartada: scroll del modal completo; pierde contexto y controles.

3. Mantener render markdown existente y encapsularlo en wrapper con altura controlada.
- Decisión: no tocar parser/render, solo constraints de contenedor.
- Alternativa descartada: truncado de documento; rompe consulta completa.

## Risks / Trade-offs

- [Riesgo] Altura fija demasiado pequeña para ciertos tamaños de fuente -> Mitigación: usar tamaño estándar pero con `max-height` relativo al viewport.
- [Riesgo] Doble scroll (modal + documento) -> Mitigación: desactivar crecimiento del modal y dejar un único scroll en panel de documento.
- [Riesgo] Regresión visual en tema dark/light -> Mitigación: reutilizar variables de tema existentes y validar ambos temas.
