## MODIFIED Requirements

### Requirement: Visualización de documentos Markdown
La app SHALL renderizar cualquier artifact Markdown del proyecto (proposal, specs, design, tasks) como HTML formateado al seleccionarlo, y manejar rutas no-fichero sin error de sistema.

#### Scenario: Apertura de un artifact
- **WHEN** el usuario selecciona un artifact desde el dashboard
- **THEN** la app muestra su contenido renderizado como Markdown con formato (encabezados, listas, código, tablas)

#### Scenario: Artifact vacío o inexistente
- **WHEN** el usuario intenta abrir un artifact que no tiene fichero en disco
- **THEN** la app muestra un mensaje indicando que el documento aún no ha sido creado

#### Scenario: Artifact apunta a directorio
- **WHEN** el artifact resuelve a una carpeta en lugar de fichero
- **THEN** la app muestra mensaje de recurso no editable y no lanza error de lectura de fichero
