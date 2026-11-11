//! Official BlocRA endpoint and signing keys, from `defaults.json` (built in at
//! compile time). The BlocRA website reads the same file, so both trust the
//! same keys.

use serde_json::Value;

const DEFAULTS_JSON: &str = include_str!("../defaults.json");

fn defaults() -> Value {
    serde_json::from_str(DEFAULTS_JSON).expect("defaults.json is valid JSON")
}

/// The official API, if this build has one.
pub fn default_api() -> Option<String> {
    defaults().get("api").and_then(Value::as_str).map(String::from)
}

/// Every key BlocRA has signed reports with, newest first.
pub fn official_keys() -> Vec<String> {
    defaults()
        .get("official_keys")
        .and_then(Value::as_array)
        .map(|keys| keys.iter().filter_map(Value::as_str).map(String::from).collect())
        .unwrap_or_default()
}
