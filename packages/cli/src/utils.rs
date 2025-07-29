use std::{path::PathBuf, sync::OnceLock};

#[derive(serde::Deserialize)]
struct CargoMetadata {
    workspace_root: PathBuf,
}

static WORKSPACE_ROOT: OnceLock<Result<PathBuf, String>> = OnceLock::new();

pub(crate) fn get_workspace_root() -> Result<PathBuf, String> {
    WORKSPACE_ROOT.get_or_init(|| {
        let output = std::process::Command::new("cargo")
            .arg("metadata")
            .arg("--no-deps")
            .arg("--format-version=1")
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!("cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr)))
        }

        let metadata = serde_json::from_slice::<CargoMetadata>(&output.stdout)
            .map_err(|e| e.to_string())?;

        Ok(metadata.workspace_root)
    }).clone()
}

pub(crate) fn icon_resource_path(resource_path: Option<PathBuf>) -> PathBuf {
    let path = resource_path.unwrap_or_else(|| PathBuf::from("icon_resources"));

    if path.is_absolute() {
        path
    } else {
        let workspace_root = get_workspace_root().expect("Failed to get workspace root");
        workspace_root.join(path)
    }
}

pub(crate) fn icon_out_dir() -> PathBuf {
    let workspace_root = get_workspace_root().expect("Failed to get workspace root");
    workspace_root.join(PathBuf::from("packages/icons/src/icons"))
}