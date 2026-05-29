# dashboard-estado Specification

## Purpose
TBD - created by archiving change visor-proyecto. Update Purpose after archive.
## Requirements
### Requirement: Vista general del estado del proyecto
La app SHALL mostrar el dashboard en composición de pipeline visual (Propuestas, Cambios activos, Cambios archivados) y SHALL renderizar en el área `content` un panel superior de descripción con estado general en barras que refleje métricas agregadas del proyecto.

#### Scenario: Dashboard con cambios activos
- **WHEN** el proyecto tiene cambios en `changes/`
- **THEN** el panel superior muestra barras y contadores actualizados con el estado general del proyecto

#### Scenario: Dashboard sin cambios
- **WHEN** el proyecto no tiene cambios activos ni archivados
- **THEN** el dashboard muestra estados vacíos claros y el bloque de barras en cero sin errores

### Requirement: Estado de artifacts por cambio
La app SHALL reflejar estado de solo lectura para cambios completos al abrir su modal de documentos, desactivando acciones de edición/guardado coherentes con la completitud. Para cambios activos, la app SHALL mostrar el estado de cada artifact con un icono visual diferenciado (✓ completado / ○ pendiente / ⊘ bloqueado), manteniendo tasks en formato icon-only sin texto adicional.

#### Scenario: Cambio completo
- **WHEN** se abre un cambio con tareas completas igual al total
- **THEN** el modal permite consulta de documentos pero desactiva acciones de edición y guardado

#### Scenario: Cambio incompleto
- **WHEN** se abre un cambio con tareas pendientes
- **THEN** el modal mantiene disponibles acciones de edición/guardado según flujo normal

#### Scenario: Cambio con todos los artifacts
- **WHEN** se selecciona un cambio que tiene proposal, design, specs y tasks
- **THEN** los iconos de artifacts se muestran con su estado y tasks se representa solo con icono

#### Scenario: Cambio sin completar
- **WHEN** se selecciona un cambio que solo tiene proposal
- **THEN** proposal muestra icono ✓ y los artifacts faltantes (design, specs, tasks) muestran icono ○

### Requirement: Progreso de tareas
La app SHALL mostrar en cada tarjeta de cambio activo el progreso en porcentaje dentro de un círculo ubicado arriba a la derecha, y SHALL organizar sus acciones en una fila separada debajo de la fila de artifacts.

#### Scenario: tasks.md con ítems de lista
- **WHEN** el cambio tiene `tasks.md` con ítems `- [ ]` / `- [x]`
- **THEN** la app muestra porcentaje en círculo y deja los botones de acción debajo de artifacts

#### Scenario: tasks.md sin ítems de lista o ausente
- **WHEN** el cambio no tiene `tasks.md` o el fichero no contiene ítems de lista
- **THEN** la app muestra progreso `0%` en círculo y mantiene la misma estructura de tarjeta

### Requirement: Tarjetas activas con color por completitud
El panel de cambios activos SHALL aplicar color configurable distinto para tarjetas completadas y pendientes.

#### Scenario: Tarjeta completada
- **WHEN** un cambio activo tiene tareas completas igual al total
- **THEN** la tarjeta usa el color configurado para estado completado

#### Scenario: Tarjeta pendiente
- **WHEN** un cambio activo tiene tareas completas menor que el total
- **THEN** la tarjeta usa el color configurado para estado pendiente

### Requirement: Fallback neutral en activos
El panel de cambios activos SHALL mantener estilo neutral cuando el color configurado sea `sin color`.

#### Scenario: Sin color en activos
- **WHEN** la configuración del estado correspondiente en activos está en `sin color`
- **THEN** la tarjeta no aplica variante de color

### Requirement: Artifact specs mostrado como contenedor navegable
El dashboard SHALL tratar el artifact `specs` como contenedor de documentos cuando su ruta sea directorio.

#### Scenario: Specs con subcarpetas
- **WHEN** el artifact `specs` apunta a un directorio con uno o más `spec.md`
- **THEN** la app permite navegar y abrir esos documentos sin error

#### Scenario: Specs sin documentos
- **WHEN** el artifact `specs` apunta a un directorio sin `spec.md`
- **THEN** la app muestra estado vacío con opción de crear especificación

### Requirement: Contexto funcional visible en cambios activos
El dashboard SHALL incluir debajo del nombre del cambio activo una vista resumida del objetivo del cambio basado en `proposal.md/Why`.

#### Scenario: Texto largo de Why
- **WHEN** el bloque `Why` excede el espacio de tarjeta
- **THEN** la UI lo trunca visualmente a un máximo de dos líneas
