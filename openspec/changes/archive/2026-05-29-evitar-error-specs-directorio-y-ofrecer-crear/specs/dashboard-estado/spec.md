## ADDED Requirements

### Requirement: Artifact specs mostrado como contenedor navegable
El dashboard SHALL tratar el artifact `specs` como contenedor de documentos cuando su ruta sea directorio.

#### Scenario: Specs con subcarpetas
- **WHEN** el artifact `specs` apunta a un directorio con uno o más `spec.md`
- **THEN** la app permite navegar y abrir esos documentos sin error

#### Scenario: Specs sin documentos
- **WHEN** el artifact `specs` apunta a un directorio sin `spec.md`
- **THEN** la app muestra estado vacío con opción de crear especificación
