## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar un dashboard con el estado global del proyecto: cambios activos, cambios archivados y specs existentes, con layout estable sin solape entre sidebar y contenido.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** el dashboard lista cada cambio activo con su nombre y el estado de sus artifacts (pendiente, listo, bloqueado)

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

#### Scenario: Layout con sidebar y panel principal
- **WHEN** el usuario navega en dashboard o lista de artifacts con contenido extenso
- **THEN** el panel principal no se solapa con sidebar y mantiene separación visual consistente
