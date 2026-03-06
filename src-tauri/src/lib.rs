mod bettercap;
mod commands;

use bettercap::client::BettercapClient;

pub struct AppState {
    pub client: BettercapClient,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = BettercapClient::new("127.0.0.1", 8081, "admin", "admin");
    let event_client = client.clone();

    tauri::Builder::default()
        .manage(AppState { client })
        .invoke_handler(tauri::generate_handler![
            commands::session::get_session,
            commands::session::run_command,
            commands::scan::start_scan,
            commands::scan::stop_scan,
            commands::scan::get_hosts,
            commands::attack::start_arp_spoof,
            commands::attack::stop_arp_spoof,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tokio::spawn(bettercap::events::listen_events(handle, event_client));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
