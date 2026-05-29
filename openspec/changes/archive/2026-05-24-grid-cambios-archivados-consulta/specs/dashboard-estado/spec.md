## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en un layout de dos columnas: columna izquierda con la lista de cambios activos (incluyendo iconos de artifacts y barras de progreso) y columna derecha con el resumen global del proyecto y el indicador circular de progreso.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** la columna izquierda lista cada cambio con su nombre, los iconos de estado de sus artifacts y su barra de progreso de tareas

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados en formato grid, donde cada elemento incluye nombre del cambio, fecha de aplicación/archivo y botón "Consultar"

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores
