use crate::types::SpecSyncConfig;
use std::fs;
use std::path::Path;

/// Load specsync.json from the project root, falling back to defaults.
pub fn load_config(root: &Path) -> SpecSyncConfig {
    let config_path = root.join("specsync.json");

    if !config_path.exists() {
        return SpecSyncConfig::default();
    }

    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return SpecSyncConfig::default(),
    };

    match serde_json::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Warning: failed to parse specsync.json: {e}");
            SpecSyncConfig::default()
        }
    }
}

/// Default schema pattern for SQL table extraction.
pub fn default_schema_pattern() -> &'static str {
    r"CREATE (?:VIRTUAL )?TABLE(?:\s+IF NOT EXISTS)?\s+(\w+)"
}
