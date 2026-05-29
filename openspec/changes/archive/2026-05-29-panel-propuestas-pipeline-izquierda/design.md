## Context

Actualmente el dashboard muestra cambios activos y archivados en paneles principales, mientras propuestas vive como bloque secundario bajo cambios activos. Para leer estado de proyecto como flujo, propuestas debe convertirse en primera etapa visible y lateral dentro del mismo nivel visual que los demás paneles.

## Goals / Non-Goals

**Goals:**
- Añadir panel de propuestas con misma jerarquía visual de paneles de cambios.
- Reubicar propuestas a la izquierda de cambios activos (orden pipeline).
- Mantener acciones existentes de propuestas (crear, filtrar, archivar, eliminar, editar, copiar).
- Mantener legibilidad y estabilidad de layout en escritorio.

**Non-Goals:**
- Cambiar modelo de datos de propuestas.
- Alterar lógica de archivado de cambios OpenSpec.
- Crear nuevas capacidades backend para propuestas.

## Decisions

1. Estructura pipeline en columnas.
- Decisión: usar layout de tres columnas/zonas en orden: Propuestas -> Cambios activos -> Cambios archivados.
- Alternativa descartada: mantener propuestas debajo de cambios activos con título decorativo; no cumple pipeline.

2. Reuso de bloque funcional de propuestas.
- Decisión: mover bloque actual al nuevo panel sin romper controles existentes.
- Alternativa descartada: reimplementar panel desde cero; riesgo de regresiones.

3. Consistencia visual de paneles.
- Decisión: aplicar cabeceras, bordes y espaciados equivalentes entre los tres paneles.
- Alternativa descartada: estilos diferenciados fuertes; diluye lectura de etapas equivalentes.

## Risks / Trade-offs

- [Riesgo] Más densidad horizontal en dashboard -> Mitigación: ajustar anchos mínimos y distribución flexible.
- [Riesgo] Acciones de propuestas podrían verse comprimidas -> Mitigación: mantener barra de acciones compacta icon-only ya existente.
- [Riesgo] Regresión en orden de scroll del contenido -> Mitigación: validar scroll en cada panel y en contenedor general.
