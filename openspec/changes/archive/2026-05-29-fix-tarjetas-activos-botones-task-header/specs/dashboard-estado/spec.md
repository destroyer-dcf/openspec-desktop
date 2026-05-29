## MODIFIED Requirements

### Requirement: Estado de artifacts por cambio
La app SHALL mostrar, para cada cambio activo, el estado de cada artifact con un icono visual diferenciado (✓ completado / ○ pendiente / ⊘ bloqueado), manteniendo tasks en formato icon-only sin texto adicional.

#### Scenario: Cambio con todos los artifacts
- **WHEN** se selecciona un cambio que tiene proposal, design, specs y tasks
- **THEN** los iconos de artifacts se muestran con su estado y tasks se representa solo con icono

#### Scenario: Cambio incompleto
- **WHEN** se selecciona un cambio que solo tiene proposal
- **THEN** proposal muestra icono ✓ y los artifacts faltantes (design, specs, tasks) muestran icono ○

### Requirement: Progreso de tareas
La app SHALL mostrar en cada tarjeta de cambio activo el progreso en porcentaje dentro de un círculo ubicado arriba a la derecha, y SHALL organizar sus acciones en una fila separada debajo de la fila de artifacts.

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems `- [ ]` / `- [x]`
- **THEN** la app muestra porcentaje en círculo y deja los botones de acción debajo de artifacts

#### Scenario: tasks.md sin ítems de lista o ausente
- **WHEN** el cambio no tiene `tasks.md` o el fichero no contiene ítems de lista
- **THEN** la app muestra progreso `0%` en círculo y mantiene la misma estructura de tarjeta
