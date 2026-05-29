# manejo-rutas-directorio Specification

## Purpose
TBD - created by archiving change fix-lectura-specs-y-solape-layout. Update Purpose after archive.
## Requirements
### Requirement: Protección ante rutas de directorio
La aplicación SHALL detectar cuando una ruta seleccionada es un directorio y bloquear intento de lectura como fichero Markdown.

#### Scenario: Ruta seleccionada es directorio
- **WHEN** el usuario selecciona un artifact cuya ruta resuelve a carpeta
- **THEN** la app no llama lectura de fichero y muestra mensaje funcional de recurso no editable

### Requirement: Detección de rutas de directorio frente a archivo
La aplicación SHALL detectar explícitamente si una ruta objetivo es directorio o archivo antes de intentar leer su contenido, y aplicar flujo diferenciado para cada caso.

#### Scenario: Ruta es directorio
- **WHEN** una ruta seleccionada corresponde a un directorio
- **THEN** la app no intenta leerla como archivo de texto
- **AND** deriva al flujo de listado o estado vacío según contenido

#### Scenario: Ruta es archivo
- **WHEN** una ruta seleccionada corresponde a un archivo válido
- **THEN** la app mantiene el flujo normal de lectura y edición

### Requirement: Manejo de carpeta vacía en specs
La aplicación SHALL manejar carpetas vacías dentro de `specs/` sin lanzar error de sistema.

#### Scenario: Carpeta specs vacía
- **WHEN** existe subcarpeta en `specs/` sin fichero Markdown
- **THEN** la app mantiene navegación estable y muestra estado vacío/no-documento en vez de error OS

