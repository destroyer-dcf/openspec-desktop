## ADDED Requirements

### Requirement: Detección de proyecto OpenSpec
La app SHALL detectar si una carpeta abierta contiene un directorio `.openspec/` válido con un fichero `config.yaml` en su interior y cargar su estructura completa.

#### Scenario: Carpeta válida con .openspec
- **WHEN** el usuario abre una carpeta que contiene `.openspec/config.yaml`
- **THEN** la app carga el proyecto y muestra su estado en el dashboard

#### Scenario: Carpeta sin .openspec
- **WHEN** el usuario abre una carpeta que no contiene `.openspec/`
- **THEN** la app muestra un mensaje indicando que la carpeta no es un proyecto OpenSpec y no carga ningún dato

#### Scenario: .openspec presente pero config.yaml ausente o inválido
- **WHEN** el usuario abre una carpeta con `.openspec/` pero sin `config.yaml` o con YAML malformado
- **THEN** la app muestra un error descriptivo indicando que el proyecto está mal configurado

### Requirement: Carga de estructura del proyecto
La app SHALL leer y exponer en memoria la estructura completa del proyecto: configuración, cambios activos, cambios archivados y specs.

#### Scenario: Proyecto con cambios activos y archivados
- **WHEN** el proyecto se carga correctamente
- **THEN** la app muestra todos los cambios en `changes/` (activos) y `changes/archive/` (archivados) con sus artifacts

#### Scenario: Proyecto vacío (sin cambios ni specs)
- **WHEN** el proyecto se carga y no tiene cambios ni specs
- **THEN** la app muestra el dashboard vacío sin errores

### Requirement: Actualización en tiempo real
La app SHALL reflejar automáticamente cualquier cambio en el directorio `.openspec/` producido por herramientas externas (CLI, editor de texto).

#### Scenario: Nuevo artifact creado externamente
- **WHEN** se crea un fichero dentro de `.openspec/changes/` desde fuera de la app
- **THEN** la app actualiza el estado del proyecto en menos de 2 segundos sin recargar manualmente
