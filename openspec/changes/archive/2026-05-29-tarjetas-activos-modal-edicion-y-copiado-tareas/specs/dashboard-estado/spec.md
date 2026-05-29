## MODIFIED Requirements

### Requirement: Estado de artifacts por cambio
La app SHALL mostrar, para cada cambio activo, el estado de cada artifact con un icono visual diferenciado (✓ completado / ○ pendiente / ⊘ bloqueado), manteniendo proposal/design/specs y añadiendo tareas con el mismo patrón visual.

#### Scenario: Cambio con todos los artifacts
- **WHEN** se selecciona un cambio que tiene proposal, design, specs y tasks
- **THEN** los cuatro artifacts se muestran con icono ✓ y son accesibles

#### Scenario: Cambio incompleto
- **WHEN** se selecciona un cambio que solo tiene proposal
- **THEN** proposal muestra icono ✓ y los artifacts faltantes (design, specs, tasks) muestran icono ○

### Requirement: Progreso de tareas
La app SHALL mostrar en cada tarjeta de cambio activo el progreso en porcentaje dentro de un círculo ubicado arriba a la derecha, sustituyendo la barra lineal, y SHALL incluir una acción para copiar el nombre de la tarea/cambio para uso externo.

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems `- [ ]` / `- [x]`
- **THEN** la app muestra porcentaje de progreso en círculo en la tarjeta y permite copiar el nombre desde acción dedicada

#### Scenario: tasks.md sin ítems de lista o ausente
- **WHEN** el cambio no tiene `tasks.md` o el fichero no contiene ítems de lista
- **THEN** la app muestra progreso `0%` en círculo y mantiene disponible la acción de copia de nombre
