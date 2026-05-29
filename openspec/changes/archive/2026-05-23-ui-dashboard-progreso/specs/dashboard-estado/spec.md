## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en un layout de dos columnas: columna izquierda con la lista de cambios activos (incluyendo iconos de artifacts y barras de progreso) y columna derecha con el resumen global del proyecto y el indicador circular de progreso.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** la columna izquierda lista cada cambio con su nombre, los iconos de estado de sus artifacts y su barra de progreso de tareas

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

### Requirement: Estado de artifacts por cambio
La app SHALL mostrar, para cada cambio activo, el estado de cada artifact con un icono visual diferenciado (✓ completado / ○ pendiente / ⊘ bloqueado).

#### Scenario: Cambio con todos los artifacts
- **WHEN** se selecciona un cambio que tiene proposal, design, specs y tasks
- **THEN** los cuatro artifacts se muestran con icono ✓ y son accesibles

#### Scenario: Cambio incompleto
- **WHEN** se selecciona un cambio que solo tiene proposal
- **THEN** proposal muestra icono ✓ y los artifacts faltantes (design, specs, tasks) muestran icono ○

### Requirement: Progreso de tareas
La app SHALL mostrar el progreso de tareas de un cambio activo con una barra de progreso lineal y el texto "X/Y tareas".

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems `- [ ]` / `- [x]`
- **THEN** la app muestra una barra de progreso y el texto "X/Y tareas"

#### Scenario: tasks.md sin ítems de lista o ausente
- **WHEN** el cambio no tiene `tasks.md` o el fichero no contiene ítems de lista
- **THEN** la app muestra "Sin tareas" sin barra de progreso
