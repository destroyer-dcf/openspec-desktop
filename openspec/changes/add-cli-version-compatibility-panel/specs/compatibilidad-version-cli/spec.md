## ADDED Requirements

### Requirement: Archivo interno de compatibilidad de versiones CLI
El sistema SHALL cargar un archivo interno mantenido por desarrollo que defina las reglas de compatibilidad de versiones de `opencode` CLI para la versión actual de la aplicación.

#### Scenario: Carga correcta de reglas de compatibilidad
- **WHEN** la aplicación inicia y el archivo interno de compatibilidad es válido
- **THEN** el sistema SHALL disponer de las reglas para evaluar compatibilidad de la versión CLI detectada

### Requirement: Soporte de rangos semánticos con operadores relacionales
El sistema SHALL aceptar reglas de compatibilidad expresadas como rangos semánticos incluyendo operadores `<`, `<=`, `>`, `>=`.

#### Scenario: Regla con operador menor que
- **WHEN** una regla de compatibilidad incluye una expresión con operador `<`
- **THEN** el sistema SHALL evaluar la versión CLI detectada respetando dicha restricción

#### Scenario: Regla con operador mayor que
- **WHEN** una regla de compatibilidad incluye una expresión con operador `>`
- **THEN** el sistema SHALL evaluar la versión CLI detectada respetando dicha restricción

### Requirement: Evaluación de compatibilidad de la versión CLI instalada
El sistema SHALL determinar un estado de compatibilidad (`compatible`, `incompatible` o `unknown`) comparando la versión de `opencode` CLI detectada con las reglas del archivo interno.

#### Scenario: Versión compatible
- **WHEN** la versión CLI detectada satisface al menos una regla válida de compatibilidad
- **THEN** el sistema SHALL establecer el estado como `compatible`

#### Scenario: Versión no compatible
- **WHEN** la versión CLI detectada no satisface ninguna regla válida de compatibilidad
- **THEN** el sistema SHALL establecer el estado como `incompatible`

#### Scenario: Estado indeterminado por error de detección o configuración
- **WHEN** la versión CLI no puede detectarse o las reglas no pueden evaluarse de forma fiable
- **THEN** el sistema SHALL establecer el estado como `unknown`

### Requirement: Panel visual de compatibilidad en cabecera derecha
El sistema SHALL mostrar un panel persistente en la parte superior derecha, pegado al borde derecho, con el texto `Opencode Desktop`, la versión CLI detectada y el estado de compatibilidad.

#### Scenario: Render del panel con datos de versión y estado
- **WHEN** la aplicación tiene disponible un estado de compatibilidad
- **THEN** el panel SHALL mostrar `Opencode Desktop`, la versión CLI detectada y un indicador del estado resultante

### Requirement: Semántica de color del panel por estado
El sistema SHALL colorear el panel de compatibilidad según estado: verde para `compatible`, rojo para `incompatible` y amarillo para `unknown`.

#### Scenario: Panel en verde
- **WHEN** el estado de compatibilidad es `compatible`
- **THEN** el panel SHALL mostrarse en color verde

#### Scenario: Panel en rojo
- **WHEN** el estado de compatibilidad es `incompatible`
- **THEN** el panel SHALL mostrarse en color rojo

#### Scenario: Panel en amarillo
- **WHEN** el estado de compatibilidad es `unknown`
- **THEN** el panel SHALL mostrarse en color amarillo
