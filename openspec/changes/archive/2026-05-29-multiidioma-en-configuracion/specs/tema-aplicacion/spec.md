## MODIFIED Requirements

### Requirement: Persistencia de preferencia de tema
La aplicación SHALL guardar y restaurar preferencias globales de UI incluyendo tema, densidad tipográfica e idioma seleccionado.

#### Scenario: Restauración al reiniciar
- **WHEN** el usuario selecciona tema/idioma, cierra y reabre la app
- **THEN** la app inicia con el mismo tema y el mismo idioma previamente seleccionados
