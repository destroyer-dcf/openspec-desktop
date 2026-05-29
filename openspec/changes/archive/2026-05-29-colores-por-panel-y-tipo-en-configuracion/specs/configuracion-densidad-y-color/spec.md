## ADDED Requirements

### Requirement: Sección de colores por panel en modal de configuración
El modal de configuración SHALL mostrar una sección específica para colores por panel/tipo con controles separados para activos, propuestas y archivados.

#### Scenario: Visualizar controles por panel
- **WHEN** la persona usuaria abre el modal de configuración
- **THEN** el modal muestra controles agrupados por panel con etiquetas de estado/tipo

### Requirement: Persistencia de configuración de colores
El sistema SHALL persistir y restaurar la configuración de colores por panel/tipo entre sesiones.

#### Scenario: Reabrir aplicación con configuración previa
- **WHEN** la aplicación inicia con configuración de colores ya guardada
- **THEN** el modal y las tarjetas reflejan esos valores guardados
