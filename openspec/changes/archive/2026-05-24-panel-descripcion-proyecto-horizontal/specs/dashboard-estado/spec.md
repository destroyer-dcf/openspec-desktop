## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en un layout de dos columnas: columna izquierda con la lista de cambios activos (incluyendo iconos de artifacts y barras de progreso) y columna derecha con el resumen global del proyecto y el indicador circular de progreso. Además, en el área `content` SHALL renderizar primero un panel horizontal de descripción del proyecto activo (basado en `config.yaml.contexto`) y debajo la sección de cambios activos.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** la columna izquierda lista cada cambio con su nombre, los iconos de estado de sus artifacts y su barra de progreso de tareas

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el dashboard muestra una sección de cambios archivados con su nombre y fecha de archivo

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra un estado vacío claro, sin errores

#### Scenario: Orden de secciones en content
- **WHEN** se renderiza el contenido principal del dashboard
- **THEN** la descripción del proyecto aparece encima de "Cambios activos" y ambas secciones conservan separación visual sin solape
