## MODIFIED Requirements

### Requirement: Selección de tema en modal de configuración
La aplicación SHALL permitir seleccionar preferencias visuales desde el modal de configuración incluyendo tema `light|dark`, tamaño de texto (`compact|normal`) y color de botones (`blue|green`).

#### Scenario: Selección de tema dark
- **WHEN** el usuario abre configuración y selecciona `dark`
- **THEN** la app aplica tema oscuro en toda la interfaz activa

#### Scenario: Selección de tema light
- **WHEN** el usuario abre configuración y selecciona `light`
- **THEN** la app aplica tema claro en toda la interfaz activa

#### Scenario: Ajuste de densidad tipográfica
- **WHEN** el usuario selecciona `compact` o `normal`
- **THEN** la app actualiza escala tipográfica global en caliente

#### Scenario: Ajuste de color de botones
- **WHEN** el usuario selecciona `blue` o `green`
- **THEN** la app actualiza color de botones principales en caliente
