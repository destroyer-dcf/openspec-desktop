# estado-vacio-specs-con-crear Specification

## Purpose
TBD - created by syncing change evitar-error-specs-directorio-y-ofrecer-crear. Update Purpose after verification.
## Requirements
### Requirement: Estado vacío de especificaciones sin error técnico
La interfaz SHALL mostrar un estado vacío amigable cuando el artifact `specs` referencia un directorio sin ficheros `spec.md` en lugar de mostrar errores de sistema.

#### Scenario: Carpeta specs vacía
- **WHEN** el usuario abre `specs` y no existen ficheros `specs/*/spec.md`
- **THEN** la app muestra "No existen ficheros de especificaciones"
- **AND** no muestra mensajes de error técnico de tipo IO

### Requirement: Acción de creación desde estado vacío
La interfaz SHALL ofrecer una acción para crear un nuevo fichero de especificación cuando el estado de `specs` está vacío.

#### Scenario: Crear spec desde estado vacío
- **WHEN** el usuario pulsa "Crear especificación" desde el estado vacío
- **THEN** la app crea un nuevo `specs/<capability>/spec.md` con plantilla inicial
- **AND** abre o selecciona el nuevo documento creado
