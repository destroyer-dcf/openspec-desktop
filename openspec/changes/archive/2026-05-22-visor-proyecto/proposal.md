## Por qué

No existe una forma visual de inspeccionar el estado de un proyecto OpenSpec. Actualmente hay que navegar manualmente por los ficheros del directorio `.openspec` para entender qué cambios están activos, qué artifacts faltan y cómo van las tareas. Una aplicación desktop centraliza toda esa información y permite trabajar sobre los documentos sin salir de ella.

## Qué Cambia

- Nueva aplicación desktop (Tauri + Rust) con interfaz visual para proyectos OpenSpec.
- Al abrir una carpeta, la app detecta si contiene `.openspec/` y carga su estructura completa.
- Se muestra un dashboard con el estado del proyecto: cambios activos, archivados, artifacts por cambio y progreso de tareas.
- Se incluye un lector/editor de Markdown integrado para abrir y modificar cualquier artifact directamente desde la app.
- Los cambios en disco se reflejan en tiempo real en la interfaz.

## Capacidades

### Nuevas Capacidades
- `deteccion-proyecto`: Detecta si una carpeta contiene `.openspec/`, la valida y carga toda la estructura del proyecto (config, cambios, specs, archivo).
- `dashboard-estado`: Vista principal que muestra el estado global del proyecto: cambios activos y archivados, artifacts de cada cambio con su estado (pendiente, listo, bloqueado) y progreso de tareas.
- `editor-markdown`: Lector y editor de Markdown integrado que permite abrir, visualizar y modificar cualquier artifact del proyecto. Los cambios se guardan directamente en disco.

### Capacidades Modificadas
<!-- Sin cambios en capacidades existentes — proyecto nuevo -->

## Impacto

- Nueva aplicación Tauri independiente; no modifica la CLI de OpenSpec.
- Depende de la estructura de ficheros de `.openspec/` como fuente de verdad.
- Dependencias nuevas: Tauri (Rust), serde_yaml, notify (file watcher), Svelte (frontend).
- Sin impacto en otros sistemas o APIs externas.
