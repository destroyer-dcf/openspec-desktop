## ADDED Requirements

### Requirement: Panel horizontal de descripción de proyecto
La aplicación SHALL mostrar un panel horizontal de descripción del proyecto activo en el área `content`, usando como fuente los campos del bloque `contexto` de `config.yaml`.

#### Scenario: Proyecto con contexto completo
- **WHEN** el proyecto activo tiene `config.yaml` con bloque `contexto` y claves válidas
- **THEN** la app muestra el panel con pares etiqueta/valor para esos campos, encima de la sección de cambios activos

#### Scenario: Proyecto sin contexto
- **WHEN** `config.yaml` no existe, no contiene `contexto`, o hay claves ausentes
- **THEN** la app muestra el panel con valores fallback "No definido" sin lanzar error

### Requirement: Comportamiento responsive del panel
La aplicación SHALL mantener el panel de descripción en una fila prioritaria con ajuste de línea en anchos pequeños, sin solapar el resto del contenido.

#### Scenario: Ventana estrecha
- **WHEN** el ancho disponible de `content` es reducido
- **THEN** los elementos del panel hacen wrap y el contenido inferior permanece accesible sin superposición
