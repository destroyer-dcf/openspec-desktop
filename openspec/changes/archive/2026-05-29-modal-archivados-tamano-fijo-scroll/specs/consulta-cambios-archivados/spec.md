## MODIFIED Requirements

### Requirement: Consulta de cambios archivados en modal
La aplicación SHALL permitir consultar un cambio archivado mediante un modal de tamaño estándar que liste sus documentos y muestre su contenido en un panel con scroll interno cuando sea necesario.

#### Scenario: Apertura del modal desde tarjeta archivada
- **WHEN** la persona usuaria pulsa el botón "Consultar" de un cambio archivado
- **THEN** la app abre un modal asociado a ese cambio con un listado de documentos disponibles y tamaño de ventana estable

#### Scenario: Visualización de documento archivado
- **WHEN** la persona usuaria selecciona un documento del listado dentro del modal
- **THEN** la app muestra su contenido en el panel de consulta del modal sin redimensionar la ventana por longitud del documento
