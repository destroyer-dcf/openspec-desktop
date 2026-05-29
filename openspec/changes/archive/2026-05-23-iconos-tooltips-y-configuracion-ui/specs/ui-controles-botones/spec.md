## ADDED Requirements

### Requirement: Botones con icono consistente
La aplicación SHALL renderizar iconos en todos los botones interactivos visibles de la UI con estilo homogéneo.

#### Scenario: Botón principal con icono
- **WHEN** el usuario visualiza una acción de botón en cualquier panel
- **THEN** el botón muestra icono y texto alineados de forma consistente

### Requirement: Tooltips descriptivos en botones
La aplicación SHALL mostrar tooltip descriptivo en botones interactivos para aclarar su acción.

#### Scenario: Hover en botón
- **WHEN** el usuario posiciona cursor sobre un botón con acción
- **THEN** se muestra tooltip con descripción clara de la acción

#### Scenario: Tooltip en controles compactos
- **WHEN** el botón es icon-only o de espacio reducido
- **THEN** el tooltip sigue disponible y describe la acción sin ambigüedad
