## MODIFIED Requirements

### Requirement: Creación y edición de propuesta
La aplicación SHALL permitir crear y modificar propuestas mediante un modal llamado “Propuesta” con nombre, tipo (`Feature`/`Bug`) y contenido markdown, conservando metadatos en frontmatter para soportar filtrado estable.

#### Scenario: Crear propuesta nueva
- **WHEN** la persona usuaria pulsa “Añadir propuesta”, completa nombre, tipo y markdown, y pulsa guardar
- **THEN** la app crea un archivo markdown en `openspec/propose/actives` con metadatos de nombre, tipo y fecha

#### Scenario: Modificar propuesta existente
- **WHEN** la persona usuaria abre “Modificar”, edita contenido y pulsa guardar
- **THEN** la app actualiza el archivo markdown existente conservando metadatos requeridos

#### Scenario: Descartar cambios en modal
- **WHEN** la persona usuaria pulsa descartar en el modal
- **THEN** la app cierra o revierte cambios no guardados sin modificar archivos
