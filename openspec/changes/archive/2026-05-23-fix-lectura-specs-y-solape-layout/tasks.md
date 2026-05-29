## 1. Manejo seguro de rutas de artifacts

- [x] 1.1 Detectar en frontend/backend cuando ruta seleccionada es directorio
- [x] 1.2 Evitar llamada de lectura Markdown para rutas de carpeta
- [x] 1.3 Mostrar mensaje funcional "recurso no editable" para rutas no-fichero
- [x] 1.4 Validar caso real: carpeta vacía dentro de `specs/` no lanza `os error 21`

## 2. Corrección de solape de layout

- [x] 2.1 Revisar contenedores sidebar/content y aplicar restricciones anti-solape (`min-width:0`, columnas estables)
- [x] 2.2 Corregir layout en dashboard para que panel cambios activos no invada sidebar
- [x] 2.3 Corregir layout en vista de artifacts (proposal/design/tasks/spec) para evitar invasión lateral
- [x] 2.4 Validar con contenido largo y múltiples elementos en lista

## 3. Validación funcional UX

- [x] 3.1 Validar navegación normal de artifacts sigue intacta
- [x] 3.2 Validar apertura de directorio muestra mensaje correcto y app no rompe
- [x] 3.3 Validar desktop/móvil sin solape en sidebar+content
- [x] 3.4 Validar accesibilidad básica de mensajes y foco tras error controlado
