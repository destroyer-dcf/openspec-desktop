## ADDED Requirements

### Requirement: Consulta de cambios archivados en modal
La aplicación SHALL permitir consultar un cambio archivado mediante un modal que liste sus documentos y muestre su contenido.

#### Scenario: Apertura del modal desde tarjeta archivada
- **WHEN** la persona usuaria pulsa el botón "Consultar" de un cambio archivado
- **THEN** la app abre un modal asociado a ese cambio con un listado de documentos disponibles

#### Scenario: Visualización de documento archivado
- **WHEN** la persona usuaria selecciona un documento del listado dentro del modal
- **THEN** la app muestra su contenido en el panel de consulta del modal

### Requirement: Consulta en modo solo lectura
La aplicación SHALL bloquear cualquier edición de documentos archivados durante la consulta.

#### Scenario: Documento consultado en modal
- **WHEN** se muestra un documento archivado en el modal
- **THEN** los controles de edición están ocultos o deshabilitados y no se pueden guardar cambios
