## ADDED Requirements

### Requirement: Contexto funcional visible en cambios activos
El dashboard SHALL incluir debajo del nombre del cambio activo una vista resumida del objetivo del cambio basado en `proposal.md/Why`.

#### Scenario: Texto largo de Why
- **WHEN** el bloque `Why` excede el espacio de tarjeta
- **THEN** la UI lo trunca visualmente a un máximo de dos líneas
