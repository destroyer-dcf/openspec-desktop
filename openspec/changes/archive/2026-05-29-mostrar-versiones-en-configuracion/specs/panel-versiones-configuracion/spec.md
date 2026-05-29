## ADDED Requirements

### Requirement: Panel de versiones en configuración
La app SHALL mostrar en la parte superior del modal de configuración un panel informativo con la versión de OpenSpec CLI y la versión de la aplicación de escritorio.

#### Scenario: Apertura de configuración con OpenSpec disponible
- **WHEN** el usuario abre el modal de configuración y OpenSpec CLI está disponible
- **THEN** el panel superior muestra ambas versiones con valores válidos

#### Scenario: OpenSpec CLI no disponible
- **WHEN** el usuario abre configuración y falla la obtención de versión de OpenSpec CLI
- **THEN** el panel muestra la versión de la app y un fallback legible para OpenSpec sin romper la UI
