## MODIFIED Requirements

### Requirement: Asistente de inicialización de proyecto
La app SHALL mostrar un asistente modal cuando se selecciona una carpeta sin `.openspec/`, para recoger los datos necesarios e inicializar el proyecto con `openspec init`, incluyendo un combo de selección de schema OpenSpec.

#### Scenario: Apertura del wizard
- **WHEN** el usuario selecciona una carpeta sin `.openspec/`
- **THEN** se abre un modal con un formulario de inicialización de proyecto que incluye selector de schema

#### Scenario: Campos obligatorios vacíos
- **WHEN** el usuario intenta confirmar el wizard con el campo "Nombre del proyecto" vacío
- **THEN** el wizard muestra un error de validación y no ejecuta `openspec init`

#### Scenario: Cancelar wizard
- **WHEN** el usuario cierra el wizard sin confirmar
- **THEN** no se crea ningún proyecto y la carpeta no se añade a la barra lateral

### Requirement: Ejecución de openspec init
El wizard SHALL ejecutar `openspec init` al confirmar, crear el `config.yaml` con los datos del formulario y añadir el proyecto a la barra lateral, escribiendo el schema elegido en la configuración.

#### Scenario: Inicialización exitosa
- **WHEN** el usuario confirma el wizard con todos los datos válidos y un schema seleccionado
- **THEN** la app ejecuta `openspec init` en la carpeta seleccionada, escribe el `config.yaml` con ese `schema`, cierra el modal y añade el proyecto a la barra lateral como proyecto activo

#### Scenario: openspec CLI no disponible
- **WHEN** el usuario confirma el wizard pero `openspec` no está instalado en el PATH
- **THEN** el wizard muestra un error descriptivo indicando que la CLI no está instalada y no crea el proyecto
