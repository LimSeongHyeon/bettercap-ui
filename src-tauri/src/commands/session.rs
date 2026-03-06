use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn get_session(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let session = state.client.get_session().await?;
    serde_json::to_value(session).map_err(|e| format!("Serialize error: {e}"))
}

#[tauri::command]
pub async fn run_command(
    state: State<'_, AppState>,
    cmd: String,
) -> Result<serde_json::Value, String> {
    state.client.run_command(&cmd).await
}
