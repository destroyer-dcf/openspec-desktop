## Context

El flujo de propuestas ya existe, pero guarda fuera del árbol `openspec`. La decisión del proyecto es que todo artefacto funcional de gestión OpenSpec viva dentro de `openspec`, por lo que esta iteración corrige únicamente la ubicación de persistencia.

## Goals / Non-Goals

**Goals:**
- Persistir propuestas activas en `openspec/propose/actives`.
- Persistir propuestas archivadas en `openspec/propose/archived`.
- Mantener formato markdown actual con header y comportamiento UI existente.

**Non-Goals:**
- Rediseñar modal o panel de propuestas.
- Cambiar formato del archivo proposal markdown.
- Migración masiva automática de carpetas antiguas fuera de `openspec` (opcional/manual en esta fase).

## Decisions

- Resolver la ruta base desde `openspec_path` ya disponible en el estado de proyecto.
- Crear automáticamente `openspec/propose/actives` y `openspec/propose/archived` si no existen.
- Mantener comandos y contratos frontend/backend, cambiando solo el path base.

## Risks / Trade-offs

- [Riesgo] Propuestas existentes en `opencode/propose` no aparecerán tras el cambio → Mitigación: documentar migración manual o futura tarea de migración automática.
- [Trade-off] Cambio pequeño pero transversal en comandos FS → Mitigación: validar flujos de alta/edición/archivado con checks existentes.

## Migration Plan

1. Cambiar resolución de rutas en comandos de propuestas.
2. Verificar creación automática de carpetas bajo `openspec/propose`.
3. Validar listado, guardado y archivado múltiple.
4. Confirmar que UI refleja propuestas desde la nueva ruta.

## Open Questions

- ¿Se quiere añadir en próxima iteración una rutina de migración de `opencode/propose` a `openspec/propose`?
