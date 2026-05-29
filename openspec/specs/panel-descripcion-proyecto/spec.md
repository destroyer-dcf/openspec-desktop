# panel-descripcion-proyecto Specification

## Purpose
TBD - created by archiving change panel-descripcion-proyecto-horizontal. Update Purpose after archive.
## Requirements
### Requirement: Panel horizontal de descripción de proyecto
La aplicación SHALL mostrar un panel horizontal de descripción del proyecto activo en el área `content`, usando como fuente los campos del bloque `contexto` de `config.yaml`, con metadatos presentados línea a línea en una zona izquierda y el estado general de proyecto basado en barras integrado en la zona derecha.

#### Scenario: Proyecto con contexto completo
- **WHEN** el proyecto activo tiene `config.yaml` con bloque `contexto` y claves válidas
- **THEN** la app muestra el panel con pares etiqueta/valor en líneas separadas y el bloque de barras de estado general en el lateral derecho

#### Scenario: Proyecto sin contexto
- **WHEN** `config.yaml` no existe, no contiene `contexto`, o hay claves ausentes
- **THEN** la app muestra el panel con valores fallback `No definido` y mantiene el bloque de estado general sin lanzar error

### Requirement: Comportamiento responsive del panel
La aplicación SHALL mantener el panel de descripción en una fila prioritaria con ajuste visual en anchos menores de escritorio, sin solapar el resto del contenido y preservando la visibilidad del rosco.

#### Scenario: Ventana estrecha
- **WHEN** el ancho disponible de `content` es reducido
- **THEN** los elementos del panel se redistribuyen sin superposición y el contenido inferior permanece accesible
