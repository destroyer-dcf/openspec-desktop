## Context

La app ya gestiona cambios activos y archivados, pero no tiene una etapa previa de ideación formal para convertir ideas en cambios. Este diseño añade una capa de propuestas local, orientada a captura rápida y evolución controlada (editar, archivar) sin mezclarla todavía con `openspec/changes`.

## Goals / Non-Goals

**Goals:**
- Mostrar panel “Propuestas” bajo “Cambios activos” con vista grid.
- Crear y modificar propuestas mediante modal con tipo, nombre y contenido markdown.
- Persistir propuestas en `opencode/propose/actives` y permitir archivado múltiple hacia `opencode/propose/archived`.
- Mantener un formato markdown consistente con header para tipo y nombre.

**Non-Goals:**
- Convertir automáticamente una propuesta en cambio OpenSpec.
- Añadir colaboración multiusuario o sincronización remota.
- Implementar versionado de propuestas más allá del archivo markdown actual.

## Decisions

- Estructura de carpetas fija: `opencode/propose/actives` y `opencode/propose/archived`.
Alternativa: base de datos local. Se descarta por complejidad innecesaria para este alcance.

- Formato de archivo markdown con frontmatter mínimo:
  - `name`, `type` (`feature|bug`), `createdAt`
  - cuerpo markdown editable.
Alternativa: metadatos en nombre de archivo. Se descarta por fragilidad y peor extensibilidad.

- Grid con selección múltiple mediante checkboxes y acción “Archivar seleccionadas”.
Alternativa: archivar una a una desde tarjeta. Se mantiene también como posible acción futura, pero la principal en este cambio es batch.

- Modal reutiliza patrón visual existente de modales para consistencia de UX.

## Risks / Trade-offs

- [Riesgo] Conflictos de nombre de archivo para propuestas duplicadas → Mitigación: slug único y validación previa al guardar.
- [Riesgo] Markdown malformado en edición manual externa → Mitigación: parser tolerante y fallback de metadatos por defecto.
- [Trade-off] Gestión por archivos implica operaciones FS y refresco de estado → Mitigación: comandos Tauri dedicados y recarga puntual del panel.

## Migration Plan

1. Añadir comandos backend para listar/crear/actualizar/archivar propuestas.
2. Definir modelo `Proposal` compartido con metadatos necesarios.
3. Implementar panel grid y selección múltiple en dashboard.
4. Implementar modal “Propuesta” con guardado/descartar y edición markdown.
5. Validar flujo completo: crear, editar, archivar múltiples, refresco UI.

## Open Questions

- ¿El nombre visible y el slug de archivo deben desacoplarse completamente o mantenerse vinculados al crear?
- ¿Se requiere filtro rápido por tipo (`Feature`/`Bug`) en esta primera versión?
