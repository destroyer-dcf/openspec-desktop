## 1. Modelo de configuración y persistencia

- [x] 1.1 Extender el modelo de configuración para incluir colores por panel/tipo (activos completado/pendiente, propuestas feature/bug, archivados).
- [x] 1.2 Añadir opción `sin color` en el modelo y en la capa de serialización/deserialización.
- [x] 1.3 Garantizar valores por defecto retrocompatibles para proyectos con configuración previa.

## 2. Modal de configuración

- [x] 2.1 Crear sección de configuración de colores por panel/tipo con controles separados y labels claros.
- [x] 2.2 Implementar selectores con opciones de color permitidas y opción `sin color`.
- [x] 2.3 Guardar cambios y reflejar estado actual al reabrir el modal.

## 3. Aplicación visual en paneles

- [x] 3.1 Aplicar color configurable a tarjetas de cambios activos según completado vs pendiente.
- [x] 3.2 Aplicar color configurable a tarjetas de propuestas según tipo feature/bug.
- [x] 3.3 Aplicar color configurable a tarjetas de cambios archivados.
- [x] 3.4 Implementar fallback neutral cuando el valor sea `sin color`.

## 4. Consistencia de estilo y validación

- [x] 4.1 Alinear tokens/clases de color con el patrón visual del sidebar seleccionado (fondo, borde y contraste equivalente).
- [x] 4.2 Verificar legibilidad en tema claro y oscuro para cada variante de color.
- [x] 4.3 Ejecutar validaciones de build/check y ajustar detalles visuales finales.
