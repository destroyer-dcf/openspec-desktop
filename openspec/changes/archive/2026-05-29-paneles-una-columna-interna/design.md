## Context

El pipeline actual usa tres paneles principales, pero algunos contenidos internos se renderizan en varias columnas (por ejemplo grids auto-fill), lo que dificulta lectura lineal. El objetivo es que cada panel tenga una única columna interna vertical.

## Goals / Non-Goals

**Goals:**
- Garantizar una sola columna de contenido dentro de Propuestas.
- Garantizar una sola columna de contenido dentro de Cambios activos.
- Garantizar una sola columna de contenido dentro de Cambios archivados.
- Mantener acciones y funcionalidad actual sin cambios de backend.

**Non-Goals:**
- Cambiar orden de paneles del pipeline.
- Cambiar lógica de propuestas/archivados.
- Rediseñar estilos globales fuera del layout interno de columnas.

## Decisions

1. Sustituir grids internos multi-columna por listas/stack verticales.
- Decisión: `grid-template-columns: 1fr` en listados internos de paneles.
- Alternativa descartada: mantener auto-fill y limitar ancho; sigue produciendo múltiples columnas.

2. Mantener tarjetas existentes, solo cambia su flujo vertical.
- Decisión: no tocar semántica de tarjetas ni acciones.
- Alternativa descartada: convertir tarjetas a tablas; rompe estilo actual.

3. Mantener scroll por panel.
- Decisión: cada panel sigue gestionando su contenido sin solape.
- Alternativa descartada: único scroll global sin control interno.

## Risks / Trade-offs

- [Riesgo] Más altura total por panel -> Mitigación: conservar compactación y scroll.
- [Riesgo] Pérdida de densidad visual en propuestas -> Mitigación: mantener información resumida por tarjeta.
- [Riesgo] Regresión en estilos responsive previos -> Mitigación: validar con ancho mínimo de escritorio.
