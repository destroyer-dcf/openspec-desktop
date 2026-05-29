## Context

El panel de propuestas ya tiene acciones masivas de archivar y borrar, pero usa confirmación nativa y no distingue semántica visual en hover de forma precisa. Esta iteración formaliza confirmación modal y feedback visual de intención.

## Goals / Non-Goals

**Goals:**
- Reemplazar confirmación nativa por modal de confirmación para borrado.
- Aplicar hover rojo al botón eliminar seleccionadas.
- Aplicar hover verde al botón archivar seleccionadas.
- Mantener icon-only + tooltip y estados disabled existentes.

**Non-Goals:**
- Cambiar lógica backend de borrado/archivado.
- Rediseñar layout general del panel.

## Decisions

- Reusar patrón modal ligero ya presente en app para mantener coherencia.
- Hover semántico solo cuando el botón está habilitado.
- Colores semánticos limitados a hover para minimizar ruido visual en estado normal.

## Risks / Trade-offs

- [Riesgo] Modal adicional añade un clic extra → Mitigación: solo para acción destructiva.
- [Trade-off] Color solo en hover reduce señal en reposo → Mitigación: tooltip explícito + iconografía.

## Migration Plan

1. Implementar modal de confirmación de borrado en dashboard.
2. Eliminar uso de `confirm()` nativo.
3. Ajustar CSS de hover para archivar y borrar.
4. Validar estados disabled + hover + confirmación.

## Open Questions

- ¿Se desea tecla rápida Enter/Escape para confirmar/cancelar modal de borrado?
