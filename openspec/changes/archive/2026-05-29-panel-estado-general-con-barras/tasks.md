## 1. Sustitución del resumen global por barras

- [x] 1.1 Reemplazar el rosco global del panel de descripción por un bloque de estado general en barras
- [x] 1.2 Implementar métricas agregadas mínimas: total cambios, activos, archivados, total tareas y completadas
- [x] 1.3 Mostrar porcentaje global destacado junto a la visualización de barras

## 2. Integración y layout

- [x] 2.1 Integrar el bloque de barras en la zona derecha del panel de descripción sin romper el contenido contextual izquierdo
- [x] 2.2 Ajustar estilos responsive de escritorio para evitar solapes en anchos reducidos
- [x] 2.3 Mantener estado vacío consistente (valores cero) cuando no haya cambios o tareas

## 3. Validación funcional

- [x] 3.1 Verificar que las barras y métricas se actualizan al cambiar de proyecto
- [x] 3.2 Verificar que los valores agregados coinciden con los datos reales de cambios y tareas
- [x] 3.3 Verificar que el pipeline (propuestas/activos/archivados) no sufre regresiones visuales por el nuevo panel
