use tauri::State;

use crate::bettercap::client::BettercapClient;
use crate::bettercap::process;
use crate::config::BettercapConfig;
use crate::AppState;

#[tauri::command]
pub async fn start_bettercap(
    state: State<'_, AppState>,
    config: BettercapConfig,
) -> Result<String, String> {
    // Kill existing process if running
    {
        let mut proc = state.process.lock().await;
        if let Some(ref mut child) = *proc {
            let _ = process::kill_bettercap(child);
        }
        *proc = None;
    }

    // Save config
    config.save()?;

    // Spawn bettercap
    let child = process::spawn_bettercap(&config)?;

    {
        let mut proc = state.process.lock().await;
        *proc = Some(child);
    }

    // Wait briefly for bettercap API to become ready
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Recreate client with new config
    let new_client =
        BettercapClient::new(&config.api_host, config.api_port, &config.username, &config.password);
    {
        let mut client = state.client.lock().await;
        *client = new_client;
    }

    Ok("bettercap started".into())
}

#[tauri::command]
pub async fn stop_bettercap(state: State<'_, AppState>) -> Result<String, String> {
    let mut proc = state.process.lock().await;
    match proc.as_mut() {
        Some(child) => {
            process::kill_bettercap(child)?;
            *proc = None;
            Ok("bettercap stopped".into())
        }
        None => Err("bettercap is not running".into()),
    }
}

#[tauri::command]
pub async fn get_bettercap_status(state: State<'_, AppState>) -> Result<bool, String> {
    let mut proc = state.process.lock().await;
    match proc.as_mut() {
        Some(child) => match child.try_wait() {
            Ok(Some(_)) => {
                *proc = None;
                Ok(false)
            }
            Ok(None) => Ok(true),
            Err(_) => {
                *proc = None;
                Ok(false)
            }
        },
        None => Ok(false),
    }
}

#[tauri::command]
pub async fn load_config() -> Result<BettercapConfig, String> {
    Ok(BettercapConfig::load())
}
