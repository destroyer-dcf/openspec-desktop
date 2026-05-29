## Context

Proyecto nuevo desde cero. No existe estado previo ni código que migrar. La app desktop debe leer la estructura de ficheros de `.openspec/` tal como la define la CLI de OpenSpec, sin modificarla ni extenderla. La fuente de verdad es siempre el sistema de ficheros.

## Goals / Non-Goals

**Goals:**
- App desktop cross-platform (macOS primero) con Tauri + Rust.
- Detección automática de proyectos OpenSpec al abrir una carpeta.
- Dashboard reactivo que refleja el estado real del directorio `.openspec/`.
- Editor/visor de Markdown integrado con guardado directo en disco.
- Actualizaciones en tiempo real cuando los ficheros cambian externamente (CLI, editor externo).

**Non-Goals:**
- No reemplaza ni extiende la CLI de OpenSpec.
- No gestiona control de versiones (git).
- No sincroniza con servicios remotos ni nube.
- No soporta múltiples proyectos abiertos simultáneamente en v1.

## Decisions

### Rust para toda la lógica de dominio
Toda la lectura de ficheros, parseo de YAML y gestión del estado del proyecto vive en Rust. El frontend solo recibe datos ya procesados vía Tauri commands.
- **Alternativa descartada**: leer ficheros desde el frontend JS — introduce inconsistencias y duplica lógica.

### Svelte como framework frontend
Ligero, sin virtual DOM, ideal para aplicaciones Tauri donde el tamaño del bundle importa. La reactividad de Svelte se alinea bien con un modelo de estado que se actualiza por eventos del backend.
- **Alternativa descartada**: React — overhead innecesario para una app de escritorio sin ecosistema compartido.

### `notify` crate para file watching
Detecta cambios en `.openspec/` en tiempo real y dispara actualizaciones del estado sin polling. Emite eventos Tauri al frontend cuando hay cambios relevantes.
- **Alternativa descartada**: polling periódico — latencia innecesaria y consumo de CPU.

### `serde_yaml` para parseo de YAML
Parseo tipado de `config.yaml` y `.openspec.yaml` de cada cambio. Los structs de Rust reflejan exactamente el esquema de OpenSpec.

### Estado global en Tauri managed state
El `ProjectState` (config, cambios, specs) vive en Rust como estado gestionado por Tauri. El frontend lo obtiene mediante `invoke()` y se actualiza vía eventos Tauri.

### Editor Markdown con preview integrado
El frontend renderiza Markdown con una librería JS ligera (marked + DOMPurify). La edición es en texto plano con preview en tiempo real. El guardado escribe directamente en disco vía Tauri command.

## Risks / Trade-offs

- **File watcher en macOS** → El crate `notify` usa FSEvents en macOS. Riesgo: latencia en sistemas de ficheros montados en red. Mitigación: documentar que la app está optimizada para ficheros locales.
- **Formato de `.openspec.yaml` puede evolucionar** → Si la CLI cambia el esquema, los structs Rust necesitan actualización. Mitigación: parseo permisivo con campos opcionales donde sea posible.
- **Editor básico sin syntax highlighting de Markdown** → En v1 es aceptable; se puede añadir CodeMirror en iteraciones futuras.
