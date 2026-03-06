use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BettercapConfig {
    pub iface: String,
    pub api_host: String,
    pub api_port: u16,
    pub username: String,
    pub password: String,
}

impl Default for BettercapConfig {
    fn default() -> Self {
        Self {
            iface: "wlan0".into(),
            api_host: "127.0.0.1".into(),
            api_port: 8081,
            username: "admin".into(),
            password: "admin".into(),
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("bettercap-ui")
        .join("config.json")
}

impl BettercapConfig {
    pub fn load() -> Self {
        let path = config_path();
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {e}"))?;
        }
        let json =
            serde_json::to_string_pretty(self).map_err(|e| format!("Serialize error: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("Failed to write config: {e}"))
    }
}
