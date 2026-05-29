# bloqueo-acciones-edicion-cambio-completo Specification

## Purpose
Disables edit/save actions for documents when a change is marked as complete, enforcing read-only mode across the application.

## Requirements

### Requirement: Acciones de edición desactivadas en cambios completos
La app SHALL desactivar acciones de edición y guardado de documentos markdown cuando el cambio esté en estado completo y no modificable.

#### Scenario: Cambio completo en modal activo
- **WHEN** el usuario abre un cambio activo marcado como completo
- **THEN** los controles de editar y guardar aparecen desactivados y no ejecutan acciones

#### Scenario: Intento de guardado con UI bloqueada
- **WHEN** se intenta disparar guardado en un cambio completo por interacción indirecta
- **THEN** la app bloquea la operación y mantiene el documento en solo lectura
