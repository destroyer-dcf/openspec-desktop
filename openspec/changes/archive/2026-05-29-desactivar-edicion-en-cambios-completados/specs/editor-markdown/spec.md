## MODIFIED Requirements

### Requirement: Edición de documentos Markdown
La app SHALL permitir editar contenido markdown solo cuando el cambio sea modificable y SHALL desactivar editar/guardar cuando el cambio esté completo (solo lectura).

#### Scenario: Edición y guardado
- **WHEN** el usuario edita el contenido de un artifact en un cambio modificable y confirma guardado
- **THEN** la app escribe el nuevo contenido en disco

#### Scenario: Cambio completo en solo lectura
- **WHEN** el usuario abre un artifact de un cambio completo
- **THEN** la UI muestra controles de editar/guardar desactivados y no permite persistir cambios

#### Scenario: Cancelar edición
- **WHEN** el usuario inicia edición en cambio modificable y cancela sin guardar
- **THEN** el fichero en disco no se modifica y la vista restaura el contenido previo
