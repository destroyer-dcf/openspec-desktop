## ADDED Requirements

### Requirement: Acciones contextuales por tarjeta con iconos y tooltip
La UI SHALL mostrar acciones de archivar y borrar dentro de cada tarjeta de propuesta usando botones con icono y tooltip.

#### Scenario: Descubrimiento de acción por tooltip
- **WHEN** la persona usuaria pasa el cursor sobre los iconos de acción
- **THEN** el sistema muestra tooltip de la acción correspondiente

## REMOVED Requirements

### Requirement: Barra global de acciones de propuestas
**Reason**: La interacción se simplifica con acciones locales por tarjeta.
**Migration**: Acciones globales se eliminan y se ejecutan directamente en cada tarjeta.
