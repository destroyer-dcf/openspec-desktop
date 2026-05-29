## ADDED Requirements

### Requirement: Eliminación múltiple de propuestas seleccionadas
La aplicación SHALL permitir eliminar en lote propuestas seleccionadas desde el panel de propuestas mediante una acción masiva icon-only con tooltip.

#### Scenario: Eliminar propuestas seleccionadas
- **WHEN** la persona usuaria selecciona una o más propuestas y confirma la acción de eliminar
- **THEN** la app elimina los archivos markdown correspondientes y refresca el grid

#### Scenario: Sin selección
- **WHEN** no hay propuestas seleccionadas
- **THEN** la acción de eliminar seleccionadas aparece deshabilitada

### Requirement: Coherencia visual de acción masiva
La aplicación SHALL mostrar el botón de eliminar seleccionadas con el mismo estilo de acciones masivas existentes (icon-only + tooltip).

#### Scenario: Render de barra de acciones
- **WHEN** se muestra la barra de acciones del panel de propuestas
- **THEN** el botón de eliminar seleccionadas respeta patrón visual, tamaño y comportamiento de tooltip
