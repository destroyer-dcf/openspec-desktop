## Context

Ya existe gestión de propuestas, pero las acciones son verbosas y no hay filtros combinados para navegar rápido entre tipos y archivadas/no archivadas. Además, falta una vía directa para copiar contenido markdown a herramientas de IA.

## Goals / Non-Goals

**Goals:**
- Incluir dos combos compactos en horizontal para filtros de tipo y estado.
- Convertir acciones de panel/tarjeta a botones icon-only con tooltip.
- Añadir botón de copiar markdown por propuesta.
- Mantener coherencia con selección múltiple y archivado.

**Non-Goals:**
- Rediseñar modelo de persistencia de propuestas.
- Cambiar formato base de markdown en esta iteración.

## Decisions

- Mantener frontmatter mínimo (`name`, `type`, `createdAt`) para filtrar y renderizar metadatos de forma robusta.
- Usar `navigator.clipboard.writeText` para copiar markdown completo de propuesta.
- Filtros aplicados en cliente sobre dataset cargado (active/archived) para respuesta inmediata.
- Botones icon-only con tooltip nativo (`title`) para consistencia y bajo coste de implementación.

## Risks / Trade-offs

- [Riesgo] Tooltips nativos tienen personalización limitada → Mitigación: estandarizar labels claros y consistentes.
- [Riesgo] Clipboard puede fallar por permisos/contexto → Mitigación: feedback de error en UI.
- [Trade-off] Mantener frontmatter añade cabecera visible en archivo → Mitigación: mejora trazabilidad y filtrado estable.

## Migration Plan

1. Añadir estado de filtros y lógica de combinación en panel propuestas.
2. Sustituir botones de texto por icon-only con tooltips.
3. Añadir acción de copiar markdown y feedback.
4. Validar filtros + selección múltiple + archivado.

## Open Questions

- ¿Se desea feedback toast global o mensaje inline en tarjeta para “copiado”?
