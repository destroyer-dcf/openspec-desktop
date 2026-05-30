## MODIFIED Requirements

### Requirement: Panel horizontal de descripción de proyecto
La aplicación SHALL mostrar un panel horizontal de descripción del proyecto activo en el área `content`, usando como fuente los campos del bloque `contexto` de `config.yaml`, con metadatos presentados línea a línea en una zona izquierda y el estado general de proyecto basado en barras integrado en la zona derecha, e SHALL incluir explícitamente la línea de `Schema` leída del mismo `config.yaml`.

#### Scenario: Proyecto con contexto completo
- **WHEN** el proyecto activo tiene `config.yaml` con bloque `contexto` y claves válidas
- **THEN** la app muestra el panel con pares etiqueta/valor en líneas separadas, incluyendo `Schema`, y el bloque de barras de estado general en el lateral derecho

#### Scenario: Proyecto sin contexto
- **WHEN** `config.yaml` no existe, no contiene `contexto`, o hay claves ausentes
- **THEN** la app muestra el panel con valores fallback `No definido`, muestra `Schema: No disponible` y mantiene el bloque de estado general sin lanzar error
