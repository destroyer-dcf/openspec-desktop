## MODIFIED Requirements

### Requirement: Progreso de tareas
La app SHALL mostrar el progreso de tareas de un cambio activo con una barra de progreso lineal, el texto `X/Y tareas` y un porcentaje textual destacado en negrita y tamaño mayor dentro de la zona de acciones de la tarjeta.

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems `- [ ]` / `- [x]`
- **THEN** la app muestra una barra de progreso, el texto `X/Y tareas` y el porcentaje textual destacado (por ejemplo `75%`) a la derecha de los botones de acción

#### Scenario: tasks.md sin ítems de lista o ausente
- **WHEN** el cambio no tiene `tasks.md` o el fichero no contiene ítems de lista
- **THEN** la app muestra `Sin tareas` sin barra de progreso y mantiene una representación textual de porcentaje coherente (`0%`) sin rosco circular
