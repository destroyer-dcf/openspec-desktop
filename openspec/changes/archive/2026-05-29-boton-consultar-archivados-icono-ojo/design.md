## Context

En el panel de cambios archivados, la acción de apertura usa botón textual “Consultar”. El resto de la UI ya utiliza acciones compactas icon-only con tooltip. Se quiere alinear esta acción al mismo patrón, idealmente con icono ojo para representar vista/consulta.

## Goals / Non-Goals

**Goals:**
- Reemplazar botón textual por botón icon-only (ojo) en cambios archivados.
- Preservar funcionalidad exacta de abrir modal de consulta.
- Mantener accesibilidad con `aria-label` y tooltip.

**Non-Goals:**
- Rediseñar tarjeta completa de archivados.
- Cambiar contenido/flujo del modal de consulta.
- Alterar datos de cambios archivados.

## Decisions

1. Icono de acción.
- Decisión: usar icono de ojo (`Eye`) como semántica de consultar/ver.
- Alternativa descartada: lupa; menos precisa para “ver detalle ya existente”.

2. Patrón visual.
- Decisión: botón icon-only con estilos compartidos de acciones compactas.
- Alternativa descartada: botón mixto icono+texto; no cumple objetivo de compactación.

3. Accesibilidad.
- Decisión: conservar `aria-label` + `title` con texto “Consultar <cambio>”.
- Alternativa descartada: solo icono sin texto alternativo; baja accesibilidad.

## Risks / Trade-offs

- [Riesgo] Menor descubribilidad para usuarios nuevos -> Mitigación: tooltip claro y consistente en hover.
- [Riesgo] Inconsistencia de tamaño/icono con otros botones -> Mitigación: reutilizar clase `icon-only` existente.
- [Riesgo] Click target pequeño -> Mitigación: mantener dimensiones mínimas de botón ya usadas en UI.
