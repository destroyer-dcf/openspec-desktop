# OpenSpec Desktop

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](#requisitos)
[![OpenSpec CLI](https://img.shields.io/badge/OpenSpec_CLI-1.3.1-green.svg)](https://github.com/Fission-AI/OpenSpec)
[![App](https://img.shields.io/badge/OpenSpec_Desktop-1.0.0-1f6feb.svg)](#versiones)

<p align="center">
  <img src="../openspec-desktop.png" alt="OpenSpec Desktop" />
</p>

<p align="center">
  🇪🇸 <a href="README.es-ES.md">Español</a> | 🇺🇸 <a href="README.en-US.md">English</a>
</p>


OpenSpec Desktop es una aplicación de escritorio para gestionar proyectos basados en [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec). Su objetivo es actuar como una interfaz visual para el flujo de trabajo de OpenSpec: propuestas, cambios activos, tareas, especificaciones, documentos markdown, cambios archivados y configuración del proyecto.

OpenSpec es un framework de desarrollo guiado por especificaciones para asistentes de programación con IA. OpenSpec Desktop no sustituye al CLI: lo complementa con una experiencia de escritorio para inspeccionar, editar y operar sobre la estructura `openspec/` de un proyecto.

**Base operativa obligatoria:** OpenSpec Desktop trabaja siempre sobre dos fuentes:  
1) el flujo y comandos de `openspec` CLI, y  
2) los artefactos markdown que genera el flujo asistido por IA (proposals, design, specs, tasks).  
A partir de esa base, la app de escritorio permite visualizar, editar, organizar y dar seguimiento al contenido; no redefine el modelo de OpenSpec ni reemplaza su CLI.

### Características

- Gestión de múltiples proyectos OpenSpec desde una barra lateral redimensionable.
- Vista de estado del proyecto con resumen, métricas y progreso global.
- Paneles de pipeline para propuestas, cambios activos y cambios archivados.
- Gestión de propuestas en markdown, con tipos `Feature` y `Bug`.
- Consulta y edición de documentos de cambios activos: `proposal`, `design`, `tasks` y `specs`.
- Vista de cambios archivados en modo solo lectura.
- Marcado de tareas desde la vista markdown.
- Configuración visual: tema claro/oscuro, idioma, tamaño de texto y colores de tarjetas.
- Soporte multiidioma: español, inglés, francés, alemán y portugués.
- Integración con el CLI de OpenSpec para inicializar proyectos.
- Aplicación de escritorio multiplataforma basada en Tauri.

### Qué puedes hacer con OpenSpec Desktop

- Abrir carpeta de proyecto y detectar automáticamente si ya contiene estructura `openspec/`.
- Inicializar proyecto OpenSpec nuevo con wizard (schema, idioma, reglas y contexto).
- Crear propuestas `Feature` o `Bug`, editarlas en markdown y archivarlas/eliminarlas.
- Filtrar propuestas por tipo y estado (`active` / `archived`) para revisión rápida.
- Copiar contenido markdown de propuestas y nombres de cambios al portapapeles.
- Consultar cambios activos, ver progreso de tareas y abrir artefactos asociados.
- Editar artefactos (`proposal`, `design`, `tasks`, `specs`) desde interfaz.
- Marcar tareas en `tasks.md` y ver reflejado progreso del cambio en panel.
- Consultar cambios archivados y documentación histórica en modo solo lectura.
- Ajustar preferencias de UI (tema, idioma, densidad, colores) y conservarlas localmente.

### Tecnologías

| Capa | Tecnología |
| --- | --- |
| Escritorio | Tauri 2 |
| Backend local | Rust |
| Frontend | SvelteKit, Svelte 5, TypeScript |
| UI | CSS custom properties, lucide-svelte |
| Markdown | marked, DOMPurify |
| Persistencia local | Estado de aplicación Tauri + filesystem |
| Integración OpenSpec | CLI `openspec` y estructura `openspec/` |

### Versiones

| Componente | Versión |
| --- | --- |
| OpenSpec Desktop | `1.0.0` |
| OpenSpec CLI verificado localmente | `1.3.1` |
| Tauri | `2.x` |
| Svelte | `5.x` |
| TypeScript | `~5.6.x` |

### Requisitos

- Node.js `20.19.0` o superior.
- Rust estable con Cargo.
- Dependencias del sistema requeridas por Tauri para tu plataforma.
- OpenSpec CLI instalado para inicializar proyectos desde la aplicación.

Instalación del CLI de OpenSpec:

```bash
npm install -g @fission-ai/openspec@latest
```

Verificación:

```bash
openspec --version
```

Nota: la aplicación puede arrancar sin el CLI de OpenSpec si solo se cargan proyectos ya existentes desde disco. Para inicializar un proyecto nuevo desde el wizard, `openspec` debe estar instalado y disponible en el `PATH`.

### Instalación

```bash
npm install
```

### Desarrollo

Arrancar la aplicación en modo desarrollo:

```bash
npm run tauri dev
```

Ejecutar solo el frontend Vite:

```bash
npm run dev
```

Validar TypeScript y Svelte:

```bash
npm run check
```

Validar Rust:

```bash
cd src-tauri
cargo check
```

### Compilación

Compilar frontend:

```bash
npm run build
```

Compilar aplicación de escritorio:

```bash
npm run tauri build
```

Los artefactos generados por Tauri quedan bajo `src-tauri/target/`.

### Estructura principal

```text
.
├── src/                    # Frontend SvelteKit
├── src/lib/components/      # Componentes de interfaz
├── src/lib/i18n.ts          # Diccionarios multiidioma
├── src-tauri/               # Aplicación Tauri/Rust
├── src-tauri/src/commands/  # Comandos invocados desde el frontend
├── src-tauri/src/openspec/  # Lectura y modelado de proyectos OpenSpec
├── static/brand/            # Recursos de marca
└── openspec/                # Especificación del propio proyecto
```

### Relación con OpenSpec

OpenSpec Desktop es un front visual para [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec). Trabaja sobre los archivos que OpenSpec usa para organizar el flujo de especificación:

- `openspec/config.yaml`
- `openspec/changes/<change>/proposal.md`
- `openspec/changes/<change>/design.md`
- `openspec/changes/<change>/tasks.md`
- `openspec/changes/<change>/specs/**/*.md`
- `openspec/changes/archive/**`

La aplicación no pretende reemplazar los comandos `/opsx:*` ni el CLI. Su papel es facilitar la visualización, edición y seguimiento del estado de los proyectos.

### Documentación y Graphify

La documentación navegable del proyecto también puede construirse con Graphify, que genera un grafo de conocimiento a partir del código y documentos del repositorio.

- Repositorio oficial: [safishamsi/graphify](https://github.com/safishamsi/graphify)
- Salida principal en este proyecto: `graphify-out/`
- Archivos clave generados:
- `graphify-out/graph.json`: grafo estructurado (nodos y relaciones).
- `graphify-out/graph.html`: visualización interactiva del grafo.
- `graphify-out/GRAPH_REPORT.md`: reporte de comunidades, nodos relevantes y conexiones.
- `graphify-out/manifest.json`: estado para actualizaciones incrementales.
- `graphify-out/cache/`: cachés de extracción para acelerar reconstrucciones.

Flujo recomendado en este repositorio:

```bash
graphify update .
```

Con eso se reextrae el grafo del estado actual del proyecto y se actualiza la documentación conectada basada en grafo.

### Licencia

MIT.

---

## Autor

**Destroyer**

Creador y mantenedor de OpenSpec Desktop. Proyecto desarrollado como una interfaz de escritorio profesional para trabajar con OpenSpec de forma visual, local y cómoda.

OpenSpec original project: [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec)
