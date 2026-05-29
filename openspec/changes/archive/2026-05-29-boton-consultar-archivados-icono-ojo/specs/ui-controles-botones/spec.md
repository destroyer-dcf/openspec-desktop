## MODIFIED Requirements

### Requirement: Tooltips descriptivos en botones
La aplicación SHALL mostrar tooltip descriptivo en botones interactivos para aclarar su acción, incluyendo la acción icon-only de consultar cambios archivados.

#### Scenario: Hover en botón
- **WHEN** el usuario posiciona cursor sobre un botón con acción
- **THEN** se muestra tooltip con descripción clara de la acción

#### Scenario: Tooltip en controles compactos
- **WHEN** el botón es icon-only o de espacio reducido
- **THEN** el tooltip sigue disponible y describe la acción sin ambigüedad

#### Scenario: Tooltip en consultar archivado
- **WHEN** el usuario hace hover sobre el botón icono ojo en cambios archivados
- **THEN** se muestra tooltip de consulta del cambio archivado correspondiente
