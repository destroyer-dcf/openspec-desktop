## ADDED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar un dashboard con el estado global del proyecto: cambios activos, cambios archivados y specs existentes.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** el dashboard lista cada cambio activo con su nombre y el estado de sus artifacts (pendiente, listo, bloqueado)

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

### Requirement: Estado de artifacts por cambio
La app SHALL mostrar, para cada cambio activo, qué artifacts existen (proposal, specs, design, tasks) y cuáles están pendientes.

#### Scenario: Cambio con todos los artifacts
- **WHEN** se selecciona un cambio que tiene proposal, design, specs y tasks
- **THEN** la app muestra los cuatro artifacts como presentes y accesibles

#### Scenario: Cambio incompleto
- **WHEN** se selecciona un cambio que solo tiene proposal
- **THEN** la app indica visualmente qué artifacts faltan (design, specs, tasks)

### Requirement: Progreso de tareas
La app SHALL mostrar el progreso de tareas de un cambio activo cuando el fichero `tasks.md` está presente.

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems de lista Markdown (`- [ ]` / `- [x]`)
- **THEN** la app muestra el número de tareas completadas sobre el total (ej. 3/7)

#### Scenario: tasks.md sin ítems de lista
- **WHEN** el cambio tiene `tasks.md` sin ítems de lista
- **THEN** la app muestra el tasks.md como presente pero sin indicador de progreso numérico
