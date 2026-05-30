# AGENT.md - OpenSpec Desktop

## 1) Resumen del proyecto

OpenSpec Desktop es una aplicacion de escritorio (Tauri + SvelteKit) para gestionar proyectos basados en OpenSpec mediante interfaz visual.

Actua como front local para:
- Propuestas (`openspec/propose/active` y `openspec/propose/archived`)
- Cambios activos y archivados (`openspec/changes`)
- Documentos Markdown de cada cambio (`proposal.md`, `design.md`, `tasks.md`, `specs/*`)
- Estado global de progreso del proyecto
- Configuracion visual y de idioma

No reemplaza OpenSpec CLI: lo complementa con gestion visual y edicion guiada.

## 2) Stack tecnico

- Escritorio: Tauri 2
- Backend local: Rust
- Frontend: SvelteKit + Svelte 5 + TypeScript
- UI: CSS (custom properties) + `lucide-svelte`
- Markdown: `marked` + `DOMPurify`
- Persistencia local de preferencias: almacenamiento local de UI + estado cargado desde filesystem
- Integracion externa: CLI `openspec`

## 3) Versiones relevantes

- App: `0.1.0`
- OpenSpec CLI validado en entorno: `1.3.1`
- Nombre de producto mostrado: `OpenSpec Desktop`

## 4) Objetivo funcional

Permitir al usuario gestionar el ciclo OpenSpec completo desde escritorio:
- Ver estado de cambios/propuestas
- Inspeccionar y editar documentos Markdown cuando el cambio lo permite
- Marcar tareas y consultar avance
- Archivar y consultar historico
- Configurar tema, idioma y preferencias visuales

## 5) Estructura del repositorio

- `src/`: frontend SvelteKit
- `src/lib/components/`: componentes UI (dashboard, modales, sidebar, editor, etc.)
- `src/lib/i18n.ts`: traducciones y textos
- `src/lib/types.ts`: contratos de datos del frontend
- `src-tauri/src/`: backend Rust (comandos, carga OpenSpec, parsing, escaneo)
- `src-tauri/src/commands/project.rs`: comandos principales de lectura/escritura de proyecto
- `src-tauri/src/openspec/`: modelo, loader y scanner para estructura OpenSpec
- `openspec/`: datos funcionales de especificaciones/cambios/propuestas
- `static/brand/`: assets de marca

## 6) Componentes clave de UI

- Sidebar de proyectos: seleccion, alta y desvinculacion de carpetas
- Panel de descripcion de proyecto: contexto y metricas generales
- Panel de propuestas: listado, filtros, creacion, edicion y archivado
- Panel de cambios activos: estado, progreso y acceso a documentos
- Panel de cambios archivados: consulta historica en modo lectura
- Modales de documentos: vista previa/edicion segun estado del cambio
- Modal de configuracion: tema, idioma y preferencias de visualizacion

## 7) Flujo de datos

1. Frontend invoca comandos Tauri.
2. Backend Rust escanea `openspec/` del proyecto seleccionado.
3. Se construyen modelos de cambios/propuestas/tareas/documentos.
4. Frontend renderiza paneles y estados.
5. Acciones del usuario (editar, guardar, archivar, crear) llaman de nuevo a comandos Rust.

## 8) Reglas funcionales importantes

- Si un cambio esta `complete`, no debe permitirse editar/guardar documentos.
- Si una ruta esperada es directorio y no fichero, no debe romper la UI; se debe mostrar estado vacio o alternativa valida.
- Los iconos/estados de documentos deben reflejar existencia real de archivos.
- La UI debe mantenerse estable al hacer scroll y en layout escritorio.
- Los tooltips y textos deben respetar idioma configurado.

## 9) Integracion con OpenSpec CLI

Uso tipico desde terminal del proyecto:

```bash
openspec --version
openspec list --json
openspec status --change "<nombre>" --json
openspec instructions apply --change "<nombre>" --json
```

La app depende del CLI para ciertas operaciones de inicializacion/sincronizacion. Para solo lectura de proyectos existentes, muchas vistas pueden funcionar sin invocar comandos de escritura del CLI.

## 10) Comandos de desarrollo

```bash
npm install
npm run dev
npm run tauri dev
npm run check
```

Para revisar backend:

```bash
cd src-tauri
cargo check
```

## 11) Convenciones de mantenimiento

- Mantener coherencia entre estado OpenSpec real y estado mostrado en UI.
- Priorizar cambios pequenos y focalizados por tarea.
- Evitar acoplar logica de negocio en demasiados componentes; centralizar en comandos Rust/modelos.
- Validar regresiones visuales en tema claro y oscuro.
- Conservar nombres de producto y branding: `OpenSpec Desktop`.

## 12) Riesgos conocidos / puntos sensibles

- Parseo Markdown con indices por bytes (especial cuidado con UTF-8 y slicing de strings en Rust).
- Diferencias de comportamiento por plataforma (macOS, Linux, Windows) en ventana, titlebar e iconos.
- Dependencia del layout para evitar solapes entre sidebar y contenido.
- Sincronizacion de preferencias UI para que cambios de color/tema apliquen en caliente.

## 13) Definicion corta para agentes

OpenSpec Desktop es un front de escritorio para OpenSpec que permite visualizar y operar cambios/propuestas/documentos markdown del arbol `openspec/`, con backend Rust (Tauri) y frontend SvelteKit.
