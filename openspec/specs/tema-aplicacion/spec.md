# tema-aplicacion Specification

## Purpose
TBD - created by archiving change tema-dark-light-modal-configuracion. Update Purpose after archive.
## Requirements
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

### Requirement: Persistencia de preferencia de tema
La aplicación SHALL guardar y restaurar preferencias globales de UI incluyendo tema, densidad tipográfica e idioma seleccionado.

#### Scenario: Restauración al reiniciar
- **WHEN** el usuario selecciona tema/idioma, cierra y reabre la app
- **THEN** la app inicia con el mismo tema y el mismo idioma previamente seleccionados

### Requirement: Tokens visuales estilo GitHub
La aplicación SHALL usar tokens semánticos de color para ambos temas siguiendo las guías de `STYLEGUIDELINES.md` y evitar colores hardcodeados por componente.

#### Scenario: Tema dark con contraste GitHub-like
- **WHEN** el tema activo es `dark`
- **THEN** fondos, paneles, bordes, texto y acento usan paleta oscura neutral con acento azul tipo GitHub

#### Scenario: Tema light con contraste GitHub-like
- **WHEN** el tema activo es `light`
- **THEN** fondos, paneles, bordes, texto y acento usan paleta clara tipo GitHub con contraste legible

#### Scenario: Panel de versiones en tema dark
- **WHEN** el tema activo es `dark`
- **THEN** el panel de versiones usa fondos, bordes y texto con contraste adecuado dentro del modal

#### Scenario: Panel de versiones en tema light
- **WHEN** el tema activo es `light`
- **THEN** el panel de versiones usa la paleta clara con contraste legible y consistente con el resto del modal

