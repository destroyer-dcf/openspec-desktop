## 1. Detección de ruta y manejo de artifacts specs

- [x] 1.1 Detectar cuando la ruta de artifact es directorio y evitar lectura como archivo.
- [x] 1.2 Implementar flujo de listado de `specs/*/spec.md` para artifacts de tipo directorio.
- [x] 1.3 Mantener lectura/edición normal para artifacts que sí son archivos.

## 2. Estado vacío y creación de especificaciones

- [x] 2.1 Mostrar mensaje "No existen ficheros de especificaciones" cuando no hay `spec.md`.
- [x] 2.2 Añadir CTA "Crear especificación" en el estado vacío.
- [x] 2.3 Implementar creación de `specs/<capability>/spec.md` con plantilla inicial.
- [x] 2.4 Validar nombre capability (no vacío, normalizado a kebab-case).
- [x] 2.5 Refrescar lista y abrir automáticamente la especificación recién creada.

## 3. UX y mensajes de error

- [x] 3.1 Suprimir error técnico `Is a directory (os error 21)` para caso esperado de carpeta.
- [x] 3.2 Mantener mensajes de error solo para fallos reales de IO/permiso.
- [x] 3.3 Verificar consistencia del flujo en cambios con specs y sin specs.

## 4. Validación final

- [x] 4.1 Probar manualmente: carpeta specs vacía, con specs existentes y creación nueva.
- [x] 4.2 Ejecutar `npm run check` y corregir incidencias.
