use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub schema: Option<String>,
    pub contexto: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub present: bool,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub complete: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeStatus {
    Pending,
    Ready,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub name: String,
    pub archived: bool,
    pub status: ChangeStatus,
    pub artifacts: Vec<Artifact>,
    pub tasks: Option<TaskProgress>,
    pub why_summary: String,
    pub archived_at: Option<String>,
    pub archived_documents: Vec<ChangeDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeDocument {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub root_path: String,
    pub openspec_path: String,
    pub config: ProjectConfig,
    pub active_changes: Vec<Change>,
    pub archived_changes: Vec<Change>,
    pub specs: Vec<String>,
}

pub type ProjectState = Project;
