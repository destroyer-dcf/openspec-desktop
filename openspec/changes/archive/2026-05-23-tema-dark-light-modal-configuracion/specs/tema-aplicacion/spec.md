## ADDED Requirements

### Requirement: Selección de tema en modal de configuración
La aplicación SHALL permitir seleccionar el tema visual desde el modal de configuración con dos opciones obligatorias: `light` y `dark`.

#### Scenario: Selección de tema dark
- **WHEN** el usuario abre configuración y selecciona `dark`
- **THEN** la app aplica tema oscuro en toda la interfaz activa

#### Scenario: Selección de tema light
- **WHEN** el usuario abre configuración y selecciona `light`
- **THEN** la app aplica tema claro en toda la interfaz activa

### Requirement: Persistencia de preferencia de tema
La aplicación SHALL guardar la preferencia de tema seleccionada y restaurarla automáticamente al iniciar.

#### Scenario: Restauración al reiniciar
- **WHEN** el usuario selecciona un tema, cierra la app y la vuelve a abrir
- **THEN** la app inicia con el mismo tema previamente seleccionado

### Requirement: Tokens visuales estilo GitHub
La aplicación SHALL usar tokens semánticos de color para ambos temas siguiendo las guías de `STYLEGUIDELINES.md` y evitar colores hardcodeados por componente.

#### Scenario: Tema dark con contraste GitHub-like
- **WHEN** el tema activo es `dark`
- **THEN** fondos, paneles, bordes, texto y acento usan paleta oscura neutral con acento azul tipo GitHub

#### Scenario: Tema light con contraste GitHub-like
- **WHEN** el tema activo es `light`
- **THEN** fondos, paneles, bordes, texto y acento usan paleta clara tipo GitHub con contraste legible
