## ADDED Requirements

### Requirement: Pipeline visual de estado del proyecto
La app SHALL mostrar el estado del proyecto en un pipeline visual con paneles equivalentes de Propuestas, Cambios activos y Cambios archivados en ese orden.

#### Scenario: Orden de paneles del pipeline
- **WHEN** se renderiza la vista de estado del proyecto
- **THEN** la app muestra Propuestas a la izquierda, Cambios activos en el centro y Cambios archivados a la derecha

### Requirement: Equivalencia visual de etapas
La app SHALL presentar el panel de Propuestas con estructura visual equivalente a los paneles de cambios (cabecera, contenedor, contenido principal).

#### Scenario: Apariencia de paneles
- **WHEN** se visualiza el dashboard
- **THEN** el panel de Propuestas mantiene misma jerarquía visual y coherencia de estilo respecto a Cambios activos y Cambios archivados
