# configuracion-densidad-y-color Specification

## Purpose
TBD - created by archiving change iconos-tooltips-y-configuracion-ui. Update Purpose after archive.
## Requirements
### Requirement: Configuración de tamaño de texto
La aplicación SHALL permitir cambiar preferencias globales desde modal de configuración incluyendo tamaño de texto (`compact`, `normal`) e idioma de interfaz (`en`, `fr`, `de`, `pt`).

#### Scenario: Selección de idioma
- **WHEN** el usuario abre configuración y selecciona un idioma soportado
- **THEN** la app aplica ese idioma a la interfaz activa y conserva el resto de preferencias visuales

#### Scenario: Selección de tamaño compact
- **WHEN** el usuario selecciona `compact`
- **THEN** textos de UI se renderizan con escala menor y mayor densidad

#### Scenario: Selección de tamaño normal
- **WHEN** el usuario selecciona `normal`
- **THEN** textos de UI se renderizan con escala base por defecto

### Requirement: Persistencia de preferencias visuales
La aplicación SHALL persistir `fontScale` y `buttonColor` y restaurarlos al iniciar.

#### Scenario: Reinicio con preferencias guardadas
- **WHEN** el usuario cierra y reabre app tras cambiar densidad
- **THEN** la UI restaura tamaño de texto previamente seleccionado

### Requirement: Sección de colores por panel en modal de configuración
El modal de configuración SHALL mostrar en su parte superior un panel de versiones (OpenSpec CLI y app) antes de las secciones de preferencias visuales y SHALL mantener los controles agrupados por panel con etiquetas de estado/tipo.

#### Scenario: Visualizar configuración completa
- **WHEN** la persona usuaria abre el modal de configuración
- **THEN** primero ve el panel de versiones y debajo los controles de configuración existentes agrupados por panel

### Requirement: Persistencia de configuración de colores
El sistema SHALL persistir y restaurar la configuración de colores por panel/tipo entre sesiones.

#### Scenario: Reabrir aplicación con configuración previa
- **WHEN** la aplicación inicia con configuración de colores ya guardada
- **THEN** el modal y las tarjetas reflejan esos valores guardados

