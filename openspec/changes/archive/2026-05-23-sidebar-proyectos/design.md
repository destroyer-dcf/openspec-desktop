## Context

El cambio anterior (`visor-proyecto`) establece la base de la app con un único proyecto activo. Este cambio introduce la gestión multi-proyecto: una barra lateral persistente y un asistente de inicialización para proyectos nuevos. Ambas funcionalidades modifican el modelo de estado de la app (de uno a varios proyectos) y añaden interacción con el sistema operativo (selector de carpetas nativo, ejecución de `openspec init`).

## Goals / Non-Goals

**Goals:**
- Barra lateral izquierda con lista de proyectos cargados y persistencia entre sesiones.
- Selector nativo de carpetas para añadir proyectos.
- Wizard modal de inicialización para carpetas sin `.openspec/`.
- Ejecución de `openspec init` desde la app con los datos del wizard.
- Campos opcionales (`Architecture`, `Deployment flow`) omitidos del `config.yaml` si se dejan vacíos.

**Non-Goals:**
- Gestión de proyectos remotos o en red.
- Sincronización entre instancias de la app.
- Importar/exportar la lista de proyectos.

## Decisions

### Estado multi-proyecto: `Vec<ProjectHandle>` en Tauri managed state
El estado global pasa de un único `ProjectState` a una colección `Vec<ProjectHandle>` donde cada `ProjectHandle` contiene la ruta, el nombre del proyecto y su `ProjectState`. Un índice `active_index: usize` indica el proyecto activo.
- **Alternativa descartada**: múltiples estados independientes — complicaría la sincronización del file watcher.

### Persistencia de proyectos cargados: `app-state.json`
La lista de rutas de proyectos se persiste en el directorio de datos de la app (Tauri `app_data_dir()`). Al arrancar, la app intenta recargar todos los proyectos de la lista; los que ya no existen en disco se eliminan silenciosamente.
- **Alternativa descartada**: registro del sistema / base de datos — innecesario para una lista de rutas.

### Selector nativo de carpetas: `tauri-plugin-dialog`
`tauri-plugin-dialog` expone `open()` con `directory: true` para mostrar el selector nativo del SO sin dependencias adicionales.

### Ejecución de `openspec init`: `std::process::Command`
El comando `openspec init` se ejecuta como proceso hijo desde Rust, pasando los campos del wizard como argumentos o mediante stdin si la CLI lo soporta. La salida (stdout/stderr) se captura y se devuelve al frontend para mostrar feedback.
- El wizard construye el `config.yaml` mínimo antes de ejecutar `openspec init` si la CLI no acepta argumentos directos.

### Campos opcionales en config.yaml
El wizard envía los campos `architecture` y `deployment_flow` como `Option<String>`. Solo se escriben en `config.yaml` si tienen valor. El serializado a YAML filtra los `None` explícitamente.

## Risks / Trade-offs

- **`openspec` CLI no instalada** → La app debe verificar que `openspec` está en el PATH antes de ejecutarlo. Si no lo está, mostrar un error descriptivo con instrucciones de instalación.
- **Wizard rellenado con datos inválidos** → La validación mínima (nombre no vacío) se hace en el frontend; el resto lo valida `openspec init`.
- **File watcher con múltiples proyectos** → Un watcher por proyecto. Si se cargan muchos proyectos simultáneamente puede haber overhead. Mitigación: en v1 el límite razonable es ~10 proyectos.
