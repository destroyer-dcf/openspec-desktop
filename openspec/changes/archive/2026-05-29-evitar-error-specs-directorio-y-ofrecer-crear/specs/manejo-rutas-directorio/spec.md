## MODIFIED Requirements

### Requirement: Detección de rutas de directorio frente a archivo
La aplicación SHALL detectar explícitamente si una ruta objetivo es directorio o archivo antes de intentar leer su contenido, y aplicar flujo diferenciado para cada caso.

#### Scenario: Ruta es directorio
- **WHEN** una ruta seleccionada corresponde a un directorio
- **THEN** la app no intenta leerla como archivo de texto
- **AND** deriva al flujo de listado o estado vacío según contenido

#### Scenario: Ruta es archivo
- **WHEN** una ruta seleccionada corresponde a un archivo válido
- **THEN** la app mantiene el flujo normal de lectura y edición
