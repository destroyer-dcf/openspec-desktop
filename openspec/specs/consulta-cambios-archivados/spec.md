# consulta-cambios-archivados Specification

## Purpose
TBD - created by archiving change grid-cambios-archivados-consulta. Update Purpose after archive.
## Requirements
### Requirement: Consulta de cambios archivados en modal
La aplicación SHALL permitir consultar un cambio archivado mediante un modal de tamaño estándar que liste sus documentos y muestre su contenido en un panel con scroll interno cuando sea necesario.

#### Scenario: Apertura del modal desde tarjeta archivada
- **WHEN** la persona usuaria pulsa el botón "Consultar" de un cambio archivado
- **THEN** la app abre un modal asociado a ese cambio con un listado de documentos disponibles y tamaño de ventana estable

#### Scenario: Visualización de documento archivado
- **WHEN** la persona usuaria selecciona un documento del listado dentro del modal
- **THEN** la app muestra su contenido en el panel de consulta del modal sin redimensionar la ventana por longitud del documento

### Requirement: Consulta en modo solo lectura
La aplicación SHALL bloquear cualquier edición de documentos archivados durante la consulta.

#### Scenario: Documento consultado en modal
- **WHEN** se muestra un documento archivado en el modal
- **THEN** los controles de edición están ocultos o deshabilitados y no se pueden guardar cambios

### Requirement: Contexto funcional visible en cambios archivados
La sección de cambios archivados SHALL mostrar el mismo resumen `Why` debajo del título de cada tarjeta.

#### Scenario: Archivado con resumen disponible
- **WHEN** el cambio archivado tiene resumen `Why`
- **THEN** la tarjeta archivada lo muestra con estilo secundario

#### Scenario: Archivado sin resumen disponible
- **WHEN** no existe resumen `Why` para el cambio archivado
- **THEN** la tarjeta muestra fallback y mantiene fecha + acción de consulta

