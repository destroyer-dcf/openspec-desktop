## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en composición de pipeline visual (Propuestas, Cambios activos, Cambios archivados) y SHALL renderizar en el área `content` un panel superior de descripción con estado general en barras que refleje métricas agregadas del proyecto.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** el panel superior muestra barras y contadores actualizados con el estado general del proyecto

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra estados vacíos claros y el bloque de barras en cero sin errores
