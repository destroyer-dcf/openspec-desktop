use std::path::{Path, PathBuf};

pub fn find_openspec_path(root: &str) -> Result<PathBuf, String> {
    let root_path = Path::new(root);
    if !root_path.exists() {
        return Err(format!("La ruta no existe: {root}"));
    }

    if !root_path.is_dir() {
        return Err(format!("La ruta no es una carpeta: {root}"));
    }

    let openspec_dir = root_path.join("openspec");
    if !openspec_dir.exists() {
        return Err("La carpeta seleccionada no es un proyecto OpenSpec: falta `openspec/`".to_string());
    }

    let config = openspec_dir.join("config.yaml");
    if !config.exists() {
        return Err("Proyecto OpenSpec inválido: falta `openspec/config.yaml`".to_string());
    }

    Ok(openspec_dir)
}
