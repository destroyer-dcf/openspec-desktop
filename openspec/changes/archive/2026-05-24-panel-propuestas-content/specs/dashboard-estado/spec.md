## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en un layout de dos columnas: columna izquierda con la lista de cambios activos (incluyendo iconos de artifacts y barras de progreso) y columna derecha con el resumen global del proyecto y el indicador circular de progreso.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** la columna izquierda lista cada cambio con su nombre, los iconos de estado de sus artifacts y su barra de progreso de tareas

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

#### Scenario: Panel de propuestas bajo cambios activos
- **WHEN** se renderiza la columna de contenido principal del dashboard
- **THEN** debajo de “Cambios activos” se muestra el panel “Propuestas” con grid y acciones de añadir, modificar y selección para archivado múltiple
