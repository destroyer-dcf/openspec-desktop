## MODIFIED Requirements

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
