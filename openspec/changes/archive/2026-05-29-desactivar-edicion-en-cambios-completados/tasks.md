## 1. Bloqueo de acciones en UI

- [x] 1.1 Identificar en modal/visor markdown los botones de editar y guardar ligados a cambios activos
- [x] 1.2 Aplicar estado `disabled` a editar/guardar cuando `canModify=false` (cambio completo)
- [x] 1.3 Añadir feedback visual/tooltip de solo lectura para acciones desactivadas

## 2. Protección de lógica de guardado

- [x] 2.1 Añadir guardas en handlers para impedir persistencia si `canModify=false`
- [x] 2.2 Verificar que cambios incompletos siguen permitiendo editar y guardar
- [x] 2.3 Verificar que consulta de documentos permanece operativa en cambios completos

## 3. Validación manual

- [x] 3.1 Confirmar que en cambios completos editar/guardar aparecen desactivados
- [x] 3.2 Confirmar que en cambios incompletos editar/guardar siguen activos
- [x] 3.3 Confirmar que no hay regresiones en lectura/preview de markdown
