## ADDED Requirements

### Requirement: Creación guiada de spec.md por capability
El editor/visor SHALL permitir crear un nuevo documento de especificación solicitando nombre de capability y generando la ruta estándar `specs/<capability>/spec.md`.

#### Scenario: Nombre capability válido
- **WHEN** el usuario introduce un nombre válido para capability
- **THEN** la app crea la carpeta capability si no existe
- **AND** genera `spec.md` con plantilla inicial de requisito

#### Scenario: Nombre capability inválido o vacío
- **WHEN** el usuario confirma creación con nombre vacío o inválido
- **THEN** la app muestra validación y no crea archivos
