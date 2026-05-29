## 1. Backend de eliminación en lote

- [x] 1.1 Implementar comando Tauri para eliminar propuestas por lista de rutas.
- [x] 1.2 Asegurar que la eliminación funciona tanto para propuestas activas como archivadas.

## 2. UI del panel de propuestas

- [x] 2.1 Añadir botón icon-only con tooltip para “eliminar seleccionadas” en la barra de acciones.
- [x] 2.2 Mantener consistencia visual con botones de añadir/archivar (tamaño, estilo, estado disabled).
- [x] 2.3 Deshabilitar acción cuando no haya selección.

## 3. Flujo de seguridad y refresco

- [x] 3.1 Añadir confirmación previa antes de eliminar propuestas seleccionadas.
- [x] 3.2 Ejecutar eliminación en lote y refrescar grid/selección al finalizar.

## 4. Validación funcional

- [x] 4.1 Validar eliminación múltiple en propuestas activas.
- [x] 4.2 Validar eliminación múltiple en propuestas archivadas.
- [x] 4.3 Validar tooltips y comportamiento disabled del botón.
