## MODIFIED Requirements

### Requirement: Barra de progreso por cambio
La app SHALL mostrar una barra de progreso lineal para cada cambio activo que tenga `tasks.md` con ítems de lista y SHALL representar su porcentaje por cambio mediante texto destacado en la tarjeta activa, reemplazando el indicador circular en ese contexto.

#### Scenario: Cambio con tareas parcialmente completadas
- **WHEN** un cambio activo tiene `tasks.md` con ítems `- [ ]` y `- [x]`
- **THEN** la app muestra una barra de progreso, el texto `X/Y tareas` y el porcentaje textual destacado en la tarjeta

#### Scenario: Cambio sin tasks.md o sin ítems de lista
- **WHEN** un cambio activo no tiene `tasks.md` o el fichero no tiene ítems de lista
- **THEN** la app muestra `Sin tareas` en lugar de barra de progreso y el porcentaje textual del cambio se mantiene en `0%` sin mostrar rosco circular
