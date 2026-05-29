## MODIFIED Requirements

### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en una composición de pipeline visual para estado operativo: panel de Propuestas a la izquierda, panel de Cambios activos en zona central y panel de Cambios archivados a la derecha. Además, en el área `content` SHALL renderizar primero el panel horizontal de descripción del proyecto activo (basado en `config.yaml.contexto`) y debajo los paneles del pipeline sin solape, garantizando que el contenido interno de cada panel se organice en una sola columna vertical.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** la zona central lista cada cambio con su nombre, los iconos de estado de sus artifacts y su barra de progreso de tareas en flujo vertical de una columna

#### Scenario: Dashboard con cambios archivados
- **WHEN** el proyecto tiene cambios en `changes/archive/`
- **THEN** el panel derecho muestra cambios archivados con su nombre y fecha de archivo en una sola columna

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra estados vacíos claros en los paneles correspondientes, sin errores

#### Scenario: Orden de secciones en content
- **WHEN** se renderiza el contenido principal del dashboard
- **THEN** la descripción del proyecto aparece encima del pipeline y ambas áreas conservan separación visual sin solape
