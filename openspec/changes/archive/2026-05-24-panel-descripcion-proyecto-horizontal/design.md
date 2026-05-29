## Context

El dashboard actual muestra principalmente cambios activos y progreso, pero no expone en primer plano el contexto descriptivo del proyecto activo. Ese contexto ya existe en `config.yaml` bajo el bloque `contexto`, por lo que la mejora consiste en proyectarlo en UI sin duplicar fuente de verdad.

## Goals / Non-Goals

**Goals:**
- Mostrar un panel horizontal de descripción por proyecto activo encima de "Cambios activos".
- Mapear y renderizar campos de `contexto` de `config.yaml` con fallback cuando falten datos.
- Mantener layout estable sin solapes con sidebar y con adaptación responsive.

**Non-Goals:**
- Editar `contexto` desde el dashboard.
- Cambiar el formato o estructura de `config.yaml`.
- Rediseñar completo el dashboard o la navegación lateral.

## Decisions

- Fuente única de datos: leer `contexto` desde `config.yaml` del proyecto activo en el mismo flujo de carga de estado del proyecto.
Alternativas: duplicar datos en estado local persistente. Se descarta para evitar desincronización.

- Presentación compacta y horizontal: usar un bloque tipo "summary panel" con items en línea (wrapping en pantallas pequeñas) para priorizar lectura rápida.
Alternativas: tarjeta vertical larga. Se descarta por consumir alto vertical y empujar demasiado contenido.

- Fallback explícito por campo ausente: mostrar etiqueta con valor "No definido" cuando falte clave en `contexto`.
Alternativas: ocultar campos vacíos. Se descarta porque rompe consistencia visual y dificulta detectar configuración incompleta.

## Risks / Trade-offs

- [Riesgo] `config.yaml` incompleto o inconsistente entre proyectos → Mitigación: validación defensiva y fallback visual por campo.
- [Riesgo] Saturación visual si hay demasiados campos de contexto → Mitigación: priorizar subset estable de campos y truncar texto largo con tooltip.
- [Trade-off] Más información en cabecera reduce espacio inicial para cambios activos → Mitigación: panel compacto con padding reducido y wrapping responsive.

## Migration Plan

1. Añadir parsing de `contexto` al modelo de estado del proyecto activo.
2. Incorporar componente del panel de descripción en la parte superior de `content`.
3. Ajustar estilos/layout para evitar solape con sidebar y asegurar reflow correcto.
4. Verificar manualmente con proyectos con/sin `config.yaml` y con campos faltantes.
5. Si hay regresión visual, rollback aislado removiendo solo el nuevo panel y manteniendo resto del dashboard.

## Open Questions

- ¿Qué campos exactos de `contexto` deben tener prioridad en el panel cuando existan muchos?
- ¿El truncado de valores largos debe incluir copia al portapapeles además de tooltip?
