## 1. Extracción backend de Why

- [x] 1.1 Añadir campo `why_summary` al modelo `Change` serializado a frontend.
- [x] 1.2 Parsear `proposal.md` para extraer bloque `## Why` durante carga de cambios activos.
- [x] 1.3 Aplicar la misma extracción para cambios archivados.
- [x] 1.4 Definir fallback estable cuando no exista `Why`.

## 2. Render en tarjetas del dashboard

- [x] 2.1 Mostrar `why_summary` debajo del título en tarjetas de cambios activos.
- [x] 2.2 Mostrar `why_summary` debajo del título en tarjetas de cambios archivados.
- [x] 2.3 Aplicar estilo secundario y truncado visual a 2 líneas.

## 3. Robustez y validación

- [x] 3.1 Verificar que ausencia de proposal o Why no genere errores visibles.
- [x] 3.2 Revisar contraste y legibilidad en tema claro/oscuro.
- [x] 3.3 Ejecutar `npm run check` y resolver incidencias.
- [x] 3.4 Ejecutar `cargo check` para validar cambios de backend.
