# configuracion-sidebar-estilo-ligero Specification

## Purpose
TBD - created from delta change ajustar-boton-configuracion-sidebar. Update Purpose after archive.

## Requirements
### Requirement: Acción de configuración con estilo ligero en sidebar
La app SHALL mostrar la acción de configuración en el footer del sidebar con estilo ligero: sin borde ni fondo persistente en estado normal y con realce visual solo en hover/focus.

#### Scenario: Estado normal
- **WHEN** la persona usuaria visualiza el footer del sidebar sin interactuar con configuración
- **THEN** el control de configuración se presenta sin borde visible ni caja sólida permanente

#### Scenario: Estado interactivo
- **WHEN** la persona usuaria pasa el cursor o enfoca con teclado la acción de configuración
- **THEN** el control muestra realce visual de hover/focus manteniendo accesibilidad y legibilidad
