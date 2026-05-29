## ADDED Requirements

### Requirement: Acción de consulta archivada con icono ojo
La app SHALL mostrar en cada tarjeta de cambio archivado una acción icon-only con icono de ojo para abrir la consulta.

#### Scenario: Render de acción en tarjeta archivada
- **WHEN** se renderiza una tarjeta de cambio archivado
- **THEN** la acción de consulta se muestra como botón icon-only con icono de ojo

### Requirement: Equivalencia funcional de apertura
La app SHALL mantener la misma apertura de modal de consulta al pulsar el botón icon-only de ojo.

#### Scenario: Click en icono ojo
- **WHEN** la persona usuaria pulsa el botón icono ojo en una tarjeta archivada
- **THEN** la app abre el modal de consulta del cambio correspondiente
