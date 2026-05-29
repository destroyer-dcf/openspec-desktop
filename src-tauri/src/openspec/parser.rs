use crate::openspec::model::ProjectConfig;
use std::fs;
use std::path::Path;

pub fn parse_project_config(path: &Path) -> Result<ProjectConfig, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("No se pudo leer {}: {}", path.display(), e))?;

    serde_yaml::from_str::<ProjectConfig>(&raw)
        .map_err(|e| format!("`config.yaml` malformado: {}", e))
}
