## MODIFIED Requirements

### Requirement: Persistencia de preferencia de tema
La aplicación SHALL guardar y restaurar preferencias globales de UI incluyendo tema, densidad tipográfica e idioma seleccionado, y SHALL incluir el tamaño de ventana principal como parte del estado restaurado al inicio.

#### Scenario: Restauración al reiniciar
- **WHEN** el usuario selecciona tema/idioma, ajusta tamaño de ventana, cierra y reabre la app
- **THEN** la app inicia con el mismo tema, el mismo idioma y el mismo tamaño de ventana previamente seleccionados
