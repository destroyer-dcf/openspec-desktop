## Context

Las tarjetas de cambios activos y archivados muestran metadatos estructurales (nombre, fecha, progreso/artifacts) pero no exponen el objetivo funcional del cambio. El texto `Why` de `proposal.md` ya existe y es la fuente natural para resumen contextual.

## Goals / Non-Goals

**Goals:**
- Mostrar resumen `Why` bajo el título en tarjetas activas y archivadas.
- Reutilizar fuente única: sección `## Why` de `proposal.md`.
- Mantener layout compacto con truncado a 2 líneas y fallback amigable.

**Non-Goals:**
- Editar `proposal.md` desde tarjetas.
- Añadir parsing markdown completo del proposal en frontend.
- Cambiar flujo de archivado/lectura de documentos.

## Decisions

1. **Extracción backend de `Why`**
   - Parsear `proposal.md` al cargar cambios y exponer campo `why_summary` en modelo `Change`.
   - Reduce lógica duplicada en frontend y asegura coherencia activos/archivados.

2. **Fallback seguro**
   - Si no existe `proposal.md` o no hay bloque `Why`, usar texto neutral (ej. "Sin resumen").
   - Evita errores visibles y mantiene UX consistente.

3. **Render de 2 líneas en tarjetas**
   - Estilo de texto secundario bajo título con clamp de 2 líneas.
   - Balance entre contexto y densidad visual.

## Risks / Trade-offs

- **[Riesgo]** Parsing frágil por variaciones de formato markdown → **Mitigación**: heurística simple por encabezado `## Why` y corte en siguiente `##`.
- **[Riesgo]** Texto largo rompiendo layout → **Mitigación**: clamp + ellipsis.
- **[Trade-off]** Campo adicional en `Change` incrementa payload → **Mitigación**: solo un resumen corto.
