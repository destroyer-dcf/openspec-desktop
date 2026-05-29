## Context

El dashboard ya distingue cambios activos y archivados, pero la experiencia para archivados es limitada: no prioriza densidad visual ni ofrece una consulta guiada de documentos. Además, al tratarse de historial, la interacción debe ser explícitamente de solo lectura.

## Goals / Non-Goals

**Goals:**
- Renderizar cambios archivados en grid con tarjeta por cambio.
- Mostrar fecha de aplicación/archivo en cada tarjeta.
- Permitir abrir modal de consulta con listado de documentos y vista de contenido sin edición.
- Evitar cualquier ruta de edición para documentos archivados.

**Non-Goals:**
- Reabrir o desarchivar cambios desde este flujo.
- Editar documentos archivados.
- Cambiar estructura física del directorio `changes/archive`.

## Decisions

- Grid responsive para archivados: usar `repeat(auto-fill, minmax(...))` para escalar densidad en desktop y mantener legibilidad en anchos pequeños.
Alternativa considerada: lista vertical. Se descarta por menor capacidad de escaneo.

- Fecha mostrada por tarjeta: derivar desde metadato disponible de archivo (p. ej. timestamp de archivado en metadata o fallback de mtime).
Alternativa: no mostrar fecha cuando falte. Se descarta; se usará fallback "Fecha no disponible".

- Modal de consulta reutilizando visor markdown existente en modo readonly.
Alternativa: crear visor nuevo. Se descarta para minimizar superficie y regresiones.

- En modo archivado, ocultar/deshabilitar controles de edición (guardar, editar, atajos de escritura).
Alternativa: permitir editar y bloquear persistencia. Se descarta por UX confusa.

## Risks / Trade-offs

- [Riesgo] Fecha inconsistente entre sistemas de archivos → Mitigación: normalizar formato y fallback explícito.
- [Riesgo] Muchos archivados pueden afectar rendimiento de render → Mitigación: carga incremental/paginación futura si crece volumen.
- [Trade-off] Modal readonly reduce flexibilidad de edición rápida → Mitigación: coherencia de historial inmutable.

## Migration Plan

1. Actualizar modelo de archivados para incluir `appliedAt`/`archivedAt` normalizado.
2. Reemplazar lista por grid en sección de archivados.
3. Incorporar botón "Consultar" por tarjeta y wiring de apertura modal.
4. Implementar modal con listado de documentos + visor readonly.
5. Validar manualmente: apertura, navegación de docs y ausencia total de edición.

## Open Questions

- ¿El label de fecha en UI debe llamarse "Fecha de aplicación" o "Fecha de archivo" cuando la fuente sea timestamp de archivado?
- ¿El modal debe recordar último documento abierto entre aperturas?
