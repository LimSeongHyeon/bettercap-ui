import { useState } from "react";
import { startScan, stopScan } from "../lib/commands";

interface Props {
  connected: boolean;
  onScanToggle?: (scanning: boolean) => void;
}

export default function ScanControl({ connected, onScanToggle }: Props) {
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const toggle = async () => {
    setError(null);
    try {
      if (scanning) {
        await stopScan();
      } else {
        await startScan();
      }
      const next = !scanning;
      setScanning(next);
      onScanToggle?.(next);
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div className="scan-control">
      <h2>Network Scan</h2>
      <button onClick={toggle} disabled={!connected}>
        {scanning ? "Stop Scan" : "Start Scan"}
      </button>
      {!connected && <span className="status-warn">Disconnected from bettercap</span>}
      {error && <span className="status-error">{error}</span>}
    </div>
  );
}
