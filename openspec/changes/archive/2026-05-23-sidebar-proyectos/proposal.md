## Por que

La app actualmente solo puede trabajar con un proyecto a la vez y no tiene forma de inicializar proyectos nuevos desde la propia interfaz. Necesitamos una barra lateral que permita gestionar múltiples proyectos OpenSpec y un asistente de inicialización para carpetas que todavía no tienen `.openspec/`.

## Qué Cambia

- Se añade una barra lateral izquierda permanente con la lista de proyectos cargados.
- El usuario puede añadir proyectos seleccionando una carpeta desde el sistema de ficheros.
- Si la carpeta seleccionada contiene `.openspec/`, se carga directamente.
- Si la carpeta **no** contiene `.openspec/`, se abre un asistente de inicialización donde se rellenan los datos del proyecto (contexto, stack, etc.), se elige el proveedor de IA (Codex, Copilot, OpenCode) y se ejecuta `openspec init`.
- Los campos `Architecture` y `Deployment flow` son opcionales: si se dejan en blanco no se escriben en el `config.yaml`.
- Tras la inicialización, el proyecto queda cargado y visible en la barra lateral.
- Se puede cambiar de proyecto activo haciendo clic en cualquier ítem de la barra lateral.

## Capacidades

### Nuevas Capacidades
- `sidebar-proyectos`: Barra lateral izquierda que lista los proyectos cargados, permite añadir nuevos seleccionando un directorio y cambiar el proyecto activo con un clic.
- `wizard-init-proyecto`: Asistente modal que se muestra cuando la carpeta seleccionada no tiene `.openspec/`. Recoge los datos del contexto del proyecto (nombre, idioma, audiencia, dominio, descripción, stack, architecture opcional, deployment flow opcional), permite elegir el proveedor de IA (Codex, Copilot, OpenCode) y ejecuta `openspec init` para crear el proyecto.

### Capacidades Modificadas
- `deteccion-proyecto`: El flujo de detección ahora puede iniciarse desde la barra lateral (múltiples proyectos) y, si no se encuentra `.openspec/`, delega en el wizard en lugar de mostrar solo un error.

## Impacto

- Requiere Tauri command para abrir el selector nativo de carpetas (`tauri::api::dialog`).
- Requiere Tauri command para ejecutar `openspec init` como proceso externo (`std::process::Command`).
- El estado de la app pasa de un único `ProjectState` a una colección `Vec<ProjectState>` con un índice de proyecto activo.
- La persistencia de la lista de proyectos cargados entre sesiones requiere un fichero local de configuración de la app (ej. `app-state.json` en el directorio de datos de la app).
