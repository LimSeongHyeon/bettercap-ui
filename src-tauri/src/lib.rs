mod bettercap;
mod commands;
mod config;

use std::process::Child;

use bettercap::client::BettercapClient;
use config::BettercapConfig;
use tokio::sync::Mutex;

pub struct AppState {
    pub client: Mutex<BettercapClient>,
    pub process: Mutex<Option<Child>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cfg = BettercapConfig::load();
    let client = BettercapClient::new(&cfg.api_host, cfg.api_port, &cfg.username, &cfg.password);
    let event_client = client.clone();

    tauri::Builder::default()
        .manage(AppState {
            client: Mutex::new(client),
            process: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::session::get_session,
            commands::session::run_command,
            commands::scan::start_scan,
            commands::scan::stop_scan,
            commands::scan::get_hosts,
            commands::attack::start_arp_spoof,
            commands::attack::stop_arp_spoof,
            commands::process::start_bettercap,
            commands::process::stop_bettercap,
            commands::process::get_bettercap_status,
            commands::process::load_config,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tokio::spawn(bettercap::events::listen_events(handle, event_client));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
