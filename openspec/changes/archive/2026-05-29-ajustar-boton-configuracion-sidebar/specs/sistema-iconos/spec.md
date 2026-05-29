## MODIFIED Requirements

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
