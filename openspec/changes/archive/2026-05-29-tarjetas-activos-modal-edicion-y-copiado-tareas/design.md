## Context

Los cambios activos hoy usan tarjeta y navegación distinta a archivados, con barra lineal de progreso. El usuario quiere unificar patrón de tarjeta+modal, con edición opcional dentro del modal, y acciones operativas rápidas sobre tareas (toggle/copia).

## Goals / Non-Goals

**Goals:**
- Igualar visual de tarjetas de activos a archivados.
- Abrir modal de documentos desde icono ver en activos.
- Soportar preview/editar con toggle en modal activo.
- Cambiar progreso por círculo porcentual en esquina superior derecha de tarjeta activa.
- Mejorar usabilidad markdown: headers distintivos y checkboxes interactivos.
- Copiar nombre de tarea desde tarjeta activa.

**Non-Goals:**
- Cambios de estructura OpenSpec en disco.
- Nuevos endpoints backend complejos.
- Rediseño de panel de archivados.

## Decisions

1. Reusar modal de archivados como base para activos.
- Decisión: derivar modal común con flag editable para activos.
- Alternativa descartada: crear modal separado desde cero.

2. Progreso circular por tarjeta activa.
- Decisión: porcentaje por cambio en badge circular top-right.
- Alternativa descartada: mantener barra; no cumple requerimiento.

3. Toggle preview/editar en modal.
- Decisión: botón icono que cambia estado y conserva documento cargado.
- Alternativa descartada: pestañas múltiples; más complejidad.

4. Markdown interactivo.
- Decisión: estilo especial para headers y toggles de tareas persistidos con write_file.
- Alternativa descartada: solo editor texto sin interacción en preview.

## Risks / Trade-offs

- [Riesgo] Modal activo puede crecer en complejidad -> Mitigación: mantener toolbar mínima (ver/editar, guardar/cancelar).
- [Riesgo] Toggle de checkbox puede romper formato no estándar -> Mitigación: operar solo en patrón exacto `- [ ]`/`- [x]`.
- [Riesgo] Densidad visual en tarjeta activa -> Mitigación: badge compacto y truncado de textos.
