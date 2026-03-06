import { useState, useEffect } from "react";
import {
  loadConfig,
  startBettercap,
  stopBettercap,
  getBettercapStatus,
} from "../lib/commands";

export interface BettercapConfig {
  iface: string;
  api_host: string;
  api_port: number;
  username: string;
  password: string;
}

interface Props {
  onStatusChange?: (running: boolean) => void;
}

export default function Settings({ onStatusChange }: Props) {
  const [config, setConfig] = useState<BettercapConfig>({
    iface: "wlan0",
    api_host: "127.0.0.1",
    api_port: 8081,
    username: "admin",
    password: "admin",
  });
  const [running, setRunning] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadConfig().then((cfg) => setConfig(cfg));
    getBettercapStatus().then((status) => {
      setRunning(status);
      onStatusChange?.(status);
    });
  }, [onStatusChange]);

  const handleStart = async () => {
    setLoading(true);
    setError(null);
    try {
      await startBettercap(config);
      setRunning(true);
      onStatusChange?.(true);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleStop = async () => {
    setLoading(true);
    setError(null);
    try {
      await stopBettercap();
      setRunning(false);
      onStatusChange?.(false);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const update = (field: keyof BettercapConfig, value: string | number) => {
    setConfig((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <div className="settings">
      <h2>Bettercap Settings</h2>
      <div className="settings-grid">
        <label>
          Interface
          <input
            value={config.iface}
            onChange={(e) => update("iface", e.target.value)}
            placeholder="wlan0"
            disabled={running}
          />
        </label>
        <label>
          API Host
          <input
            value={config.api_host}
            onChange={(e) => update("api_host", e.target.value)}
            placeholder="127.0.0.1"
            disabled={running}
          />
        </label>
        <label>
          API Port
          <input
            type="number"
            value={config.api_port}
            onChange={(e) => update("api_port", parseInt(e.target.value) || 8081)}
            disabled={running}
          />
        </label>
        <label>
          Username
          <input
            value={config.username}
            onChange={(e) => update("username", e.target.value)}
            disabled={running}
          />
        </label>
        <label>
          Password
          <input
            type="password"
            value={config.password}
            onChange={(e) => update("password", e.target.value)}
            disabled={running}
          />
        </label>
      </div>
      <div className="settings-actions">
        {running ? (
          <button className="btn-danger" onClick={handleStop} disabled={loading}>
            {loading ? "Stopping..." : "Stop Bettercap"}
          </button>
        ) : (
          <button className="btn-primary" onClick={handleStart} disabled={loading}>
            {loading ? "Starting..." : "Start Bettercap"}
          </button>
        )}
        <span className={`process-status ${running ? "running" : "stopped"}`}>
          {running ? "Running" : "Stopped"}
        </span>
      </div>
      {error && <div className="status-error">{error}</div>}
    </div>
  );
}
