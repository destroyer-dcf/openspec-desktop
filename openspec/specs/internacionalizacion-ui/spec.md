# internacionalizacion-ui Specification

## Purpose
Multi-language UI support with automatic fallback mechanism for missing translations.

## Requirements
### Requirement: Internacionalización de interfaz
La app SHALL soportar traducción de textos de interfaz para inglés, francés, alemán y portugués mediante selector de idioma.

#### Scenario: Cambio de idioma en caliente
- **WHEN** el usuario selecciona un idioma distinto en configuración
- **THEN** los textos de UI se actualizan sin reiniciar la aplicación

#### Scenario: Idioma con clave faltante
- **WHEN** una clave no exista en el idioma activo
- **THEN** la app usa fallback al idioma por defecto sin mostrar error
