## ADDED Requirements

### Requirement: Columna única por panel del pipeline
La app SHALL renderizar el contenido interno de cada panel del pipeline (Propuestas, Cambios activos, Cambios archivados) en una sola columna vertical.

#### Scenario: Render del panel de propuestas
- **WHEN** se visualiza el panel de propuestas
- **THEN** sus elementos internos se muestran en una única columna

#### Scenario: Render del panel de archivados
- **WHEN** se visualiza el panel de cambios archivados
- **THEN** sus tarjetas se muestran en una única columna

### Requirement: Consistencia de flujo vertical
La app SHALL mantener orden secuencial de arriba a abajo dentro de cada panel sin saltos laterales de tarjetas.

#### Scenario: Listas con muchos elementos
- **WHEN** un panel contiene múltiples tarjetas
- **THEN** la navegación visual y de scroll sigue una secuencia vertical lineal
