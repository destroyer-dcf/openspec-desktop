# configuracion-colores-por-panel Specification

## Purpose
TBD - created by syncing change colores-por-panel-y-tipo-en-configuracion. Update Purpose after archive.

## Requirements

### Requirement: Configuración de color por panel y tipo
El sistema SHALL permitir definir desde el modal de configuración una preferencia de color por panel y tipo para tarjetas de cambios activos, propuestas y cambios archivados.

#### Scenario: Configurar colores por panel/tipo
- **WHEN** la persona usuaria abre el modal de configuración y selecciona colores para cada panel/tipo
- **THEN** el sistema guarda la configuración segmentada por panel y tipo
- **AND** los cambios quedan disponibles para renderizado inmediato de tarjetas

### Requirement: Opción sin color por cada selector
El sistema SHALL ofrecer la opción `sin color` en cada selector de color por panel/tipo.

#### Scenario: Mantener estilo neutral
- **WHEN** la persona usuaria selecciona `sin color` para un panel/tipo
- **THEN** el sistema renderiza las tarjetas correspondientes con estilo neutral actual
