# OpenSpec Desktop

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](#requirements)
[![OpenSpec CLI](https://img.shields.io/badge/OpenSpec_CLI-1.3.1-green.svg)](https://github.com/Fission-AI/OpenSpec)
[![App](https://img.shields.io/badge/OpenSpec_Desktop-1.0.0-1f6feb.svg)](#versions)

<p align="center">
  <img src="../openspec-desktop.png" alt="OpenSpec Desktop" />
</p>

<p align="center">
  🇪🇸 <a href="README.es-ES.md">Español</a> | 🇺🇸 <a href="README.en-US.md">English</a>
</p>


OpenSpec Desktop is a desktop application for managing projects based on [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec). It provides a visual interface for the OpenSpec workflow: proposals, active changes, tasks, specifications, markdown documents, archived changes, and project configuration.

OpenSpec is a spec-driven development framework for AI coding assistants. OpenSpec Desktop does not replace the CLI: it complements it with a desktop experience for inspecting, editing, and operating on a project's `openspec/` structure.

### Features

- Manage multiple OpenSpec projects from a resizable sidebar.
- Project status dashboard with summary, metrics, and global progress.
- Pipeline panels for proposals, active changes, and archived changes.
- Markdown proposal management with `Feature` and `Bug` types.
- Read and edit active change documents: `proposal`, `design`, `tasks`, and `specs`.
- Read-only archived change viewer.
- Toggle tasks directly from markdown preview.
- Visual settings: light/dark theme, language, text size, and card colors.
- Multi-language support: Spanish, English, French, German, and Portuguese.
- OpenSpec CLI integration for project initialization.
- Cross-platform desktop app powered by Tauri.

### What You Can Do With OpenSpec Desktop

- Open a project folder and auto-detect whether it already contains an `openspec/` workspace.
- Initialize a new OpenSpec project with the setup wizard (schema, language, rules, context).
- Create `Feature` or `Bug` proposals, edit them in markdown, archive or delete them.
- Filter proposals by type and status (`active` / `archived`) for quick triage.
- Copy proposal markdown content and change names directly to clipboard.
- Inspect active changes, track task progress, and open linked artifacts.
- Edit change artifacts (`proposal`, `design`, `tasks`, `specs`) from the desktop UI.
- Toggle tasks in `tasks.md` and immediately reflect progress in dashboard panels.
- Browse archived changes and historical documents in read-only mode.
- Customize UI preferences (theme, language, density, card colors) with local persistence.

### Technology Stack

| Layer | Technology |
| --- | --- |
| Desktop | Tauri 2 |
| Local backend | Rust |
| Frontend | SvelteKit, Svelte 5, TypeScript |
| UI | CSS custom properties, lucide-svelte |
| Markdown | marked, DOMPurify |
| Local persistence | Tauri app state + filesystem |
| OpenSpec integration | `openspec` CLI and `openspec/` structure |

### Versions

| Component | Version |
| --- | --- |
| OpenSpec Desktop | `1.0.0` |
| Locally verified OpenSpec CLI | `1.3.1` |
| Tauri | `2.x` |
| Svelte | `5.x` |
| TypeScript | `~5.6.x` |

### Requirements

- Node.js `20.19.0` or newer.
- Stable Rust with Cargo.
- Platform-specific system dependencies required by Tauri.
- OpenSpec CLI installed to initialize projects from the app.

Install OpenSpec CLI:

```bash
npm install -g @fission-ai/openspec@latest
```

Verify:

```bash
openspec --version
```

Note: the app can start without the OpenSpec CLI when loading existing projects from disk. To initialize a new project from the wizard, `openspec` must be installed and available in `PATH`.

### Installation

```bash
npm install
```

### Development

Run the desktop app in development mode:

```bash
npm run tauri dev
```

Run only the Vite frontend:

```bash
npm run dev
```

Validate TypeScript and Svelte:

```bash
npm run check
```

Validate Rust:

```bash
cd src-tauri
cargo check
```

### Build

Build frontend:

```bash
npm run build
```

Build desktop application:

```bash
npm run tauri build
```

Tauri build artifacts are generated under `src-tauri/target/`.

### Project Structure

```text
.
├── src/                    # SvelteKit frontend
├── src/lib/components/      # UI components
├── src/lib/i18n.ts          # Multi-language dictionaries
├── src-tauri/               # Tauri/Rust application
├── src-tauri/src/commands/  # Commands invoked from the frontend
├── src-tauri/src/openspec/  # OpenSpec project loading and models
├── static/brand/            # Brand assets
└── openspec/                # This project's own OpenSpec workspace
```

### Relationship With OpenSpec

OpenSpec Desktop is a visual front for [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec). It works with the files OpenSpec uses to organize its specification workflow:

- `openspec/config.yaml`
- `openspec/changes/<change>/proposal.md`
- `openspec/changes/<change>/design.md`
- `openspec/changes/<change>/tasks.md`
- `openspec/changes/<change>/specs/**/*.md`
- `openspec/changes/archive/**`

The app is not intended to replace `/opsx:*` commands or the OpenSpec CLI. Its role is to make project visualization, editing, and progress tracking easier.

Core contract: this desktop app is always built on top of `openspec` CLI behavior and the AI-assisted artifacts generated in the OpenSpec workflow. From that baseline, the desktop UI helps you review, edit, organize, and track those artifacts locally.

### Documentation and Graphify

Project documentation can also be generated as a knowledge graph using Graphify, built from this repository's code and docs.

- Official repository: [safishamsi/graphify](https://github.com/safishamsi/graphify)
- Main output folder in this project: `graphify-out/`
- Key generated files:
- `graphify-out/graph.json`: structured graph data (nodes and relations).
- `graphify-out/graph.html`: interactive graph visualization.
- `graphify-out/GRAPH_REPORT.md`: report with communities, key nodes, and connections.
- `graphify-out/manifest.json`: incremental update state.
- `graphify-out/cache/`: extraction caches for faster rebuilds.

Recommended flow in this repository:

```bash
graphify update .
```

This refreshes the graph from current project state and updates graph-based connected documentation artifacts.

### License

MIT.

---

## Author

**Destroyer**

Creator and maintainer of OpenSpec Desktop. This project was built as a professional desktop interface for working with OpenSpec in a visual, local, and practical way.

OpenSpec original project: [Fission-AI/OpenSpec](https://github.com/Fission-AI/OpenSpec)
