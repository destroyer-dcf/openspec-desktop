## ADDED Requirements

### Requirement: Confirmación por modal para borrado
La aplicación SHALL solicitar confirmación mediante modal antes de eliminar propuestas seleccionadas.

#### Scenario: Confirmar borrado
- **WHEN** la persona usuaria pulsa eliminar seleccionadas con elementos seleccionados
- **THEN** la app abre modal de confirmación y solo elimina tras confirmación explícita

#### Scenario: Cancelar borrado
- **WHEN** la persona usuaria cancela o cierra el modal
- **THEN** la app no elimina propuestas y mantiene la selección

### Requirement: Color semántico en hover para acciones masivas
La aplicación SHALL aplicar color semántico únicamente en hover para botones de archivar y borrar seleccionadas.

#### Scenario: Hover en archivar seleccionadas
- **WHEN** el puntero pasa por botón de archivar habilitado
- **THEN** el botón muestra estado hover verde

#### Scenario: Hover en eliminar seleccionadas
- **WHEN** el puntero pasa por botón de eliminar habilitado
- **THEN** el botón muestra estado hover rojo
