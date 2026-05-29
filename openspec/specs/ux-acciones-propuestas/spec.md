# ux-acciones-propuestas Specification

## Purpose
TBD - created by archiving change confirmacion-modal-y-hover-colores-acciones. Update Purpose after archive.
## Requirements
### Requirement: Acciones contextuales por tarjeta con iconos y tooltip
La UI SHALL mostrar acciones de archivar y borrar dentro de cada tarjeta de propuesta usando botones con icono y tooltip.

#### Scenario: Descubrimiento de acción por tooltip
- **WHEN** la persona usuaria pasa el cursor sobre los iconos de acción
- **THEN** el sistema muestra tooltip de la acción correspondiente

### Requirement: Confirmación por modal para borrado
La aplicación SHALL solicitar confirmación mediante modal antes de eliminar propuestas seleccionadas.

#### Scenario: Confirmar borrado
- **WHEN** la persona usuaria pulsa eliminar seleccionadas con elementos seleccionados
- **THEN** la app abre modal de confirmación y solo elimina tras confirmación explícita

#### Scenario: Cancelar borrado
- **WHEN** la persona usuaria cancela o cierra el modal
- **THEN** la app no elimina propuestas y mantiene la selección

### Requirement: Color semántico en hover para acciones por tarjeta
La aplicación SHALL aplicar color semántico únicamente en hover para botones de archivar y borrar seleccionadas.

#### Scenario: Hover en archivar tarjeta
- **WHEN** el puntero pasa por botón de archivar en tarjeta
- **THEN** el botón muestra estado hover verde

#### Scenario: Hover en eliminar tarjeta
- **WHEN** el puntero pasa por botón de eliminar en tarjeta
- **THEN** el botón muestra estado hover rojo

