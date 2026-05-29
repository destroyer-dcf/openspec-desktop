## MODIFIED Requirements

### Requirement: Barra de progreso por cambio
La app SHALL mostrar por cada cambio activo un indicador circular porcentual en la tarjeta (esquina superior derecha) en lugar de barra lineal, usando el cálculo de tareas completadas sobre total.

#### Scenario: Cambio con tareas parcialmente completadas
- **WHEN** un cambio activo tiene `tasks.md` con ítems `- [ ]` y `- [x]`
- **THEN** la app muestra el porcentaje en un círculo en la tarjeta del cambio

#### Scenario: Cambio sin tasks.md o sin ítems de lista
- **WHEN** un cambio activo no tiene `tasks.md` o el fichero no tiene ítems de lista
- **THEN** la app muestra `0%` en el círculo de la tarjeta
