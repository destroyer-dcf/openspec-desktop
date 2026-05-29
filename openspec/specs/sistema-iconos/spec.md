# sistema-iconos Specification

## Purpose
TBD - created by archiving change ui-dashboard-progreso. Update Purpose after archive.
## Requirements
### Requirement: Iconos en navegación y acciones
La app SHALL mostrar iconos visuales consistentes en todos los elementos de navegación y acciones de la interfaz, incluyendo icono de rueda dentada para el acceso a configuración del sidebar.

#### Scenario: Barra lateral con iconos
- **WHEN** la barra lateral muestra la lista de proyectos
- **THEN** cada ítem de proyecto muestra un icono de carpeta junto a su nombre

#### Scenario: Botón de añadir proyecto
- **WHEN** el usuario ve la barra lateral
- **THEN** el botón de añadir proyecto muestra un icono "+" reconocible

#### Scenario: Botón de configuración en sidebar
- **WHEN** el usuario visualiza la acción de configuración del footer del sidebar
- **THEN** la acción muestra un icono de rueda dentada coherente con la semántica de configuración

#### Scenario: Botones de acción en artifacts
- **WHEN** el usuario ve los artifacts de un cambio
- **THEN** cada artifact tiene iconos de acción visibles para "abrir" y "editar"

### Requirement: Iconos de estado de artifacts
La app SHALL mostrar un icono de estado diferenciado para cada artifact de un cambio: completado, pendiente o bloqueado.

#### Scenario: Artifact completado
- **WHEN** un artifact existe en disco y tiene contenido
- **THEN** se muestra un icono de verificación (✓) junto al nombre del artifact

#### Scenario: Artifact pendiente
- **WHEN** un artifact no existe en disco todavía
- **THEN** se muestra un icono de círculo vacío (○) junto al nombre del artifact

#### Scenario: Artifact bloqueado
- **WHEN** un artifact está marcado como bloqueado (sin estado previo)
- **THEN** se muestra un icono de bloqueo (⊘) junto al nombre del artifact

### Requirement: Iconos de secciones del dashboard
La app SHALL mostrar iconos en las cabeceras de las secciones principales del dashboard.

#### Scenario: Cabeceras de sección
- **WHEN** el dashboard muestra las secciones "Cambios activos", "Cambios archivados" y "Resumen global"
- **THEN** cada cabecera tiene un icono representativo y consistente con el contenido de la sección
