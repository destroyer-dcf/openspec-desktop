# sidebar-proyectos Specification

## Purpose
TBD - created by archiving change sidebar-proyectos. Update Purpose after archive.
## Requirements
### Requirement: Lista de proyectos en barra lateral
La app SHALL mostrar los proyectos en el sidebar como tarjetas, con nombre en negrita en primera línea y path del proyecto en segunda línea con tipografía más pequeña.

#### Scenario: Render de proyectos
- **WHEN** existen múltiples proyectos registrados
- **THEN** cada proyecto se muestra en tarjeta con nombre destacado y path secundario

#### Scenario: Selección de proyecto
- **WHEN** la persona usuaria selecciona una tarjeta
- **THEN** la tarjeta seleccionada se muestra en color azul y activa ese proyecto

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

### Requirement: Gestión de proyectos en sidebar
La app SHALL permitir desvincular un proyecto desde su tarjeta mediante botón `X`, quitándolo de la lista de la aplicación sin borrar la carpeta en disco.

#### Scenario: Desvincular proyecto
- **WHEN** la persona usuaria pulsa `X` en una tarjeta
- **THEN** ese proyecto deja de mostrarse en el sidebar y se actualiza la persistencia de proyectos abiertos

### Requirement: Footer y responsive del sidebar
La app SHALL ubicar la acción de configuración en el footer del sidebar, con texto alineado a la izquierda y estilo visual ligero (sin borde persistente), manteniendo layout de escritorio usable en anchos reducidos.

#### Scenario: Sidebar en viewport reducido
- **WHEN** el ancho disponible es pequeño
- **THEN** las tarjetas, texto y acciones del sidebar mantienen legibilidad y no se solapan

#### Scenario: Alineación del control de configuración
- **WHEN** la persona usuaria visualiza el footer del sidebar
- **THEN** el texto del control de configuración aparece alineado a la izquierda dentro del área clicable

### Requirement: Patrón visual consistente con seleccionado del sidebar
Las variantes de color de tarjetas SHALL usar el mismo patrón visual base del proyecto seleccionado del sidebar (fondo, borde y contraste equivalentes).

#### Scenario: Aplicar patrón visual común
- **WHEN** una tarjeta de cualquier panel tiene un color aplicado
- **THEN** el estilo resultante mantiene el patrón visual base equivalente al seleccionado del sidebar
