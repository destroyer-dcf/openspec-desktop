## ADDED Requirements

### Requirement: Indicador de progreso global
La app SHALL mostrar un indicador circular que representa el porcentaje de tareas completadas sobre el total de tareas de todos los cambios activos del proyecto.

#### Scenario: Proyecto con tareas activas
- **WHEN** el proyecto activo tiene cambios con `tasks.md` que contienen ítems de lista
- **THEN** el indicador circular muestra el porcentaje completado (ej. "43%") con el círculo relleno proporcionalmente

#### Scenario: Proyecto sin tareas
- **WHEN** el proyecto activo no tiene ningún cambio con `tasks.md` o todos los `tasks.md` están vacíos
- **THEN** el indicador circular muestra "0%" sin mostrar un error

#### Scenario: Todas las tareas completadas
- **WHEN** todos los ítems de todos los `tasks.md` están marcados como `- [x]`
- **THEN** el indicador circular muestra "100%"

### Requirement: Barra de progreso por cambio
La app SHALL mostrar una barra de progreso lineal para cada cambio activo que tenga `tasks.md` con ítems de lista.

#### Scenario: Cambio con tareas parcialmente completadas
- **WHEN** un cambio activo tiene `tasks.md` con ítems `- [ ]` y `- [x]`
- **THEN** la app muestra una barra de progreso con el texto "X/Y tareas" donde X son las completadas e Y el total

#### Scenario: Cambio sin tasks.md o sin ítems de lista
- **WHEN** un cambio activo no tiene `tasks.md` o el fichero no tiene ítems de lista
- **THEN** la app muestra "Sin tareas" en lugar de una barra de progreso

### Requirement: Actualización en tiempo real del progreso
Los indicadores de progreso SHALL reflejar automáticamente los cambios producidos en los `tasks.md` por herramientas externas.

#### Scenario: Tarea marcada como completada externamente
- **WHEN** un ítem de `tasks.md` cambia de `- [ ]` a `- [x]` desde fuera de la app
- **THEN** la barra de progreso del cambio y el indicador global se actualizan en menos de 2 segundos
