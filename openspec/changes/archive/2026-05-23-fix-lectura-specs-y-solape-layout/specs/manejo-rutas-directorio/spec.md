## ADDED Requirements

### Requirement: Protección ante rutas de directorio
La aplicación SHALL detectar cuando una ruta seleccionada es un directorio y bloquear intento de lectura como fichero Markdown.

#### Scenario: Ruta seleccionada es directorio
- **WHEN** el usuario selecciona un artifact cuya ruta resuelve a carpeta
- **THEN** la app no llama lectura de fichero y muestra mensaje funcional de recurso no editable

### Requirement: Manejo de carpeta vacía en specs
La aplicación SHALL manejar carpetas vacías dentro de `specs/` sin lanzar error de sistema.

#### Scenario: Carpeta specs vacía
- **WHEN** existe subcarpeta en `specs/` sin fichero Markdown
- **THEN** la app mantiene navegación estable y muestra estado vacío/no-documento en vez de error OS
