## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar un dashboard con el estado global del proyecto: cambios activos, cambios archivados y specs existentes, respetando el tema visual activo (`light` o `dark`).

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** el dashboard lista cada cambio activo con su nombre y el estado de sus artifacts (pendiente, listo, bloqueado)

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

#### Scenario: Dashboard en tema dark
- **WHEN** el tema activo es `dark`
- **THEN** el dashboard renderiza fondos, paneles, bordes y tipografía con tokens dark definidos por la guía visual

#### Scenario: Dashboard en tema light
- **WHEN** el tema activo es `light`
- **THEN** el dashboard renderiza fondos, paneles, bordes y tipografía con tokens light definidos por la guía visual
