//! blocra.json: the project's subjects, chain, API and pinned signing key.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use crate::js::stringify;

pub const CONFIG_FILE: &str = "blocra.json";

fn path() -> PathBuf {
    PathBuf::from(CONFIG_FILE)
}

/// The config in the current directory, if there is one.
pub fn load() -> Result<Option<Value>, String> {
    let path = path();
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("Could not read {CONFIG_FILE}: {e}"))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|e| format!("{CONFIG_FILE} is not valid JSON: {e}"))
}

pub fn save(config: &Value) -> Result<PathBuf, String> {
    let path = path();
    fs::write(&path, stringify(config, true) + "\n").map_err(|e| format!("Could not write {CONFIG_FILE}: {e}"))?;
    Ok(path)
}
