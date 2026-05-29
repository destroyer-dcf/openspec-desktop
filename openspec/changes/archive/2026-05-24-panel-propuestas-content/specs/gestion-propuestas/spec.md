## ADDED Requirements

### Requirement: Panel de propuestas en grid
La aplicación SHALL mostrar un panel “Propuestas” en `content`, ubicado debajo de “Cambios activos”, con visualización en grid de propuestas activas.

#### Scenario: Render de propuestas activas
- **WHEN** existen archivos de propuesta en `opencode/propose/actives`
- **THEN** la app muestra una tarjeta por propuesta con fecha de creación, nombre y botón “Modificar”

#### Scenario: Sin propuestas activas
- **WHEN** no existen propuestas en `opencode/propose/actives`
- **THEN** la app muestra estado vacío del panel y mantiene disponible el botón para añadir propuesta

### Requirement: Creación y edición de propuesta
La aplicación SHALL permitir crear y modificar propuestas mediante un modal llamado “Propuesta” con nombre, tipo (`Feature`/`Bug`) y contenido markdown.

#### Scenario: Crear propuesta nueva
- **WHEN** la persona usuaria pulsa “Añadir propuesta”, completa nombre, tipo y markdown, y pulsa guardar
- **THEN** la app crea un archivo markdown en `opencode/propose/actives` con header que incluya tipo y nombre

#### Scenario: Modificar propuesta existente
- **WHEN** la persona usuaria abre “Modificar”, edita contenido y pulsa guardar
- **THEN** la app actualiza el archivo markdown existente conservando metadatos requeridos

#### Scenario: Descartar cambios en modal
- **WHEN** la persona usuaria pulsa descartar en el modal
- **THEN** la app cierra o revierte cambios no guardados sin modificar archivos

### Requirement: Archivado múltiple de propuestas
La aplicación SHALL permitir seleccionar múltiples propuestas activas y archivarlas en lote moviendo sus archivos a `opencode/propose/archived`.

#### Scenario: Archivar varias propuestas
- **WHEN** la persona usuaria selecciona varias propuestas del grid y ejecuta la acción de archivar
- **THEN** la app mueve los archivos correspondientes desde `opencode/propose/actives` hacia `opencode/propose/archived` y refresca el panel
