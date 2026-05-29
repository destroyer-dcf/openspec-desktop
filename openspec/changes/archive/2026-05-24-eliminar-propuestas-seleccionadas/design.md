## Context

El panel de propuestas ya soporta selección múltiple y archivado en lote. Falta la operación simétrica de eliminación, útil para limpieza de propuestas descartadas.

## Goals / Non-Goals

**Goals:**
- Exponer acción de eliminar seleccionadas con estilo icon-only y tooltip.
- Permitir borrar propuestas seleccionadas de forma masiva en backend.
- Mantener flujo consistente: selección -> acción -> refresco de lista.

**Non-Goals:**
- Recuperación/papelera de propuestas eliminadas en esta iteración.
- Rediseño de filtros o modal de edición.

## Decisions

- Reusar patrón de acción masiva existente (como archivar seleccionadas).
- Aplicar eliminación según dataset visible por filtro de estado (actives/archived).
- Mostrar confirmación previa mínima (prompt/modal ligera) para evitar borrado accidental.

## Risks / Trade-offs

- [Riesgo] Borrado accidental irreversible → Mitigación: confirmación explícita antes de ejecutar.
- [Trade-off] Más acciones en barra compacta pueden cargar visualmente → Mitigación: iconos claros + tooltips.

## Migration Plan

1. Añadir comando backend para eliminar propuestas por rutas.
2. Añadir botón icon-only “eliminar seleccionadas” en el panel.
3. Conectar confirmación, ejecución y refresco.
4. Validar operación en activas y archivadas.

## Open Questions

- ¿Se quiere en futuro papelera lógica en vez de borrado físico?
