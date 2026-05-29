## 1. Ajuste de UI en tarjetas activas

- [x] 1.1 Eliminar el indicador circular de porcentaje de las tarjetas de cambios activos
- [x] 1.2 Añadir indicador textual de porcentaje (negrita y tamaño mayor) alineado a la derecha del bloque de acciones
- [x] 1.3 Ajustar layout/CSS para que botones e indicador textual convivan sin solapes en anchos reducidos de escritorio

## 2. Consistencia funcional

- [x] 2.1 Reutilizar el cálculo actual de progreso por cambio sin modificar la lógica de datos
- [x] 2.2 Mantener visible el texto `X/Y tareas` y el estado `Sin tareas` según corresponda
- [x] 2.3 Verificar que cambios archivados, propuestas y resumen global no se ven afectados por este ajuste

## 3. Validación manual

- [x] 3.1 Validar que una tarjeta con progreso parcial muestra correctamente porcentaje textual y `X/Y tareas`
- [x] 3.2 Validar que una tarjeta sin tareas no muestra errores y conserva representación coherente (`0%` + `Sin tareas`)
- [x] 3.3 Validar alineación visual en diferentes anchos de ventana de escritorio (mínimo 1024)
