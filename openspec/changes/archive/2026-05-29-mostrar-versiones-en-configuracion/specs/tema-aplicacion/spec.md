## MODIFIED Requirements

### Requirement: Tokens visuales estilo GitHub
La aplicación SHALL aplicar tokens semánticos de color al nuevo panel de versiones en configuración para asegurar legibilidad y coherencia en temas `light` y `dark`.

#### Scenario: Panel de versiones en tema dark
- **WHEN** el tema activo es `dark`
- **THEN** el panel de versiones usa fondos, bordes y texto con contraste adecuado dentro del modal

#### Scenario: Panel de versiones en tema light
- **WHEN** el tema activo es `light`
- **THEN** el panel de versiones usa la paleta clara con contraste legible y consistente con el resto del modal
