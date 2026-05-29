# ui-controles-botones Specification

## Purpose
TBD - created by archiving change iconos-tooltips-y-configuracion-ui. Update Purpose after archive.
## Requirements
### Requirement: Colores de botón basados en tema
La interfaz SHALL usar colores de botón y hover definidos por el tema activo, sin variantes seleccionables por usuario.

#### Scenario: Tema claro
- **WHEN** la aplicación está en tema claro
- **THEN** los botones usan color base y hover del tema claro por defecto

#### Scenario: Tema oscuro
- **WHEN** la aplicación está en tema oscuro
- **THEN** los botones usan color base y hover del tema oscuro por defecto

#### Scenario: Sin selector de color
- **WHEN** la persona usuaria abre configuración
- **THEN** no existe selector para cambiar color de botones

### Requirement: Botones con icono consistente
La aplicación SHALL renderizar iconos en todos los botones interactivos visibles de la UI con estilo homogéneo.

#### Scenario: Botón principal con icono
- **WHEN** el usuario visualiza una acción de botón en cualquier panel
- **THEN** el botón muestra icono y texto alineados de forma consistente

### Requirement: Tooltips descriptivos en botones
La aplicación SHALL mostrar tooltip descriptivo en botones interactivos para aclarar su acción.

#### Scenario: Hover en botón
- **WHEN** el usuario posiciona cursor sobre un botón con acción
- **THEN** se muestra tooltip con descripción clara de la acción

#### Scenario: Tooltip en controles compactos
- **WHEN** el botón es icon-only o de espacio reducido
- **THEN** el tooltip sigue disponible y describe la acción sin ambigüedad

#### Scenario: Tooltip en consultar archivado
- **WHEN** el usuario hace hover sobre el botón icono ojo en cambios archivados
- **THEN** se muestra tooltip de consulta del cambio archivado correspondiente

