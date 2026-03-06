use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BettercapClient {
    client: Client,
    base_url: String,
    auth_header: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub ipv4: String,
    pub mac: String,
    pub hostname: String,
    pub vendor: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(default)]
    pub interfaces: serde_json::Value,
    #[serde(default)]
    pub modules: serde_json::Value,
    #[serde(default)]
    pub lan: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl BettercapClient {
    pub fn new(host: &str, port: u16, username: &str, password: &str) -> Self {
        let credentials = STANDARD.encode(format!("{username}:{password}"));
        Self {
            client: Client::new(),
            base_url: format!("http://{host}:{port}/api"),
            auth_header: format!("Basic {credentials}"),
        }
    }

    pub async fn run_command(&self, cmd: &str) -> Result<serde_json::Value, String> {
        let resp = self
            .client
            .post(format!("{}/session", self.base_url))
            .header("Authorization", &self.auth_header)
            .json(&serde_json::json!({ "cmd": cmd }))
            .send()
            .await
            .map_err(|e| format!("Connection failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP {}", resp.status()));
        }

        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    }

    pub async fn get_session(&self) -> Result<Session, String> {
        let resp = self
            .client
            .get(format!("{}/session", self.base_url))
            .header("Authorization", &self.auth_header)
            .send()
            .await
            .map_err(|e| format!("Connection failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP {}", resp.status()));
        }

        resp.json::<Session>()
            .await
            .map_err(|e| format!("Parse error: {e}"))
    }

    pub fn websocket_url(&self) -> String {
        self.base_url.replace("http://", "ws://") + "/events"
    }

    pub fn auth_header(&self) -> &str {
        &self.auth_header
    }
}
