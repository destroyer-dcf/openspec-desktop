# Graph Report - .  (2026-05-31)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 475 nodes · 1087 edges · 43 communities (28 shown, 15 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 17 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5484a28a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Permisos Tauri (ACL)|Permisos Tauri (ACL)]]
- [[_COMMUNITY_Núcleo de comandos del proyecto|Núcleo de comandos del proyecto]]
- [[_COMMUNITY_Frontend Svelte (UI principal)|Frontend Svelte (UI principal)]]
- [[_COMMUNITY_Loader y parser de OpenSpec|Loader y parser de OpenSpec]]
- [[_COMMUNITY_Esquema Desktop (JSON)|Esquema Desktop (JSON)]]
- [[_COMMUNITY_Esquema macOS (JSON)|Esquema macOS (JSON)]]
- [[_COMMUNITY_Specs funcionales OpenSpec|Specs funcionales OpenSpec]]
- [[_COMMUNITY_Tooling Node y scripts npm|Tooling Node y scripts npm]]
- [[_COMMUNITY_Configuración Tauri|Configuración Tauri]]
- [[_COMMUNITY_Configuración TypeScript|Configuración TypeScript]]
- [[_COMMUNITY_Modelo de dominio (Rust)|Modelo de dominio (Rust)]]
- [[_COMMUNITY_Permisos core (ACL sets)|Permisos core (ACL sets)]]
- [[_COMMUNITY_Bootstrap y ciclo de vida app|Bootstrap y ciclo de vida app]]
- [[_COMMUNITY_Tipos compartidos frontend|Tipos compartidos frontend]]
- [[_COMMUNITY_Capacidades de permisos|Capacidades de permisos]]
- [[_COMMUNITY_Roadmap funcional del producto|Roadmap funcional del producto]]
- [[_COMMUNITY_Preferencias UI (temaidioma)|Preferencias UI (tema/idioma)]]
- [[_COMMUNITY_Permisos por defecto|Permisos por defecto]]
- [[_COMMUNITY_Internacionalización (i18n)|Internacionalización (i18n)]]
- [[_COMMUNITY_Plugins OpenCode|Plugins OpenCode]]
- [[_COMMUNITY_Skill OpenSpec Continue|Skill OpenSpec Continue]]
- [[_COMMUNITY_Skill OpenSpec New Change|Skill OpenSpec New Change]]
- [[_COMMUNITY_Skill OpenSpec Onboard|Skill OpenSpec Onboard]]
- [[_COMMUNITY_Skill OpenSpec Explore|Skill OpenSpec Explore]]
- [[_COMMUNITY_Skill OpenSpec Verify|Skill OpenSpec Verify]]
- [[_COMMUNITY_UX de configuración visual|UX de configuración visual]]
- [[_COMMUNITY_UX de edición de documentos|UX de edición de documentos]]
- [[_COMMUNITY_Config Svelte|Config Svelte]]
- [[_COMMUNITY_Recomendaciones VSCode|Recomendaciones VSCode]]
- [[_COMMUNITY_Settings VSCode|Settings VSCode]]
- [[_COMMUNITY_Punto de entrada de app|Punto de entrada de app]]
- [[_COMMUNITY_Workflow Build & Release|Workflow Build & Release]]
- [[_COMMUNITY_Feature About legal|Feature About legal]]
- [[_COMMUNITY_Feature persistencia ventana|Feature persistencia ventana]]

## God Nodes (most connected - your core abstractions)
1. `permissions` - 79 edges
2. `commands` - 75 edges
3. `identifier` - 74 edges
4. `String` - 56 edges
5. `Result` - 31 edges
6. `AppState` - 22 edges
7. `State` - 16 edges
8. `InitProjectInput` - 16 edges
9. `Vec` - 15 edges
10. `save_proposal()` - 14 edges

## Surprising Connections (you probably didn't know these)
- `archive_proposals()` --calls--> `loadProposals()`  [EXTRACTED]
  src-tauri/src/commands/project.rs → src/lib/components/Dashboard.svelte
- `openspec-apply-change` --references--> `OpenSpec CLI`  [EXTRACTED]
  .codex/skills/openspec-apply-change/SKILL.md → openspec/specs/wizard-init-proyecto/spec.md
- `run()` --calls--> `bootstrap_state()`  [INFERRED]
  src-tauri/src/lib.rs → src-tauri/src/commands/project.rs
- `run()` --calls--> `persist_window_size()`  [INFERRED]
  src-tauri/src/lib.rs → src-tauri/src/commands/project.rs
- `InitProjectInput` --references--> `String`  [EXTRACTED]
  src/lib/types.ts → src-tauri/src/commands/project.rs

## Import Cycles
- 1-file cycle: `src-tauri/src/openspec/loader.rs -> src-tauri/src/openspec/loader.rs`
- 1-file cycle: `src-tauri/src/commands/project.rs -> src-tauri/src/commands/project.rs`
- 1-file cycle: `src-tauri/src/openspec/parser.rs -> src-tauri/src/openspec/parser.rs`

## Communities (43 total, 15 thin omitted)

### Community 0 - "Permisos Tauri (ACL)"
Cohesion: 0.10
Nodes (79): allow, deny, permissions, commands, description, identifier, allow-app-hide, allow-app-show (+71 more)

### Community 1 - "Núcleo de comandos del proyecto"
Cohesion: 0.11
Nodes (75): active_project_root(), app_state_path(), apply_window_bounds(), AppState, archive_proposals(), bootstrap_state(), build_context(), build_proposal_summary() (+67 more)

### Community 2 - "Frontend Svelte (UI principal)"
Cohesion: 0.09
Nodes (20): confirmDeleteProposal(), copyChangeName(), copyProposalMarkdown(), copyText(), formatArchivedDate(), formatProposalDate(), loadProposals(), proposalSummary() (+12 more)

### Community 3 - "Loader y parser de OpenSpec"
Cohesion: 0.14
Nodes (28): Change, artifact(), collect_archived_documents(), collect_markdown_files(), extract_first_h2_section(), has_spec_documents(), list_specs(), load_project() (+20 more)

### Community 4 - "Esquema Desktop (JSON)"
Cohesion: 0.17
Nodes (28): anyOf, properties, required, definitions, Application, Capability, CapabilityRemote, Number (+20 more)

### Community 5 - "Esquema macOS (JSON)"
Cohesion: 0.17
Nodes (28): anyOf, properties, required, definitions, Application, Capability, CapabilityRemote, Number (+20 more)

### Community 6 - "Specs funcionales OpenSpec"
Cohesion: 0.09
Nodes (23): OpenSpec CLI, opsx-apply prompt, opsx-archive prompt, opsx-bulk-archive prompt, opsx-ff prompt, opsx-propose prompt, opsx-sync prompt, sidebar-proyectos Specification (+15 more)

### Community 7 - "Tooling Node y scripts npm"
Cohesion: 0.09
Nodes (22): description, devDependencies, svelte, svelte-check, @sveltejs/adapter-static, @sveltejs/kit, @sveltejs/vite-plugin-svelte, @tauri-apps/cli (+14 more)

### Community 8 - "Configuración Tauri"
Cohesion: 0.11
Nodes (17): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+9 more)

### Community 9 - "Configuración TypeScript"
Cohesion: 0.17
Nodes (11): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule, skipLibCheck (+3 more)

### Community 10 - "Modelo de dominio (Rust)"
Cohesion: 0.42
Nodes (10): Artifact, Change, ChangeDocument, ChangeStatus, Project, ProjectConfig, TaskProgress, Option (+2 more)

### Community 11 - "Permisos core (ACL sets)"
Cohesion: 0.50
Nodes (8): core, core:app, default_permission, core:event, global_scope_schema, core:image, core:menu, permission_sets

### Community 12 - "Bootstrap y ciclo de vida app"
Cohesion: 0.25
Nodes (8): Menu, Project, R, build_app_menu(), run(), set_macos_application_icon(), AppHandle, Result

### Community 13 - "Tipos compartidos frontend"
Cohesion: 0.22
Nodes (8): Artifact, Change, OpenProjectResponse, ProjectHandle, ProjectState, ProposalDetail, ProposalList, TaskProgress

### Community 14 - "Capacidades de permisos"
Cohesion: 0.29
Nodes (6): default, description, identifier, local, permissions, windows

### Community 15 - "Roadmap funcional del producto"
Cohesion: 0.29
Nodes (7): Vista general del estado del proyecto, Panel de estado general con barras, Gestión de propuestas, Columna única por panel del pipeline, Panel horizontal de descripción de proyecto, Indicador de progreso global y por cambio, Selector de schema en inicialización

### Community 16 - "Preferencias UI (tema/idioma)"
Cohesion: 0.29
Nodes (6): CardColorChoice, CardColorPrefs, DEFAULT_CARD_COLORS, FontScale, ThemeMode, UiLanguage

### Community 17 - "Permisos por defecto"
Cohesion: 0.33
Nodes (5): description, identifier, permissions, $schema, windows

### Community 18 - "Internacionalización (i18n)"
Cohesion: 0.40
Nodes (3): dict, UiKey, UiLanguage

## Knowledge Gaps
- **114 isolated node(s):** `config`, `name`, `version`, `description`, `type` (+109 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `archive_proposals()` connect `Núcleo de comandos del proyecto` to `Frontend Svelte (UI principal)`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Why does `load_project()` connect `Loader y parser de OpenSpec` to `Núcleo de comandos del proyecto`, `Bootstrap y ciclo de vida app`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **What connects `config`, `name`, `version` to the rest of the system?**
  _130 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Permisos Tauri (ACL)` be split into smaller, more focused modules?**
  _Cohesion score 0.09542356377799416 - nodes in this community are weakly interconnected._
- **Should `Núcleo de comandos del proyecto` be split into smaller, more focused modules?**
  _Cohesion score 0.10594668489405332 - nodes in this community are weakly interconnected._
- **Should `Frontend Svelte (UI principal)` be split into smaller, more focused modules?**
  _Cohesion score 0.0919661733615222 - nodes in this community are weakly interconnected._
- **Should `Loader y parser de OpenSpec` be split into smaller, more focused modules?**
  _Cohesion score 0.14193548387096774 - nodes in this community are weakly interconnected._