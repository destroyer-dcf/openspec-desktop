## Why

Cuando el usuario intenta consultar `specs` y la ruta es un directorio, la app intenta leerlo como archivo y muestra un error técnico (`Is a directory`). Esto rompe la UX y no guía al usuario sobre qué hacer cuando no hay ficheros de especificación.

## What Changes

- Detectar explícitamente cuando `specs` es directorio y evitar tratarlo como documento único.
- Mostrar estado vacío amigable: "No existen ficheros de especificaciones" en lugar de error.
- Añadir acción para crear nueva especificación desde la UI cuando la carpeta `specs/` está vacía.
- Mantener lectura normal cuando sí existen `spec.md` dentro de subcarpetas.

## Capabilities

### New Capabilities
- `estado-vacio-specs-con-crear`: UX específica para carpeta de specs sin ficheros, incluyendo CTA para crear spec.

### Modified Capabilities
- `manejo-rutas-directorio`: Ajusta el comportamiento para no leer directorios como archivos.
- `dashboard-estado`: Mejora la presentación de artifacts tipo directorio sin error técnico.
- `editor-markdown`: Soporta iniciar creación de spec desde estado vacío de specs.

## Impact

- Lógica frontend que abre artifacts (`Dashboard`, modal/visor de cambio).
- Comandos backend de lectura/escritura para validar ruta archivo vs directorio.
- Flujo de creación de archivos en `openspec/changes/<change>/specs/<capability>/spec.md`.
