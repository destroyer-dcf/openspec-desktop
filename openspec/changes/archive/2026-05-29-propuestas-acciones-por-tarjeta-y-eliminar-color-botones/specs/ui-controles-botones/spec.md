## MODIFIED Requirements

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
