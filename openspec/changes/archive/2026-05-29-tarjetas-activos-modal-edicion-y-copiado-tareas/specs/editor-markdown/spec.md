## MODIFIED Requirements

### Requirement: Visualización de documentos Markdown
La app SHALL renderizar cualquier artifact Markdown del proyecto (proposal, specs, design, tasks) como HTML formateado al seleccionarlo, y manejar rutas no-fichero sin error de sistema; además SHALL resaltar encabezados markdown con estilo distintivo frente al texto normal.

#### Scenario: Apertura de un artifact
- **WHEN** el usuario selecciona un artifact desde el dashboard o modal activo
- **THEN** la app muestra su contenido renderizado como Markdown con formato (encabezados, listas, código, tablas)

#### Scenario: Artifact vacío o inexistente
- **WHEN** el usuario intenta abrir un artifact que no tiene fichero en disco
- **THEN** la app muestra un mensaje indicando que el documento aún no ha sido creado

#### Scenario: Artifact apunta a directorio
- **WHEN** el artifact resuelve a una carpeta en lugar de fichero
- **THEN** la app muestra mensaje de recurso no editable y no lanza error de lectura de fichero

### Requirement: Edición de documentos Markdown
La app SHALL permitir editar el contenido de cualquier artifact Markdown directamente desde la interfaz o modal activo.

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
- **WHEN** el usuario selecciona otro artifact desde la vista de detalle del cambio o modal activo
- **THEN** la app muestra el nuevo artifact sin recargar el estado del proyecto

### Requirement: Marcado de tareas desde vista markdown
La app SHALL permitir marcar y desmarcar checkboxes de tareas (`- [ ]` / `- [x]`) desde la visualización markdown y persistir el cambio.

#### Scenario: Toggle de checkbox en preview
- **WHEN** la persona usuaria marca/desmarca una tarea desde la vista markdown
- **THEN** la app actualiza el fichero markdown en disco y refresca el estado de progreso asociado
