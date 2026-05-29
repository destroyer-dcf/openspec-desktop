## ADDED Requirements

### Requirement: Archivado por tarjeta de propuesta
El sistema SHALL permitir archivar una propuesta directamente desde su tarjeta individual.

#### Scenario: Archivar desde tarjeta
- **WHEN** la persona usuaria pulsa el botón archivar en una tarjeta
- **THEN** el sistema archiva esa propuesta específica
- **AND** refresca la lista mostrada

### Requirement: Borrado por tarjeta de propuesta
El sistema SHALL permitir borrar una propuesta directamente desde su tarjeta individual, manteniendo confirmación previa.

#### Scenario: Borrar desde tarjeta
- **WHEN** la persona usuaria confirma borrado desde una tarjeta
- **THEN** el sistema elimina esa propuesta específica
- **AND** refresca la lista mostrada

## REMOVED Requirements

### Requirement: Selección múltiple para acciones de propuestas
**Reason**: Se reemplaza el flujo de lote por acciones contextuales por tarjeta.
**Migration**: Usar botones de archivar/borrar en cada tarjeta en lugar de checkboxes y acciones globales.
