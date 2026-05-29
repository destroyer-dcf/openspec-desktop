## 1. Backend de propuestas (Tauri + FS)

- [x] 1.1 Definir modelo `Proposal` con `name`, `type`, `createdAt`, `path`, `status` y contenido markdown.
- [x] 1.2 Implementar comando para listar propuestas activas y archivadas desde `opencode/propose/actives` y `opencode/propose/archived`.
- [x] 1.3 Implementar comando para crear propuesta nueva en markdown con header (`name`, `type`) y fecha de creación.
- [x] 1.4 Implementar comando para actualizar propuesta existente desde el modal.
- [x] 1.5 Implementar comando de archivado múltiple moviendo propuestas seleccionadas de `actives` a `archived`.

## 2. UI del panel de propuestas en dashboard

- [x] 2.1 Añadir panel “Propuestas” debajo de “Cambios activos” dentro de `content`.
- [x] 2.2 Renderizar propuestas activas en grid responsive con nombre, fecha y botón “Modificar”.
- [x] 2.3 Añadir botón “Añadir propuesta” en el panel.
- [x] 2.4 Añadir selección múltiple en tarjetas y acción “Archivar seleccionadas”.

## 3. Modal Propuesta y edición markdown

- [x] 3.1 Implementar modal “Propuesta” con campos: nombre identificador, tipo (`Feature`/`Bug`) y editor markdown.
- [x] 3.2 Implementar acción Guardar para crear/editar propuesta persistiendo en `opencode/propose/actives`.
- [x] 3.3 Implementar acción Descartar para cerrar/revertir cambios no guardados.
- [x] 3.4 Validar que el archivo markdown resultante incluye header con tipo y nombre.

## 4. Integración y validación funcional

- [x] 4.1 Validar flujo completo de alta: añadir propuesta -> guardar -> ver en grid con fecha.
- [x] 4.2 Validar flujo de edición: modificar propuesta -> guardar -> refresco en panel.
- [x] 4.3 Validar archivado múltiple: seleccionar varias -> archivar -> desaparecen de activas.
- [x] 4.4 Validar que propuestas archivadas quedan en `opencode/propose/archived`.
