## MODIFIED Requirements

### Requirement: Estado de artifacts por cambio
La app SHALL reflejar estado de solo lectura para cambios completos al abrir su modal de documentos, desactivando acciones de edición/guardado coherentes con la completitud.

#### Scenario: Cambio completo
- **WHEN** se abre un cambio con tareas completas igual al total
- **THEN** el modal permite consulta de documentos pero desactiva acciones de edición y guardado

#### Scenario: Cambio incompleto
- **WHEN** se abre un cambio con tareas pendientes
- **THEN** el modal mantiene disponibles acciones de edición/guardado según flujo normal
