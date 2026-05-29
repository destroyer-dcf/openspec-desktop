## ADDED Requirements

### Requirement: Visualización de documentos Markdown
La app SHALL renderizar cualquier artifact Markdown del proyecto (proposal, specs, design, tasks) como HTML formateado al seleccionarlo.

#### Scenario: Apertura de un artifact
- **WHEN** el usuario selecciona un artifact desde el dashboard
- **THEN** la app muestra su contenido renderizado como Markdown con formato (encabezados, listas, código, tablas)

#### Scenario: Artifact vacío o inexistente
- **WHEN** el usuario intenta abrir un artifact que no tiene fichero en disco
- **THEN** la app muestra un mensaje indicando que el documento aún no ha sido creado

### Requirement: Edición de documentos Markdown
La app SHALL permitir editar el contenido de cualquier artifact Markdown directamente desde la interfaz.

#### Scenario: Edición y guardado
- **WHEN** el usuario edita el contenido de un artifact y confirma el guardado
- **THEN** la app escribe el nuevo contenido en el fichero correspondiente en disco

#### Scenario: Cancelar edición
- **WHEN** el usuario inicia la edición pero cancela sin guardar
- **THEN** el fichero en disco no se modifica y el contenido previo se restaura en la vista

#### Scenario: Preview en tiempo real durante edición
- **WHEN** el usuario está editando un documento
- **THEN** la app muestra una vista previa renderizada del Markdown actualizada conforme escribe

### Requirement: Navegación entre documentos
La app SHALL permitir navegar entre los distintos artifacts de un cambio sin perder el contexto del proyecto.

#### Scenario: Cambio de artifact
- **WHEN** el usuario selecciona otro artifact desde la vista de detalle del cambio
- **THEN** la app muestra el nuevo artifact sin recargar el estado del proyecto
