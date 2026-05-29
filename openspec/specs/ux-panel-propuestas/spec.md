# ux-panel-propuestas Specification

## Purpose
TBD - created by archiving change filtros-iconos-y-copiar-propuestas. Update Purpose after archive.
## Requirements
### Requirement: Filtros compactos del panel de propuestas
La aplicación SHALL mostrar dos combos pequeños en horizontal junto a las acciones del panel de propuestas para filtrar por tipo y por estado (activas/archivadas).

#### Scenario: Filtrado por tipo
- **WHEN** la persona usuaria selecciona `Feature` o `Bug` en el combo de tipo
- **THEN** el grid muestra solo propuestas que coinciden con ese tipo

#### Scenario: Filtrado por estado
- **WHEN** la persona usuaria selecciona `Activas` o `Archivadas` en el combo de estado
- **THEN** el grid muestra propuestas del estado seleccionado

### Requirement: Botones icon-only con tooltip
La aplicación SHALL usar botones de solo icono con tooltip para añadir propuesta, archivar seleccionadas y modificar propuesta.

#### Scenario: Visualización de acciones
- **WHEN** se renderiza el panel y las tarjetas de propuestas
- **THEN** las acciones se muestran como icon-only y cada botón expone tooltip descriptivo

### Requirement: Copiar markdown al portapapeles
La aplicación SHALL permitir copiar el contenido markdown completo de una propuesta mediante un botón icon-only con tooltip.

#### Scenario: Copia de propuesta
- **WHEN** la persona usuaria pulsa el botón de copiar de una propuesta
- **THEN** la app copia al portapapeles el markdown completo de esa propuesta y muestra confirmación o error

