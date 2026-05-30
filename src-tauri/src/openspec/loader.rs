use crate::openspec::model::{Artifact, Change, ChangeDocument, ChangeStatus, Project, TaskProgress};
use crate::openspec::parser::parse_project_config;
use crate::openspec::scanner::find_openspec_path;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_project(root: &str) -> Result<Project, String> {
    let openspec_path = find_openspec_path(root)?;
    let config = parse_project_config(&openspec_path.join("config.yaml")).unwrap_or_default();

    let changes_dir = openspec_path.join("changes");
    let archive_dir = changes_dir.join("archive");

    let active_changes = read_changes(&changes_dir, false)?;
    let archived_changes = if archive_dir.exists() {
        read_changes(&archive_dir, true)?
    } else {
        Vec::new()
    };

    let specs = list_specs(&openspec_path.join("specs"))?;

    Ok(Project {
        root_path: root.to_string(),
        openspec_path: openspec_path.to_string_lossy().to_string(),
        config,
        active_changes,
        archived_changes,
        specs,
    })
}

fn list_specs(specs_dir: &Path) -> Result<Vec<String>, String> {
    if !specs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut specs = Vec::new();
    let entries = fs::read_dir(specs_dir)
        .map_err(|e| format!("No se pudo leer {}: {}", specs_dir.display(), e))?;

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            specs.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    specs.sort();
    Ok(specs)
}

fn read_changes(base_dir: &Path, archived: bool) -> Result<Vec<Change>, String> {
    if !base_dir.exists() {
        return Ok(Vec::new());
    }

    let mut changes = Vec::new();
    let entries = fs::read_dir(base_dir)
        .map_err(|e| format!("No se pudo leer {}: {}", base_dir.display(), e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if !archived && entry.file_name() == "archive" {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        changes.push(read_change(path, name, archived)?);
    }

    changes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(changes)
}

fn read_change(path: PathBuf, name: String, archived: bool) -> Result<Change, String> {
    let proposal = artifact(&path, "proposal", "proposal.md");
    let design = artifact(&path, "design", "design.md");
    let tasks = artifact(&path, "tasks", "tasks.md");
    let tasks_progress = tasks
        .path
        .as_ref()
        .and_then(|p| parse_task_progress(Path::new(p)).ok().flatten());
    let specs_path = path.join("specs");
    let specs_present = has_spec_documents(&specs_path);
    let specs_dir_exists = specs_path.exists() && specs_path.is_dir();

    let mut artifacts = vec![proposal, design, tasks];
    artifacts.push(Artifact {
        name: "specs".to_string(),
        present: specs_present,
        path: if specs_dir_exists {
            Some(specs_path.to_string_lossy().to_string())
        } else {
            None
        },
    });

    let status = if artifacts.iter().all(|a| a.present) {
        ChangeStatus::Ready
    } else if artifacts.iter().any(|a| a.present) {
        ChangeStatus::Pending
    } else {
        ChangeStatus::Blocked
    };

    let archived_at = if archived {
        fs::metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs().to_string())
    } else {
        None
    };

    Ok(Change {
        name,
        archived,
        status,
        artifacts,
        tasks: tasks_progress,
        why_summary: read_why_summary(&path),
        archived_at,
        archived_documents: if archived {
            collect_archived_documents(&path)?
        } else {
            Vec::new()
        },
    })
}

fn read_why_summary(change_dir: &Path) -> String {
    let proposal = change_dir.join("proposal.md");
    let Ok(raw) = fs::read_to_string(&proposal) else {
        return "Sin resumen".to_string();
    };
    extract_first_h2_section(&raw).unwrap_or_else(|| "Sin resumen".to_string())
}

fn extract_first_h2_section(raw: &str) -> Option<String> {
    let lines: Vec<&str> = raw.lines().collect();
    let mut start_idx: Option<usize> = None;
    for (idx, line) in lines.iter().enumerate() {
        if line.trim().starts_with("## ") {
            start_idx = Some(idx + 1);
            break;
        }
    }
    let start = start_idx?;
    let mut out = String::new();
    for line in lines.iter().skip(start) {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(trimmed);
    }
    let cleaned = out.trim();
    if cleaned.is_empty() {
        None
    } else {
        let truncated: String = cleaned.chars().take(220).collect();
        Some(truncated.trim_end().to_string())
    }
}

fn has_spec_documents(specs_path: &Path) -> bool {
    if !specs_path.exists() || !specs_path.is_dir() {
        return false;
    }
    let Ok(entries) = fs::read_dir(specs_path) else {
        return false;
    };
    for entry in entries.flatten() {
        let cap_dir = entry.path();
        if !cap_dir.is_dir() {
            continue;
        }
        let spec_file = cap_dir.join("spec.md");
        if spec_file.exists() && spec_file.is_file() {
            return true;
        }
    }
    false
}

fn collect_archived_documents(change_dir: &Path) -> Result<Vec<ChangeDocument>, String> {
    let mut docs = Vec::new();

    for file in ["proposal.md", "design.md", "tasks.md"] {
        let file_path = change_dir.join(file);
        if file_path.exists() && file_path.is_file() {
            docs.push(ChangeDocument {
                name: file.to_string(),
                path: file_path.to_string_lossy().to_string(),
            });
        }
    }

    let specs_dir = change_dir.join("specs");
    if specs_dir.exists() && specs_dir.is_dir() {
        collect_markdown_files(&specs_dir, &specs_dir, &mut docs)?;
    }

    docs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(docs)
}

fn collect_markdown_files(root: &Path, dir: &Path, docs: &mut Vec<ChangeDocument>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("No se pudo leer {}: {}", dir.display(), e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(root, &path, docs)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let name = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        docs.push(ChangeDocument {
            name,
            path: path.to_string_lossy().to_string(),
        });
    }

    Ok(())
}

fn artifact(base: &Path, name: &str, file: &str) -> Artifact {
    let path = base.join(file);
    let present = path.exists();
    Artifact {
        name: name.to_string(),
        present,
        path: if present {
            Some(path.to_string_lossy().to_string())
        } else {
            None
        },
    }
}

fn parse_task_progress(path: &Path) -> Result<Option<TaskProgress>, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("No se pudo leer tasks.md {}: {}", path.display(), e))?;

    let mut total = 0usize;
    let mut complete = 0usize;

    for line in raw.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("- [ ]") {
            total += 1;
        } else if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            total += 1;
            complete += 1;
        }
    }

    if total == 0 {
        Ok(None)
    } else {
        Ok(Some(TaskProgress { complete, total }))
    }
}
