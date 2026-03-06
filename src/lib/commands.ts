import { invoke } from "@tauri-apps/api/core";

export async function getSession(): Promise<Record<string, unknown>> {
  return invoke("get_session");
}

export async function runCommand(cmd: string): Promise<Record<string, unknown>> {
  return invoke("run_command", { cmd });
}

export async function startScan(): Promise<Record<string, unknown>> {
  return invoke("start_scan");
}

export async function stopScan(): Promise<Record<string, unknown>> {
  return invoke("stop_scan");
}

export async function getHosts(): Promise<Record<string, unknown>> {
  return invoke("get_hosts");
}

export async function startArpSpoof(targets: string): Promise<Record<string, unknown>> {
  return invoke("start_arp_spoof", { targets });
}

export async function stopArpSpoof(): Promise<Record<string, unknown>> {
  return invoke("stop_arp_spoof");
}
