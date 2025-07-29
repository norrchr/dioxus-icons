use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct IconConfig {
    pub(crate) resource_path: Option<PathBuf>,
    pub(crate) resources: Vec<IconResource>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct IconResource {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) rev: Option<String>,
    pub(crate) srcs: Vec<String>
}

pub (crate) fn read_config(path: Option<PathBuf>) -> Result<IconConfig, String> {
    let config_path = path.unwrap_or(PathBuf::from("config.json"));

    let config = std::fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let config: IconConfig = serde_json::from_str(&config).map_err(|e| e.to_string())?;

    Ok(config)
}