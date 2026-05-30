# persistencia-dimension-ventana Specification

## Purpose
TBD

## Requirements

### Requirement: Persistencia de tamaño de ventana principal
La aplicación SHALL guardar el ancho y alto de la ventana principal al cierre normal y SHALL restaurar ese tamaño en el siguiente arranque.

#### Scenario: Restauración de tamaño tras reinicio
- **WHEN** el usuario redimensiona la ventana, cierra la aplicación y vuelve a abrirla
- **THEN** la ventana principal inicia con el mismo ancho y alto guardados

#### Scenario: Fallback por dimensiones inválidas
- **WHEN** las dimensiones persistidas no existen o son inválidas
- **THEN** la aplicación inicia con dimensiones por defecto compatibles con los mínimos definidos

### Requirement: Respeto de límites mínimos
La aplicación MUST validar que el tamaño restaurado cumple con las dimensiones mínimas de la ventana antes de aplicarlo.

#### Scenario: Dimensiones por debajo del mínimo
- **WHEN** el estado persistido contiene dimensiones menores al mínimo permitido
- **THEN** la aplicación ajusta el tamaño de arranque al mínimo configurado
