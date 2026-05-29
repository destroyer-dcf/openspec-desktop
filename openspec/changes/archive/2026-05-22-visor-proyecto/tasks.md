## 1. Scaffolding del proyecto Tauri

- [x] 1.1 Inicializar proyecto Tauri con Svelte como frontend
- [x] 1.2 Configurar estructura de carpetas: `src-tauri/src/openspec/` y `src-tauri/src/commands/`
- [x] 1.3 Añadir dependencias Rust: `serde`, `serde_yaml`, `notify`, `tauri`
- [x] 1.4 Añadir dependencias frontend: `marked`, `dompurify`

## 2. Detección y carga del proyecto

- [x] 2.1 Implementar `scanner.rs`: recorre la carpeta seleccionada y verifica la existencia de `.openspec/config.yaml`
- [x] 2.2 Implementar `parser.rs`: parsea `config.yaml` con `serde_yaml` a structs tipados (`ProjectConfig`)
- [x] 2.3 Implementar `model.rs`: definir structs `Project`, `Change`, `Artifact`, `ChangeStatus`
- [x] 2.4 Implementar Tauri command `open_project`: recibe ruta, ejecuta scanner/parser, devuelve `ProjectState` al frontend
- [x] 2.5 Validar: carpeta sin `.openspec/` devuelve error descriptivo al frontend
- [x] 2.6 Validar: `config.yaml` malformado devuelve error descriptivo al frontend

## 3. File watcher (actualización en tiempo real)

- [x] 3.1 Integrar crate `notify` en `src-tauri/src/` para observar cambios en `.openspec/`
- [x] 3.2 Emitir evento Tauri `project-updated` al frontend cuando se detecte un cambio en disco
- [x] 3.3 Validar: crear un fichero en `.openspec/changes/` desde terminal actualiza el dashboard en menos de 2 segundos

## 4. Carga de cambios y artifacts

- [x] 4.1 Implementar lectura de `changes/`: listar subdirectorios, detectar artifacts presentes (`proposal.md`, `design.md`, `tasks.md`, `specs/`)
- [x] 4.2 Implementar lectura de `changes/archive/`: listar cambios archivados con sus metadatos
- [x] 4.3 Implementar parseo de `tasks.md` para extraer conteo de tareas (`- [ ]` vs `- [x]`)
- [x] 4.4 Implementar Tauri command `get_state`: devuelve el `ProjectState` completo (cambios, specs, config)

## 5. Dashboard (frontend)

- [x] 5.1 Crear vista `Dashboard.svelte`: lista de cambios activos con nombre y estado de artifacts
- [x] 5.2 Añadir sección de cambios archivados en el dashboard
- [x] 5.3 Mostrar indicador de progreso de tareas por cambio (ej. `3/7 tareas`)
- [x] 5.4 Mostrar estado vacío cuando no hay cambios ni specs
- [x] 5.5 Suscribirse al evento `project-updated` para refrescar el dashboard automáticamente

## 6. Editor/visor de Markdown

- [x] 6.1 Crear vista `EditorMarkdown.svelte` con panel de edición (textarea) y panel de preview (HTML renderizado)
- [x] 6.2 Implementar preview en tiempo real: renderizar Markdown con `marked` + sanitizar con `dompurify` al escribir
- [x] 6.3 Implementar Tauri command `read_file`: lee el contenido de un artifact y lo devuelve al frontend
- [x] 6.4 Implementar Tauri command `write_file`: recibe ruta y contenido, escribe en disco
- [x] 6.5 Añadir botón Guardar: llama a `write_file` con el contenido del editor
- [x] 6.6 Añadir botón Cancelar: restaura el contenido original sin modificar el fichero
- [x] 6.7 Validar: abrir un artifact inexistente muestra mensaje "documento aún no creado"

## 7. Navegación entre artifacts

- [x] 7.1 Crear vista `ChangeDetail.svelte`: muestra los artifacts de un cambio como ítems navegables
- [x] 7.2 Al seleccionar un artifact, abrir `EditorMarkdown.svelte` con su contenido sin recargar el proyecto
- [x] 7.3 Validar: navegar entre artifacts mantiene el estado del dashboard intacto

## 8. Validación general

- [x] 8.1 Probar flujo completo: abrir carpeta → detectar proyecto → ver dashboard → abrir artifact → editar → guardar
- [x] 8.2 Probar que cambios externos (CLI) se reflejan en la app en tiempo real
- [x] 8.3 Probar carpetas sin `.openspec/` muestran el error correcto
- [x] 8.4 Probar cancelar edición no modifica el fichero en disco
