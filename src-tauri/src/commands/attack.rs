use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn start_arp_spoof(
    state: State<'_, AppState>,
    targets: String,
) -> Result<serde_json::Value, String> {
    state
        .client
        .run_command(&format!("set arp.spoof.targets {targets}"))
        .await?;
    state.client.run_command("arp.spoof on").await
}

#[tauri::command]
pub async fn stop_arp_spoof(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state.client.run_command("arp.spoof off").await
}
