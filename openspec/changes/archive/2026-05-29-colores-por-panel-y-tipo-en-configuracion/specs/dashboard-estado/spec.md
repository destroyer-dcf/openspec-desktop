## ADDED Requirements

### Requirement: Tarjetas activas con color por completitud
El panel de cambios activos SHALL aplicar color configurable distinto para tarjetas completadas y pendientes.

#### Scenario: Tarjeta completada
- **WHEN** un cambio activo tiene tareas completas igual al total
- **THEN** la tarjeta usa el color configurado para estado completado

#### Scenario: Tarjeta pendiente
- **WHEN** un cambio activo tiene tareas completas menor que el total
- **THEN** la tarjeta usa el color configurado para estado pendiente

### Requirement: Fallback neutral en activos
El panel de cambios activos SHALL mantener estilo neutral cuando el color configurado sea `sin color`.

#### Scenario: Sin color en activos
- **WHEN** la configuración del estado correspondiente en activos está en `sin color`
- **THEN** la tarjeta no aplica variante de color
