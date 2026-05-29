# deteccion-proyecto Specification

## Purpose
TBD - created by archiving change visor-proyecto. Update Purpose after archive.
## Requirements
### Requirement: Detección de proyecto OpenSpec
La app SHALL detectar si una carpeta seleccionada contiene un directorio `.openspec/` válido con un fichero `config.yaml`. Si existe, carga el proyecto. Si no existe, delega en el wizard de inicialización en lugar de mostrar un error genérico.

#### Scenario: Carpeta válida con .openspec
- **WHEN** el usuario añade una carpeta desde la barra lateral y contiene `.openspec/config.yaml`
- **THEN** la app carga el proyecto y lo añade a la barra lateral como ítem activo

#### Scenario: Carpeta sin .openspec
- **WHEN** el usuario añade una carpeta desde la barra lateral y no contiene `.openspec/`
- **THEN** la app abre el wizard de inicialización en lugar de mostrar un error

#### Scenario: .openspec presente pero config.yaml ausente o inválido
- **WHEN** la carpeta contiene `.openspec/` pero no tiene `config.yaml` o está malformado
- **THEN** la app muestra un error descriptivo indicando que el proyecto está mal configurado y no lo añade a la barra lateral

### Requirement: Carga de estructura del proyecto
La app SHALL leer y exponer en memoria la estructura completa del proyecto: configuración, cambios activos, cambios archivados y specs.

#### Scenario: Proyecto con cambios activos y archivados
- **WHEN** el proyecto se carga correctamente
- **THEN** la app muestra todos los cambios en `changes/` (activos) y `changes/archive/` (archivados) con sus artifacts

#### Scenario: Proyecto vacío (sin cambios ni specs)
- **WHEN** el proyecto se carga y no tiene cambios ni specs
- **THEN** la app muestra el dashboard vacío sin errores

### Requirement: Actualización en tiempo real
La app SHALL reflejar automáticamente cualquier cambio en el directorio `.openspec/` del proyecto activo producido por herramientas externas.

#### Scenario: Nuevo artifact creado externamente
- **WHEN** se crea un fichero dentro de `.openspec/changes/` desde fuera de la app
- **THEN** la app actualiza el estado del proyecto activo en menos de 2 segundos sin recargar manualmente

