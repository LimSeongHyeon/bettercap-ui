use std::process::{Child, Command};

use crate::config::BettercapConfig;

pub fn spawn_bettercap(config: &BettercapConfig) -> Result<Child, String> {
    let eval_cmd = format!(
        "set api.rest.address {addr}; set api.rest.port {port}; set api.rest.username {user}; set api.rest.password {pass}; api.rest on",
        addr = config.api_host,
        port = config.api_port,
        user = config.username,
        pass = config.password,
    );

    Command::new("sudo")
        .args(["bettercap", "-iface", &config.iface, "-eval", &eval_cmd])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start bettercap: {e}"))
}

pub fn kill_bettercap(child: &mut Child) -> Result<(), String> {
    // bettercap runs under sudo, so we need to kill the process group
    let pid = child.id();
    Command::new("sudo")
        .args(["kill", "-TERM", &pid.to_string()])
        .output()
        .map_err(|e| format!("Failed to kill bettercap: {e}"))?;

    child
        .wait()
        .map_err(|e| format!("Failed to wait for bettercap exit: {e}"))?;

    Ok(())
}
