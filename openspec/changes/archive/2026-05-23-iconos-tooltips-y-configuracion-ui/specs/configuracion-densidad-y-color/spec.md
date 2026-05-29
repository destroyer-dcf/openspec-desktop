## ADDED Requirements

### Requirement: Configuración de tamaño de texto
La aplicación SHALL permitir cambiar tamaño de texto global desde modal de configuración, con al menos dos niveles: `compact` y `normal`.

#### Scenario: Selección de tamaño compact
- **WHEN** el usuario selecciona `compact`
- **THEN** textos de UI se renderizan con escala menor y mayor densidad

#### Scenario: Selección de tamaño normal
- **WHEN** el usuario selecciona `normal`
- **THEN** textos de UI se renderizan con escala base por defecto

### Requirement: Configuración de color de botones
La aplicación SHALL permitir seleccionar color de botones desde modal de configuración con opciones `blue` y `green`.

#### Scenario: Selección azul
- **WHEN** el usuario selecciona variante `blue`
- **THEN** botones principales usan token cromático azul

#### Scenario: Selección verde
- **WHEN** el usuario selecciona variante `green`
- **THEN** botones principales usan token cromático verde

### Requirement: Persistencia de preferencias visuales
La aplicación SHALL persistir `fontScale` y `buttonColor` y restaurarlos al iniciar.

#### Scenario: Reinicio con preferencias guardadas
- **WHEN** el usuario cierra y reabre app tras cambiar densidad y color
- **THEN** la UI restaura tamaño de texto y color de botón previamente seleccionados
