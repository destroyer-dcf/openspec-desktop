## MODIFIED Requirements

### Requirement: Configuración de tamaño de texto
La aplicación SHALL permitir cambiar preferencias globales desde modal de configuración incluyendo tamaño de texto (`compact`, `normal`) e idioma de interfaz (`en`, `fr`, `de`, `pt`).

#### Scenario: Selección de idioma
- **WHEN** el usuario abre configuración y selecciona un idioma soportado
- **THEN** la app aplica ese idioma a la interfaz activa y conserva el resto de preferencias visuales

#### Scenario: Selección de tamaño compact
- **WHEN** el usuario selecciona `compact`
- **THEN** textos de UI se renderizan con escala menor y mayor densidad

#### Scenario: Selección de tamaño normal
- **WHEN** el usuario selecciona `normal`
- **THEN** textos de UI se renderizan con escala base por defecto
