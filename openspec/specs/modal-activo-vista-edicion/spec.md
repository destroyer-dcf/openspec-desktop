# modal-activo-vista-edicion Specification

## Purpose
Modal interface for viewing, editing, and managing active change documents. Extends consultation modal from archived changes with edit/preview toggle, document list navigation, and integration with markdown editor functionality.

## Requirements

### Requirement: Consulta de documentos de cambios activos en modal
La app SHALL abrir documentos de cambios activos en un modal de consulta equivalente al de archivados, iniciado desde un icono de ver en la tarjeta activa.

#### Scenario: Abrir documento activo desde icono ver
- **WHEN** la persona usuaria pulsa el icono de ver en una tarjeta de cambio activo
- **THEN** la app abre un modal con listado de documentos del cambio y vista previa del documento seleccionado

### Requirement: Toggle preview/edición en modal activo
La app SHALL permitir alternar entre modo preview y modo edición dentro del modal activo, incluyendo guardado y cancelación.

#### Scenario: Alternar a edición
- **WHEN** la persona usuaria activa el icono de editar en el modal activo
- **THEN** la app cambia a modo edición del documento actual y habilita guardar/cancelar

#### Scenario: Volver a preview
- **WHEN** la persona usuaria desactiva edición o cancela
- **THEN** la app vuelve a preview del documento sin cerrar el modal
