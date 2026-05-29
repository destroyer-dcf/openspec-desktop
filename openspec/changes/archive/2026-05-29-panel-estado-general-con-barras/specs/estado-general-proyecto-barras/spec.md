## ADDED Requirements

### Requirement: Panel de estado general con barras
La app SHALL mostrar un panel de estado general del proyecto con visualización principal en barras y métricas agregadas de cambios y tareas.

#### Scenario: Proyecto con cambios y tareas
- **WHEN** el proyecto activo tiene cambios y `tasks.md` procesables
- **THEN** el panel muestra barras con proporciones y contadores para estado global del proyecto

#### Scenario: Proyecto sin datos
- **WHEN** no hay cambios o no existen tareas válidas
- **THEN** el panel muestra métricas en cero y barras vacías sin error
