## ADDED Requirements

### Requirement: Tarjetas de propuestas con color por tipo
El sistema SHALL aplicar color configurable a tarjetas de propuestas según su tipo (`feature` o `bug`).

#### Scenario: Propuesta feature
- **WHEN** una propuesta tiene tipo `feature`
- **THEN** la tarjeta usa el color configurado para `feature`

#### Scenario: Propuesta bug
- **WHEN** una propuesta tiene tipo `bug`
- **THEN** la tarjeta usa el color configurado para `bug`

### Requirement: Fallback neutral en propuestas
El sistema SHALL renderizar propuestas con estilo neutral cuando su selector esté en `sin color`.

#### Scenario: Sin color en tipo feature
- **WHEN** el color de `feature` está en `sin color`
- **THEN** las propuestas feature se muestran con estilo neutral
