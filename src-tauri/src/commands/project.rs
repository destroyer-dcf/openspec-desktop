use crate::openspec::loader::load_project;
use crate::openspec::model::ProjectState;
use arboard::Clipboard;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHandle {
    pub path: String,
    pub name: String,
    pub state: ProjectState,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct PersistedAppState {
    project_paths: Vec<String>,
    active_index: Option<usize>,
}

#[derive(Default)]
pub struct AppState {
    pub projects: Mutex<Vec<ProjectHandle>>,
    pub active_index: Mutex<Option<usize>>,
    pub watchers: Mutex<Vec<notify::RecommendedWatcher>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum OpenProjectResponse {
    Loaded { project: ProjectState },
    NeedsInit { path: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitProjectInput {
    pub path: String,
    pub name: String,
    pub language: Option<String>,
    pub audience: Option<String>,
    pub domain: Option<String>,
    pub description: Option<String>,
    pub stack: String,
    pub architecture: Option<String>,
    pub deployment_flow: Option<String>,
    pub ai_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub name: String,
    pub proposal_type: String,
    pub created_at: String,
    pub status: String,
    pub path: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalList {
    pub active: Vec<Proposal>,
    pub archived: Vec<Proposal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalDetail {
    pub proposal: Proposal,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecDocument {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub app_version: String,
    pub openspec_version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveProposalInput {
    pub path: Option<String>,
    pub name: String,
    pub proposal_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitConfig {
    schema: String,
    ai_provider: String,
    contexto: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_flow: Option<String>,
}

pub fn bootstrap_state(app: &AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
    let mut persisted = load_app_state(app)?;
    let mut loaded = Vec::new();

    for p in &persisted.project_paths {
        if !Path::new(p).exists() {
            continue;
        }
        if let Ok(project) = load_project(p) {
            loaded.push(ProjectHandle {
                path: p.clone(),
                name: path_name(p),
                state: project,
            });
        }
    }

    if loaded.is_empty() {
        persisted.active_index = None;
    } else if let Some(idx) = persisted.active_index {
        if idx >= loaded.len() {
            persisted.active_index = Some(0);
        }
    } else {
        persisted.active_index = Some(0);
    }

    {
        let mut guard = state
            .projects
            .lock()
            .map_err(|_| "No se pudo bloquear projects".to_string())?;
        *guard = loaded;
    }
    {
        let mut guard = state
            .active_index
            .lock()
            .map_err(|_| "No se pudo bloquear active_index".to_string())?;
        *guard = persisted.active_index;
    }

    register_all_watchers(app, state)?;
    save_app_state(app, state)?;

    Ok(())
}

#[tauri::command]
pub fn get_projects(state: State<'_, AppState>) -> Result<Vec<ProjectHandle>, String> {
    let guard = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?;
    Ok(guard.clone())
}

#[tauri::command]
pub fn get_active_index(state: State<'_, AppState>) -> Result<Option<usize>, String> {
    let guard = state
        .active_index
        .lock()
        .map_err(|_| "No se pudo bloquear active_index".to_string())?;
    Ok(*guard)
}

#[tauri::command]
pub fn set_active_project(index: usize, state: State<'_, AppState>) -> Result<(), String> {
    let len = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?
        .len();
    if index >= len {
        return Err("Índice de proyecto inválido".to_string());
    }

    let mut guard = state
        .active_index
        .lock()
        .map_err(|_| "No se pudo bloquear active_index".to_string())?;
    *guard = Some(index);
    Ok(())
}

#[tauri::command]
pub fn unlink_project(index: usize, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    {
        let mut projects = state
            .projects
            .lock()
            .map_err(|_| "No se pudo bloquear projects".to_string())?;
        if index >= projects.len() {
            return Err("Índice de proyecto inválido".to_string());
        }
        projects.remove(index);
    }

    {
        let projects_len = state
            .projects
            .lock()
            .map_err(|_| "No se pudo bloquear projects".to_string())?
            .len();
        let mut active = state
            .active_index
            .lock()
            .map_err(|_| "No se pudo bloquear active_index".to_string())?;

        *active = match (*active, projects_len) {
            (_, 0) => None,
            (Some(current), len) if current == index => Some(index.min(len - 1)),
            (Some(current), _) if current > index => Some(current - 1),
            (Some(current), _) => Some(current),
            (None, _) => Some(0),
        };
    }

    register_all_watchers(&app, &state)?;
    save_app_state(&app, &state)?;
    Ok(())
}

#[tauri::command]
pub async fn pick_project_folder(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    app.dialog().file().pick_folder(move |path| {
        let _ = tx.send(path.map(|p| p.to_string()));
    });

    rx.await
        .map_err(|e| format!("Error al recibir ruta seleccionada: {e}"))
}

#[tauri::command]
pub fn open_project(path: String, app: AppHandle, state: State<'_, AppState>) -> Result<OpenProjectResponse, String> {
    if !Path::new(&path).join("openspec").exists() {
        return Ok(OpenProjectResponse::NeedsInit { path });
    }

    let project = load_project(&path)?;
    upsert_project(&app, &state, &path, project.clone())?;
    Ok(OpenProjectResponse::Loaded { project })
}

#[tauri::command]
pub fn get_state(state: State<'_, AppState>) -> Result<Option<ProjectState>, String> {
    let projects = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?;
    let active = *state
        .active_index
        .lock()
        .map_err(|_| "No se pudo bloquear active_index".to_string())?;

    Ok(active.and_then(|idx| projects.get(idx).map(|p| p.state.clone())))
}

#[tauri::command]
pub fn list_proposals(state: State<'_, AppState>) -> Result<ProposalList, String> {
    let root = active_project_root(&state)?;
    let propose_root = root.join("openspec").join("propose");
    let actives_dir = propose_root.join("actives");
    let archived_dir = propose_root.join("archived");
    fs::create_dir_all(&actives_dir).map_err(|e| format!("No se pudo crear actives: {e}"))?;
    fs::create_dir_all(&archived_dir).map_err(|e| format!("No se pudo crear archived: {e}"))?;

    Ok(ProposalList {
        active: read_proposals_from_dir(&actives_dir, "active")?,
        archived: read_proposals_from_dir(&archived_dir, "archived")?,
    })
}

#[tauri::command]
pub fn get_proposal(path: String) -> Result<ProposalDetail, String> {
    let raw = fs::read_to_string(&path).map_err(|e| format!("No se pudo leer propuesta {path}: {e}"))?;
    let (meta, body) = parse_markdown_with_frontmatter(&raw);
    let name = meta
        .get("name")
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .unwrap_or_else(|| file_stem_or_default(Path::new(&path), "propuesta"));
    let proposal_type = meta
        .get("type")
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "feature".to_string());
    let created_at = extract_created_at(&meta, Path::new(&path));

    Ok(ProposalDetail {
        proposal: Proposal {
            name,
            proposal_type,
            created_at,
            status: "active".to_string(),
            path,
            summary: build_proposal_summary(&body),
        },
        content: body,
    })
}

#[tauri::command]
pub fn save_proposal(input: SaveProposalInput, state: State<'_, AppState>) -> Result<Proposal, String> {
    let root = active_project_root(&state)?;
    let actives_dir = root.join("openspec").join("propose").join("actives");
    fs::create_dir_all(&actives_dir).map_err(|e| format!("No se pudo crear actives: {e}"))?;

    let now = now_iso_like();
    let created_at = if let Some(existing_path) = &input.path {
        let raw = fs::read_to_string(existing_path)
            .map_err(|e| format!("No se pudo leer propuesta existente {existing_path}: {e}"))?;
        let (meta, _) = parse_markdown_with_frontmatter(&raw);
        meta.get("createdAt")
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .unwrap_or_else(|| now.clone())
    } else {
        now.clone()
    };

    let target = match input.path {
        Some(existing) => PathBuf::from(existing),
        None => {
            let slug = slugify(&input.name);
            let file_name = format!("{slug}.md");
            unique_file_path(&actives_dir, &file_name)
        }
    };

    let normalized_type = normalize_proposal_type(&input.proposal_type);
    let markdown = format!(
        "---\nname: {name}\ntype: {typ}\ncreatedAt: {created}\n---\n\n{body}\n",
        name = input.name.trim(),
        typ = normalized_type,
        created = created_at,
        body = input.content.trim()
    );
    fs::write(&target, markdown).map_err(|e| format!("No se pudo guardar propuesta {}: {e}", target.display()))?;

    Ok(Proposal {
        name: input.name.trim().to_string(),
        proposal_type: normalized_type,
        created_at,
        status: "active".to_string(),
        path: target.to_string_lossy().to_string(),
        summary: build_proposal_summary(input.content.trim()),
    })
}

#[tauri::command]
pub fn archive_proposals(paths: Vec<String>, state: State<'_, AppState>) -> Result<(), String> {
    if paths.is_empty() {
        return Ok(());
    }

    let root = active_project_root(&state)?;
    let archived_dir = root.join("openspec").join("propose").join("archived");
    fs::create_dir_all(&archived_dir).map_err(|e| format!("No se pudo crear archived: {e}"))?;

    for src in paths {
        let src_path = PathBuf::from(&src);
        if !src_path.exists() {
            continue;
        }
        let file_name = src_path
            .file_name()
            .ok_or_else(|| format!("Ruta inválida de propuesta: {src}"))?;
        let dest = unique_file_path(&archived_dir, &file_name.to_string_lossy());
        fs::rename(&src_path, &dest).map_err(|e| {
            format!(
                "No se pudo archivar propuesta {} -> {}: {e}",
                src_path.display(),
                dest.display()
            )
        })?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_proposals(paths: Vec<String>, _state: State<'_, AppState>) -> Result<(), String> {
    if paths.is_empty() {
        return Ok(());
    }

    for src in paths {
        let src_path = PathBuf::from(&src);
        if !src_path.exists() {
            continue;
        }
        if !src_path.is_file() {
            continue;
        }
        fs::remove_file(&src_path)
            .map_err(|e| format!("No se pudo eliminar propuesta {}: {e}", src_path.display()))?;
    }

    Ok(())
}

#[tauri::command]
pub fn copy_to_clipboard(text: String) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("No se pudo abrir portapapeles: {e}"))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("No se pudo copiar al portapapeles: {e}"))
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err("documento aún no creado".to_string());
    }
    fs::read_to_string(p).map_err(|e| format!("No se pudo leer {path}: {e}"))
}

#[tauri::command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("No se pudo escribir {path}: {e}"))
}

#[tauri::command]
pub fn list_spec_documents(specs_dir: String) -> Result<Vec<SpecDocument>, String> {
    let root = PathBuf::from(&specs_dir);
    if !root.exists() || !root.is_dir() {
        return Ok(Vec::new());
    }

    let mut docs = Vec::new();
    let entries = fs::read_dir(&root).map_err(|e| format!("No se pudo leer {}: {e}", root.display()))?;
    for entry in entries.flatten() {
        let cap_dir = entry.path();
        if !cap_dir.is_dir() {
            continue;
        }
        let spec_path = cap_dir.join("spec.md");
        if !spec_path.exists() || !spec_path.is_file() {
            continue;
        }
        let capability = cap_dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "spec".to_string());
        docs.push(SpecDocument {
            name: format!("{capability}/spec.md"),
            path: spec_path.to_string_lossy().to_string(),
        });
    }

    docs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(docs)
}

#[tauri::command]
pub fn create_spec_document(specs_dir: String, capability: String) -> Result<String, String> {
    let slug = slugify(&capability);
    if slug.trim().is_empty() {
        return Err("Nombre de capability inválido".to_string());
    }

    let root = PathBuf::from(&specs_dir);
    fs::create_dir_all(&root).map_err(|e| format!("No se pudo crear {}: {e}", root.display()))?;
    let cap_dir = root.join(&slug);
    fs::create_dir_all(&cap_dir).map_err(|e| format!("No se pudo crear {}: {e}", cap_dir.display()))?;
    let spec_file = cap_dir.join("spec.md");

    if !spec_file.exists() {
        let template = "## ADDED Requirements\n\n### Requirement: Nueva capacidad\nLa aplicación SHALL describir el comportamiento esperado.\n\n#### Scenario: Caso base\n- **WHEN** ocurre la condición\n- **THEN** el sistema responde como se espera\n";
        fs::write(&spec_file, template)
            .map_err(|e| format!("No se pudo crear {}: {e}", spec_file.display()))?;
    }

    Ok(spec_file.to_string_lossy().to_string())
}

#[tauri::command]
pub fn check_openspec_cli() -> Result<bool, String> {
    let output = Command::new("which")
        .arg("openspec")
        .output()
        .map_err(|e| format!("No se pudo ejecutar verificación de CLI: {e}"))?;

    Ok(output.status.success())
}

#[tauri::command]
pub fn get_versions() -> Result<VersionInfo, String> {
    let app_version = env!("CARGO_PKG_VERSION").to_string();

    let openspec_version = match Command::new("openspec").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                "No disponible".to_string()
            } else {
                stdout
            }
        }
        _ => "No disponible".to_string(),
    };

    Ok(VersionInfo {
        app_version,
        openspec_version,
    })
}

#[tauri::command]
pub fn init_project(input: InitProjectInput, app: AppHandle, state: State<'_, AppState>) -> Result<ProjectState, String> {
    if input.name.trim().is_empty() || input.ai_provider.trim().is_empty() {
        return Err("Nombre y proveedor IA son obligatorios".to_string());
    }

    if !check_openspec_cli()? {
        return Err("OpenSpec CLI no instalada o fuera del PATH".to_string());
    }

    let openspec_dir = Path::new(&input.path).join("openspec");
    fs::create_dir_all(&openspec_dir)
        .map_err(|e| format!("No se pudo crear openspec/: {e}"))?;

    let config = InitConfig {
        schema: "spec-driven".to_string(),
        ai_provider: input.ai_provider.clone(),
        contexto: build_context(&input),
        architecture: normalize_opt(input.architecture),
        deployment_flow: normalize_opt(input.deployment_flow),
    };

    let yaml = serde_yaml::to_string(&config)
        .map_err(|e| format!("No se pudo serializar config.yaml: {e}"))?;
    fs::write(openspec_dir.join("config.yaml"), yaml)
        .map_err(|e| format!("No se pudo escribir config.yaml: {e}"))?;

    let output = Command::new("openspec")
        .arg("init")
        .arg("--tools")
        .arg(tools_for_provider(&input.ai_provider))
        .arg(&input.path)
        .output()
        .map_err(|e| format!("No se pudo ejecutar openspec init: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        return Err(format!("openspec init falló\nstdout:\n{stdout}\nstderr:\n{stderr}"));
    }

    let project = load_project(&input.path)?;
    upsert_project(&app, &state, &input.path, project.clone())?;
    Ok(project)
}

fn build_context(input: &InitProjectInput) -> String {
    let mut lines = vec![
        format!("Proyecto: {}", input.name),
        format!("Idioma: {}", input.language.clone().unwrap_or_else(|| "Español".to_string())),
        format!("Audiencia: {}", input.audience.clone().unwrap_or_else(|| "Equipo".to_string())),
        format!("Dominio: {}", input.domain.clone().unwrap_or_else(|| "General".to_string())),
        format!("Descripción: {}", input.description.clone().unwrap_or_else(|| "Proyecto OpenSpec".to_string())),
        format!("Stack: {}", input.stack),
    ];

    if let Some(a) = normalize_opt(input.architecture.clone()) {
        lines.push(format!("Architecture: {a}"));
    }
    if let Some(d) = normalize_opt(input.deployment_flow.clone()) {
        lines.push(format!("Deployment flow: {d}"));
    }

    lines.join("\n")
}

fn normalize_opt(value: Option<String>) -> Option<String> {
    value.and_then(|v| {
        let trimmed = v.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn tools_for_provider(provider: &str) -> &'static str {
    match provider.trim().to_ascii_lowercase().as_str() {
        "codex" => "codex",
        "copilot" => "github-copilot",
        "opencode" => "opencode",
        _ => "codex",
    }
}

fn upsert_project(app: &AppHandle, state: &State<'_, AppState>, path: &str, project: ProjectState) -> Result<(), String> {
    {
        let mut projects = state
            .projects
            .lock()
            .map_err(|_| "No se pudo bloquear projects".to_string())?;

        if let Some(idx) = projects.iter().position(|p| p.path == path) {
            projects[idx].state = project;
            let mut active = state
                .active_index
                .lock()
                .map_err(|_| "No se pudo bloquear active_index".to_string())?;
            *active = Some(idx);
        } else {
            projects.push(ProjectHandle {
                path: path.to_string(),
                name: path_name(path),
                state: project,
            });
            let mut active = state
                .active_index
                .lock()
                .map_err(|_| "No se pudo bloquear active_index".to_string())?;
            *active = Some(projects.len() - 1);
        }
    }

    register_all_watchers(app, state)?;
    save_app_state(app, state)?;
    Ok(())
}

fn register_all_watchers(app: &AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
    let projects = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?
        .clone();

    let mut new_watchers = Vec::new();

    for handle in projects {
        let app_handle = app.clone();
        let target_path = handle.path.clone();

        let mut watcher = recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let changed = event
                    .paths
                    .iter()
                    .any(|p| p.to_string_lossy().contains("openspec"));
                if changed {
                    let _ = app_handle.emit("project-updated", target_path.clone());
                }
            }
        })
        .map_err(|e| format!("No se pudo crear watcher: {e}"))?;

        watcher
            .watch(Path::new(&handle.state.openspec_path), RecursiveMode::Recursive)
            .map_err(|e| format!("No se pudo observar {}: {e}", handle.state.openspec_path))?;

        new_watchers.push(watcher);
    }

    let mut watchers = state
        .watchers
        .lock()
        .map_err(|_| "No se pudo bloquear watchers".to_string())?;
    *watchers = new_watchers;

    Ok(())
}

fn path_name(path: &str) -> String {
    PathBuf::from(path)
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string())
}

fn app_state_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("No se pudo resolver app_data_dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("No se pudo crear app_data_dir: {e}"))?;
    Ok(dir.join("app-state.json"))
}

fn load_app_state(app: &AppHandle) -> Result<PersistedAppState, String> {
    let path = app_state_path(app)?;
    if !path.exists() {
        return Ok(PersistedAppState::default());
    }

    let raw = fs::read_to_string(&path)
        .map_err(|e| format!("No se pudo leer app-state.json: {e}"))?;
    serde_json::from_str::<PersistedAppState>(&raw)
        .map_err(|e| format!("app-state.json inválido: {e}"))
}

fn save_app_state(app: &AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
    let projects = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?;
    let active_index = *state
        .active_index
        .lock()
        .map_err(|_| "No se pudo bloquear active_index".to_string())?;

    let persisted = PersistedAppState {
        project_paths: projects.iter().map(|p| p.path.clone()).collect(),
        active_index,
    };

    let data = serde_json::to_string_pretty(&persisted)
        .map_err(|e| format!("No se pudo serializar app-state.json: {e}"))?;
    fs::write(app_state_path(app)?, data)
        .map_err(|e| format!("No se pudo guardar app-state.json: {e}"))?;
    Ok(())
}

fn active_project_root(state: &State<'_, AppState>) -> Result<PathBuf, String> {
    let projects = state
        .projects
        .lock()
        .map_err(|_| "No se pudo bloquear projects".to_string())?;
    let active = *state
        .active_index
        .lock()
        .map_err(|_| "No se pudo bloquear active_index".to_string())?;
    let idx = active.ok_or_else(|| "No hay proyecto activo".to_string())?;
    let project = projects
        .get(idx)
        .ok_or_else(|| "Índice de proyecto activo inválido".to_string())?;
    Ok(PathBuf::from(&project.path))
}

fn read_proposals_from_dir(dir: &Path, status: &str) -> Result<Vec<Proposal>, String> {
    let mut proposals = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("No se pudo leer {}: {e}", dir.display()))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let raw = fs::read_to_string(&path).map_err(|e| format!("No se pudo leer {}: {e}", path.display()))?;
        let (meta, _) = parse_markdown_with_frontmatter(&raw);
        let (_, body) = parse_markdown_with_frontmatter(&raw);
        proposals.push(Proposal {
            name: meta
                .get("name")
                .and_then(Value::as_str)
                .map(|s| s.to_string())
                .unwrap_or_else(|| file_stem_or_default(&path, "propuesta")),
            proposal_type: normalize_proposal_type(
                meta.get("type").and_then(Value::as_str).unwrap_or("feature"),
            ),
            created_at: extract_created_at(&meta, &path),
            status: status.to_string(),
            path: path.to_string_lossy().to_string(),
            summary: build_proposal_summary(&body),
        });
    }

    proposals.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(proposals)
}

fn parse_markdown_with_frontmatter(raw: &str) -> (serde_yaml::Mapping, String) {
    if !raw.starts_with("---\n") {
        return (serde_yaml::Mapping::new(), raw.to_string());
    }

    let remainder = &raw[4..];
    if let Some(end) = remainder.find("\n---\n") {
        let yaml_part = &remainder[..end];
        let body = &remainder[end + 5..];
        let mapping = serde_yaml::from_str::<serde_yaml::Mapping>(yaml_part).unwrap_or_default();
        return (mapping, body.to_string());
    }

    (serde_yaml::Mapping::new(), raw.to_string())
}

fn extract_created_at(meta: &serde_yaml::Mapping, path: &Path) -> String {
    for key in ["createdAt", "created_at", "date", "fecha"] {
        if let Some(value) = meta
            .get(key)
            .and_then(Value::as_str)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
        {
            return value;
        }
    }

    fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|| "0".to_string())
}

fn file_stem_or_default(path: &Path, fallback: &str) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn normalize_proposal_type(value: &str) -> String {
    match value.trim().to_ascii_lowercase().as_str() {
        "bug" => "bug".to_string(),
        _ => "feature".to_string(),
    }
}

fn build_proposal_summary(content: &str) -> String {
    let mut out = String::new();
    let mut in_frontmatter = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
            in_frontmatter = !in_frontmatter;
            continue;
        }
        if in_frontmatter {
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("```") {
            continue;
        }
        let cleaned = trimmed
            .replace("**", "")
            .replace('*', "")
            .replace('`', "")
            .replace('[', "")
            .replace(']', "")
            .replace("(", "")
            .replace(")", "");
        if cleaned.trim().is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(cleaned.trim());
        if out.len() >= 220 {
            break;
        }
    }
    out.trim().to_string()
}

fn slugify(name: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in name.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let out = out.trim_matches('-').to_string();
    if out.is_empty() {
        "propuesta".to_string()
    } else {
        out
    }
}

fn unique_file_path(dir: &Path, base_name: &str) -> PathBuf {
    let candidate = dir.join(base_name);
    if !candidate.exists() {
        return candidate;
    }

    let stem = Path::new(base_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("propuesta");
    let ext = Path::new(base_name).extension().and_then(|e| e.to_str()).unwrap_or("md");
    for i in 2..10000 {
        let file_name = format!("{stem}-{i}.{ext}");
        let next = dir.join(file_name);
        if !next.exists() {
            return next;
        }
    }
    dir.join(format!("{stem}-{}.{}", now_unix(), ext))
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn now_iso_like() -> String {
    // Keep a stable, sortable UTC-like timestamp without extra deps.
    format!("{}", now_unix())
}
