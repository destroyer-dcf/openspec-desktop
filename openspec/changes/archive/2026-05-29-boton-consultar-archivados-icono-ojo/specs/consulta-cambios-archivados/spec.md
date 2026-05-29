## MODIFIED Requirements

### Requirement: Consulta de cambios archivados en modal
La aplicación SHALL permitir consultar un cambio archivado mediante un modal que liste sus documentos y muestre su contenido, iniciando la acción desde un botón icon-only de consulta en la tarjeta archivada.

#### Scenario: Apertura del modal desde tarjeta archivada
- **WHEN** la persona usuaria pulsa el botón de consulta (icono ojo) de un cambio archivado
- **THEN** la app abre un modal asociado a ese cambio con un listado de documentos disponibles

#### Scenario: Visualización de documento archivado
- **WHEN** la persona usuaria selecciona un documento del listado dentro del modal
- **THEN** la app muestra su contenido en el panel de consulta del modal
