## MODIFIED Requirements

### Requirement: Lista de proyectos en barra lateral
La app SHALL mostrar los proyectos en el sidebar como tarjetas, con nombre en negrita en primera línea y path del proyecto en segunda línea con tipografía más pequeña.

#### Scenario: Render de proyectos
- **WHEN** existen múltiples proyectos registrados
- **THEN** cada proyecto se muestra en tarjeta con nombre destacado y path secundario

#### Scenario: Selección de proyecto
- **WHEN** la persona usuaria selecciona una tarjeta
- **THEN** la tarjeta seleccionada se muestra en color azul y activa ese proyecto

### Requirement: Gestión de proyectos en sidebar
La app SHALL permitir desvincular un proyecto desde su tarjeta mediante botón `X`, quitándolo de la lista de la aplicación sin borrar la carpeta en disco.

#### Scenario: Desvincular proyecto
- **WHEN** la persona usuaria pulsa `X` en una tarjeta
- **THEN** ese proyecto deja de mostrarse en el sidebar y se actualiza la persistencia de proyectos abiertos

### Requirement: Footer y responsive del sidebar
La app SHALL ubicar la acción de configuración en el footer del sidebar y mantener layout responsive usable en anchos reducidos.

#### Scenario: Sidebar en viewport reducido
- **WHEN** el ancho disponible es pequeño
- **THEN** las tarjetas, texto y acciones del sidebar mantienen legibilidad y no se solapan
