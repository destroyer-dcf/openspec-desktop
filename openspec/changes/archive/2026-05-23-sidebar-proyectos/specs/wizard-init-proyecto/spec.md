## ADDED Requirements

### Requirement: Asistente de inicialización de proyecto
La app SHALL mostrar un asistente modal cuando se selecciona una carpeta sin `.openspec/`, para recoger los datos necesarios e inicializar el proyecto con `openspec init`.

#### Scenario: Apertura del wizard
- **WHEN** el usuario selecciona una carpeta sin `.openspec/`
- **THEN** se abre un modal con un formulario de inicialización de proyecto

#### Scenario: Campos obligatorios vacíos
- **WHEN** el usuario intenta confirmar el wizard con el campo "Nombre del proyecto" vacío
- **THEN** el wizard muestra un error de validación y no ejecuta `openspec init`

#### Scenario: Cancelar wizard
- **WHEN** el usuario cierra el wizard sin confirmar
- **THEN** no se crea ningún proyecto y la carpeta no se añade a la barra lateral

### Requirement: Formulario de contexto del proyecto
El wizard SHALL incluir campos para todos los datos del contexto del proyecto: nombre, idioma, audiencia, dominio, descripción, stack, architecture (opcional) y deployment flow (opcional).

#### Scenario: Campos opcionales vacíos
- **WHEN** el usuario deja "Architecture" y/o "Deployment flow" en blanco y confirma el wizard
- **THEN** esos campos NO se escriben en el `config.yaml` generado

#### Scenario: Campos opcionales con valor
- **WHEN** el usuario rellena "Architecture" y/o "Deployment flow" y confirma el wizard
- **THEN** esos campos SÍ se incluyen en el `config.yaml` generado

### Requirement: Selección de proveedor de IA
El wizard SHALL ofrecer al usuario la selección del proveedor de IA con el que trabajará el proyecto, con las opciones: Codex, Copilot y OpenCode.

#### Scenario: Selección de proveedor
- **WHEN** el usuario selecciona un proveedor de IA en el wizard
- **THEN** el proveedor queda registrado y se incluye en el `config.yaml` del proyecto

#### Scenario: Sin proveedor seleccionado
- **WHEN** el usuario intenta confirmar el wizard sin seleccionar proveedor de IA
- **THEN** el wizard muestra un error de validación y no ejecuta `openspec init`

### Requirement: Ejecución de openspec init
El wizard SHALL ejecutar `openspec init` al confirmar, crear el `config.yaml` con los datos del formulario y añadir el proyecto a la barra lateral.

#### Scenario: Inicialización exitosa
- **WHEN** el usuario confirma el wizard con todos los datos válidos
- **THEN** la app ejecuta `openspec init` en la carpeta seleccionada, escribe el `config.yaml`, cierra el modal y añade el proyecto a la barra lateral como proyecto activo

#### Scenario: openspec CLI no disponible
- **WHEN** el usuario confirma el wizard pero `openspec` no está instalado en el PATH
- **THEN** el wizard muestra un error descriptivo indicando que la CLI no está instalada y no crea el proyecto
