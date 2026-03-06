use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn start_scan(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state.client.run_command("net.probe on").await
}

#[tauri::command]
pub async fn stop_scan(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state.client.run_command("net.probe off").await
}

#[tauri::command]
pub async fn get_hosts(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state.client.run_command("net.show").await
}
