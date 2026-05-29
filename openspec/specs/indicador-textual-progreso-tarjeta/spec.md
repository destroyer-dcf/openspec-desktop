# indicador-textual-progreso-tarjeta Specification

## Purpose
Display textual progress percentage indicator on active change cards.

## Requirements

### Requirement: Indicador textual de progreso por tarjeta activa
La app SHALL mostrar, en cada tarjeta de cambio activo, un indicador textual del porcentaje completado en tipografía destacada (tamaño mayor y negrita), alineado a la derecha del área de acciones.

#### Scenario: Tarjeta con tareas parcialmente completadas
- **WHEN** un cambio activo tiene tareas completadas y pendientes
- **THEN** la tarjeta muestra el porcentaje como texto destacado (por ejemplo `67%`) en la zona derecha de acciones

#### Scenario: Tarjeta sin tareas
- **WHEN** un cambio activo no tiene `tasks.md` o no contiene ítems de lista
- **THEN** la tarjeta muestra el estado de tareas existente y el indicador textual se mantiene en valor coherente (`0%`) sin error visual
