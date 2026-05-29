## 1. Modelo y datos de archivados

- [x] 1.1 Extender el modelo de cambios archivados para incluir fecha de aplicación/archivo normalizada.
- [x] 1.2 Implementar fallback "Fecha no disponible" cuando no exista metadato o timestamp utilizable.

## 2. Grid de cambios archivados en dashboard

- [x] 2.1 Reemplazar el listado actual de archivados por un layout grid responsive.
- [x] 2.2 Mostrar en cada tarjeta: nombre del cambio, fecha y botón "Consultar".
- [x] 2.3 Verificar que la sección conserva alineación y no solapa con otros paneles del content.

## 3. Modal de consulta solo lectura

- [x] 3.1 Implementar apertura de modal al pulsar "Consultar" con contexto del cambio archivado.
- [x] 3.2 Mostrar listado de documentos archivados (proposal/design/specs/tasks) en el modal.
- [x] 3.3 Mostrar contenido del documento seleccionado en visor readonly.
- [x] 3.4 Ocultar o deshabilitar cualquier acción de edición/guardado mientras el modal esté en modo archivado.

## 4. Validación funcional

- [x] 4.1 Validar flujo completo: grid -> botón consultar -> modal -> selección de documento.
- [x] 4.2 Validar que no se pueden editar documentos archivados por UI ni atajos de edición.
- [x] 4.3 Validar comportamiento con cambios archivados sin fecha disponible o con documentos faltantes.
