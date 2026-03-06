use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::{connect_async, tungstenite::http::Request};

use super::client::BettercapClient;

pub async fn listen_events(app: AppHandle, client: BettercapClient) {
    loop {
        match connect_to_ws(&app, &client).await {
            Ok(()) => log::info!("WebSocket closed, reconnecting..."),
            Err(e) => log::error!("WebSocket error: {e}"),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

async fn connect_to_ws(app: &AppHandle, client: &BettercapClient) -> Result<(), String> {
    let request = Request::builder()
        .uri(client.websocket_url())
        .header("Authorization", client.auth_header())
        .header("Host", "localhost")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            tokio_tungstenite::tungstenite::handshake::client::generate_key(),
        )
        .body(())
        .map_err(|e| format!("Request build error: {e}"))?;

    let (ws_stream, _) = connect_async(request)
        .await
        .map_err(|e| format!("WebSocket connect failed: {e}"))?;

    let _ = app.emit("bettercap-connected", true);

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                if let Ok(event) = serde_json::from_str::<serde_json::Value>(&text) {
                    let _ = app.emit("bettercap-event", event);
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => break,
            Err(e) => {
                log::error!("WebSocket read error: {e}");
                break;
            }
            _ => {}
        }
    }

    let _ = app.emit("bettercap-connected", false);
    Ok(())
}
