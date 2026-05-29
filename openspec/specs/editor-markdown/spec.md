# editor-markdown Specification

## Purpose
TBD - created by archiving change visor-proyecto. Update Purpose after archive.
## Requirements
### Requirement: Visualización de documentos Markdown
La app SHALL renderizar cualquier artifact Markdown del proyecto (proposal, specs, design, tasks) como HTML formateado al seleccionarlo, y manejar rutas no-fichero sin error de sistema, mostrando headers markdown con color del texto principal y negrita para diferenciar estructura del texto normal.

#### Scenario: Apertura de un artifact
- **WHEN** el usuario selecciona un artifact desde el dashboard
- **THEN** la app muestra su contenido renderizado como Markdown con formato (encabezados, listas, código, tablas)

#### Scenario: Artifact vacío o inexistente
- **WHEN** el usuario intenta abrir un artifact que no tiene fichero en disco
- **THEN** la app muestra un mensaje indicando que el documento aún no ha sido creado

#### Scenario: Artifact apunta a directorio
- **WHEN** el artifact resuelve a una carpeta en lugar de fichero
- **THEN** la app muestra mensaje de recurso no editable y no lanza error de lectura de fichero

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

### Requirement: Creación guiada de spec.md por capability
El editor/visor SHALL permitir crear un nuevo documento de especificación solicitando nombre de capability y generando la ruta estándar `specs/<capability>/spec.md`.

#### Scenario: Nombre capability válido
- **WHEN** el usuario introduce un nombre válido para capability
- **THEN** la app crea la carpeta capability si no existe
- **AND** genera `spec.md` con plantilla inicial de requisito

#### Scenario: Nombre capability inválido o vacío
- **WHEN** el usuario confirma creación con nombre vacío o inválido
- **THEN** la app muestra validación y no crea archivos
