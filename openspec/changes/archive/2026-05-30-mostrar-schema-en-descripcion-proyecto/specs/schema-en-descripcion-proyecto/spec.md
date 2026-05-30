## ADDED Requirements

### Requirement: Mostrar schema del proyecto en descripción
La aplicación SHALL mostrar en el panel de descripción del proyecto el valor de `schema` leído desde `openspec/config.yaml` del proyecto activo.

#### Scenario: Config con schema válido
- **WHEN** el proyecto activo contiene `openspec/config.yaml` con `schema`
- **THEN** el panel de descripción muestra una línea `Schema: <valor>` con el contenido real del archivo

#### Scenario: Config sin schema o no legible
- **WHEN** falta `config.yaml`, falta la clave `schema` o falla el parseo
- **THEN** el panel muestra `Schema: No disponible` sin romper la carga de la vista
