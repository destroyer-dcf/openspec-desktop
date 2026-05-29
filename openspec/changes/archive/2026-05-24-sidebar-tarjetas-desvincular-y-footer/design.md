## Context

La app ya maneja múltiples proyectos y persistencia de lista abierta, pero el sidebar está orientado a botones simples. La evolución a tarjetas incrementa escaneabilidad y permite acciones de gestión por ítem (desvincular) sin navegar a otra vista.

## Goals / Non-Goals

**Goals:**
- Renderizar proyectos como tarjetas con jerarquía tipográfica (nombre/path).
- Añadir botón `X` por tarjeta para desvincular carpeta de la app.
- Mantener selección azul clara.
- Reubicar configuración en footer del sidebar.
- Garantizar responsive en desktop y mobile.

**Non-Goals:**
- Borrar carpetas del sistema de archivos (solo desvincular de la app).
- Cambiar estructura interna de proyectos OpenSpec.

## Decisions

- Desvincular = eliminar entrada de `project_paths` persistidos y actualizar índice activo.
- Tarjeta activa mantiene color azul actual para consistencia visual con selección existente.
- Path se trunca con ellipsis si no cabe, pero accesible vía `title`.
- Footer fijo del sidebar para acción de configuración, separada del listado desplazable.

## Risks / Trade-offs

- [Riesgo] Desvincular proyecto activo puede dejar estado inconsistente → Mitigación: recalcular proyecto activo automáticamente.
- [Trade-off] Más controles por tarjeta aumenta densidad → Mitigación: iconografía compacta y jerarquía visual limpia.

## Migration Plan

1. Ajustar estructura visual del sidebar a tarjetas.
2. Añadir acción de desvincular con actualización de estado persistido.
3. Mover configuración al footer y conectar evento existente.
4. Validar responsive y estados de selección.

## Open Questions

- ¿Se requiere confirmación modal al desvincular o basta acción directa?
