## ADDED Requirements

### Requirement: Integración del resumen global en panel de proyecto
La app SHALL mostrar el indicador circular de resumen global dentro del panel de descripción del proyecto, ubicado en la zona derecha del panel.

#### Scenario: Render del panel con resumen integrado
- **WHEN** se muestra el panel de descripción del proyecto en el dashboard
- **THEN** el rosco de resumen global aparece a la derecha dentro del mismo panel

### Requirement: Conservación de cálculo del resumen
La app SHALL mantener el mismo cálculo de progreso global del rosco tras su reubicación.

#### Scenario: Cambio de ubicación sin cambio de valor
- **WHEN** el rosco se renderiza en el panel de proyecto
- **THEN** el porcentaje y estado mostrado coinciden con el resumen global previo
