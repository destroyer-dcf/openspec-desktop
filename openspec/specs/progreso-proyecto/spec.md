# progreso-proyecto Specification

## Purpose
TBD - created by archiving change ui-dashboard-progreso. Update Purpose after archive.
## Requirements
### Requirement: Indicador de progreso global
La app SHALL mostrar el estado global del proyecto con una visualización de barras y métricas agregadas en lugar del indicador circular, manteniendo el cálculo de porcentaje sobre tareas completadas respecto al total.

#### Scenario: Proyecto con tareas activas
- **WHEN** el proyecto activo tiene cambios con `tasks.md` que contienen ítems de lista
- **THEN** el panel global muestra porcentaje completado y barras con distribución de tareas completadas/pendientes

#### Scenario: Proyecto sin tareas
- **WHEN** el proyecto activo no tiene ningún cambio con `tasks.md` o todos los `tasks.md` están vacíos
- **THEN** el panel global muestra `0%` y barras en estado vacío sin error

#### Scenario: Todas las tareas completadas
- **WHEN** todos los ítems de todos los `tasks.md` están marcados como `- [x]`
- **THEN** el panel global muestra `100%` y barra de completado al máximo

### Requirement: Barra de progreso por cambio
La app SHALL mostrar por cada cambio activo un indicador circular porcentual en la tarjeta (esquina superior derecha) en lugar de barra lineal, usando el cálculo de tareas completadas sobre total.

#### Scenario: Cambio con tareas parcialmente completadas
- **WHEN** un cambio activo tiene `tasks.md` con ítems `- [ ]` y `- [x]`
- **THEN** la app muestra el porcentaje en un círculo en la tarjeta del cambio

#### Scenario: Cambio sin tasks.md o sin ítems de lista
- **WHEN** un cambio activo no tiene `tasks.md` o el fichero no tiene ítems de lista
- **THEN** la app muestra `0%` en el círculo de la tarjeta

### Requirement: Actualización en tiempo real del progreso
Los indicadores de progreso SHALL reflejar automáticamente los cambios producidos en los `tasks.md` por herramientas externas.

#### Scenario: Tarea marcada como completada externamente
- **WHEN** un ítem de `tasks.md` cambia de `- [ ]` a `- [x]` desde fuera de la app
- **THEN** la barra de progreso del cambio y el indicador global se actualizan en menos de 2 segundos

