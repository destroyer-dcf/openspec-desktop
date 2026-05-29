## 1. Refactoring del estado multi-proyecto (Rust)

- [x] 1.1 Definir struct `ProjectHandle` con campos: `path`, `name`, `state: ProjectState`
- [x] 1.2 Reemplazar `ProjectState` único en Tauri managed state por `AppState { projects: Vec<ProjectHandle>, active_index: Option<usize> }`
- [x] 1.3 Adaptar Tauri command `get_state` para devolver el proyecto activo según `active_index`
- [x] 1.4 Adaptar el file watcher para registrar un watcher por proyecto añadido

## 2. Persistencia de proyectos (Rust)

- [x] 2.1 Implementar `load_app_state()`: lee `app-state.json` desde `app_data_dir()` al arrancar
- [x] 2.2 Implementar `save_app_state()`: escribe `app-state.json` con la lista de rutas al añadir/eliminar proyectos
- [x] 2.3 Al arrancar, intentar recargar cada ruta persistida; eliminar silenciosamente las que no existen en disco
- [x] 2.4 Validar: cerrar y reabrir la app restaura los proyectos previos en la barra lateral

## 3. Selector nativo de carpetas (Rust + Tauri)

- [x] 3.1 Añadir `tauri-plugin-dialog` como dependencia en `Cargo.toml`
- [x] 3.2 Implementar Tauri command `pick_project_folder`: abre el selector nativo con `directory: true` y devuelve la ruta seleccionada (o `None` si se cancela)
- [x] 3.3 Validar: cancelar el selector no modifica la barra lateral

## 4. Detección y enrutado al wizard (Rust)

- [x] 4.1 Modificar Tauri command `open_project`: si `.openspec/config.yaml` existe → carga el proyecto; si no → devuelve señal `needs_init` con la ruta
- [x] 4.2 Validar: carpeta con `.openspec/` malformado devuelve error descriptivo
- [x] 4.3 Validar: carpeta sin `.openspec/` devuelve `{ status: "needs_init", path: "..." }` al frontend

## 5. Ejecución de openspec init (Rust)

- [x] 5.1 Implementar Tauri command `check_openspec_cli`: verifica que `openspec` está en el PATH (`which openspec`)
- [x] 5.2 Implementar Tauri command `init_project`: recibe la ruta y los campos del wizard, escribe el `config.yaml` mínimo y ejecuta `openspec init` como proceso hijo capturando stdout/stderr
- [x] 5.3 Lógica de serialización YAML: omitir los campos `architecture` y `deployment_flow` si son `None` o cadena vacía
- [x] 5.4 Validar: `openspec init` exitoso → el proyecto queda accesible y se añade al `AppState`
- [x] 5.5 Validar: CLI no instalada → devuelve error descriptivo al frontend

## 6. Barra lateral (frontend Svelte)

- [x] 6.1 Crear componente `Sidebar.svelte`: lista de proyectos como ítems con nombre y resaltado del activo
- [x] 6.2 Añadir botón "+" en la barra lateral que llama a `pick_project_folder`
- [x] 6.3 Al recibir respuesta de `open_project`:
  - Si `status: "loaded"` → añadir a la lista y activar
  - Si `status: "needs_init"` → abrir `WizardInit.svelte`
  - Si `status: "error"` → mostrar mensaje de error
- [x] 6.4 Al hacer clic en un ítem de la lista → llamar a Tauri command `set_active_project` y refrescar el dashboard
- [x] 6.5 Estado vacío: mostrar texto de bienvenida cuando no hay proyectos
- [x] 6.6 Validar: cambiar de proyecto activo actualiza el dashboard sin recargar la app

## 7. Wizard de inicialización (frontend Svelte)

- [x] 7.1 Crear componente `WizardInit.svelte` con campos: Nombre (obligatorio), Idioma, Audiencia, Dominio, Descripción, Stack (obligatorio), Architecture (opcional), Deployment flow (opcional)
- [x] 7.2 Añadir selector de proveedor de IA con opciones: Codex, Copilot, OpenCode (selección obligatoria)
- [x] 7.3 Validación en frontend: bloquear confirmación si Nombre o proveedor de IA están vacíos
- [x] 7.4 Al confirmar: llamar a `check_openspec_cli` primero; si falla → mostrar error de CLI no instalada
- [x] 7.5 Si CLI disponible → llamar a `init_project` con los datos del formulario; mostrar feedback de progreso
- [x] 7.6 Tras inicialización exitosa → cerrar modal, añadir proyecto a la barra lateral y activarlo
- [x] 7.7 Botón Cancelar → cerrar modal sin crear proyecto
- [x] 7.8 Validar: dejar Architecture y Deployment flow vacíos → no aparecen en el `config.yaml` generado
- [x] 7.9 Validar: rellenar Architecture y Deployment flow → sí aparecen en el `config.yaml` generado

## 8. Validación general

- [x] 8.1 Flujo completo: añadir proyecto con `.openspec/` → aparece en sidebar → cambiar proyecto activo → dashboard correcto
- [x] 8.2 Flujo completo: añadir carpeta vacía → wizard → rellenar → confirmar → proyecto en sidebar
- [x] 8.3 Reiniciar app → proyectos restaurados desde persistencia
- [x] 8.4 Cancelar selector de carpeta → sin cambios en la UI
- [x] 8.5 Cancelar wizard → sin cambios en la UI y sin ficheros creados en disco
