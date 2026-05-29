## Context

Actualmente el modal de cambios activos puede abrirse con `canModify=false` en cambios completados, pero la UI sigue mostrando controles de edición/guardado en algunos flujos. Esto introduce incoherencia entre reglas de negocio y affordance visual.

## Goals / Non-Goals

**Goals:**
- Alinear la UI con la regla de no edición en cambios completos.
- Desactivar explícitamente editar/guardar cuando `canModify=false`.
- Mantener feedback claro del porqué la acción no está disponible.

**Non-Goals:**
- No cambiar la política de cuándo un cambio se considera completo.
- No rediseñar todo el editor markdown.
- No alterar permisos de cambios no completados.

## Decisions

- Usar `canModify` como fuente única de verdad para habilitar/deshabilitar editar y guardar.
- Mantener botones visibles pero disabled para evitar saltos de layout y comunicar capacidad bloqueada.
- Añadir guardas en handlers de guardado para evitar acciones aunque exista bypass visual.

## Risks / Trade-offs

- [Riesgo] Usuarios no entienden por qué está desactivado.
  Mitigación: tooltip/mensaje corto indicando "Cambio completo, solo lectura".

- [Riesgo] Estados inconsistentes entre botones y atajos.
  Mitigación: bloquear también en handlers lógicos, no solo en vista.

- [Riesgo] Regresión en edición normal.
  Mitigación: pruebas manuales separadas para cambio completo vs incompleto.
