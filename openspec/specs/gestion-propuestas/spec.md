# gestion-propuestas Specification

## Purpose
TBD - created by archiving change eliminar-propuestas-seleccionadas. Update Purpose after archive.
## Requirements
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

### Requirement: Archivado múltiple de propuestas
La aplicación SHALL permitir seleccionar múltiples propuestas activas y archivarlas en lote moviendo sus archivos a `openspec/propose/archived`.

#### Scenario: Archivar varias propuestas
- **WHEN** la persona usuaria selecciona varias propuestas del grid y ejecuta la acción de archivar
- **THEN** la app mueve los archivos correspondientes desde `openspec/propose/actives` hacia `openspec/propose/archived` y refresca el panel

### Requirement: Tarjetas de propuestas con color por tipo
El sistema SHALL aplicar color configurable a tarjetas de propuestas según su tipo (`feature` o `bug`).

#### Scenario: Propuesta feature
- **WHEN** una propuesta tiene tipo `feature`
- **THEN** la tarjeta usa el color configurado para `feature`

#### Scenario: Propuesta bug
- **WHEN** una propuesta tiene tipo `bug`
- **THEN** la tarjeta usa el color configurado para `bug`

### Requirement: Fallback neutral en propuestas
El sistema SHALL renderizar propuestas con estilo neutral cuando su selector esté en `sin color`.

#### Scenario: Sin color en tipo feature
- **WHEN** el color de `feature` está en `sin color`
- **THEN** las propuestas feature se muestran con estilo neutral

