## MODIFIED Requirements

### Requirement: Acciones contextuales por tarjeta con iconos y tooltip
La UI SHALL mostrar acciones de archivar y borrar dentro de cada tarjeta de propuesta usando botones con icono y tooltip. La acción de editar SHALL mostrarse únicamente para propuestas no archivadas y MUST ocultarse para propuestas archivadas.

#### Scenario: Descubrimiento de acción por tooltip
- **WHEN** la persona usuaria pasa el cursor sobre los iconos de acción
- **THEN** el sistema muestra tooltip de la acción correspondiente

#### Scenario: Edición disponible en propuesta activa
- **WHEN** una propuesta está activa (no archivada)
- **THEN** la tarjeta muestra la acción de editar

#### Scenario: Edición oculta en propuesta archivada
- **WHEN** una propuesta está archivada
- **THEN** la tarjeta no muestra la acción de editar
