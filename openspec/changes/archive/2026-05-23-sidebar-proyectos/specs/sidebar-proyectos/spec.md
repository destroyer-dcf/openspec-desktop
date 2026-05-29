## ADDED Requirements

### Requirement: Lista de proyectos en barra lateral
La app SHALL mostrar una barra lateral izquierda permanente con la lista de todos los proyectos cargados en la sesión.

#### Scenario: Barra lateral con proyectos cargados
- **WHEN** la app tiene uno o más proyectos cargados
- **THEN** la barra lateral muestra el nombre de cada proyecto como ítem seleccionable

#### Scenario: Barra lateral vacía
- **WHEN** no hay proyectos cargados
- **THEN** la barra lateral muestra un estado vacío con una invitación a añadir el primer proyecto

### Requirement: Añadir proyecto desde barra lateral
La app SHALL permitir añadir un nuevo proyecto seleccionando una carpeta del sistema de ficheros mediante el selector nativo del SO.

#### Scenario: Añadir proyecto con .openspec existente
- **WHEN** el usuario pulsa el botón de añadir proyecto y selecciona una carpeta que contiene `.openspec/config.yaml`
- **THEN** el proyecto se carga y aparece en la barra lateral como nuevo ítem

#### Scenario: Añadir proyecto sin .openspec
- **WHEN** el usuario pulsa el botón de añadir proyecto y selecciona una carpeta sin `.openspec/`
- **THEN** la app abre el wizard de inicialización en lugar de mostrar un error

#### Scenario: Cancelar selector de carpeta
- **WHEN** el usuario abre el selector de carpetas y lo cierra sin seleccionar nada
- **THEN** la barra lateral no cambia y no se muestra ningún error

### Requirement: Cambio de proyecto activo
La app SHALL permitir cambiar el proyecto activo haciendo clic en cualquier ítem de la barra lateral.

#### Scenario: Clic en proyecto de la barra lateral
- **WHEN** el usuario hace clic en un proyecto de la barra lateral
- **THEN** el dashboard principal muestra el estado de ese proyecto y el ítem queda resaltado como activo

### Requirement: Persistencia de proyectos entre sesiones
La app SHALL recordar los proyectos cargados y restaurarlos automáticamente al volver a abrirse.

#### Scenario: Reapertura de la app con proyectos previos
- **WHEN** la app se cierra con proyectos cargados y se vuelve a abrir
- **THEN** los proyectos se restauran automáticamente en la barra lateral

#### Scenario: Proyecto eliminado del disco entre sesiones
- **WHEN** la app se reabre y una ruta persistida ya no existe en disco
- **THEN** ese proyecto se elimina silenciosamente de la lista sin mostrar un error bloqueante
