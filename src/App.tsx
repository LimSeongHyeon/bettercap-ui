import { useCallback, useEffect, useRef, useState } from "react";
import Settings from "./components/Settings";
import ScanControl from "./components/ScanControl";
import HostList from "./components/HostList";
import EventLog from "./components/EventLog";
import { useBettercapEvents, usePollingHosts } from "./hooks/useBettercap";
import { getSession } from "./lib/commands";
import "./App.css";

export default function App() {
  const [bettercapRunning, setBettercapRunning] = useState(false);
  const { events, connected, clearEvents } = useBettercapEvents();
  const fetchHosts = useCallback(() => getSession(), []);
  const { hosts, loading, error, start } = usePollingHosts(fetchHosts);
  const cleanupRef = useRef<(() => void) | null>(null);

  const handleScanToggle = (scanning: boolean) => {
    if (scanning) {
      cleanupRef.current = start();
    } else {
      cleanupRef.current?.();
      cleanupRef.current = null;
    }
  };

  useEffect(() => {
    return () => cleanupRef.current?.();
  }, []);

  return (
    <div className="app">
      <header>
        <h1>Bettercap UI</h1>
        <span className={`status ${connected ? "online" : "offline"}`}>
          {connected ? "Connected" : "Disconnected"}
        </span>
      </header>
      <main>
        <Settings onStatusChange={setBettercapRunning} />
        {bettercapRunning && (
          <>
            <ScanControl connected={connected} onScanToggle={handleScanToggle} />
            <HostList hosts={hosts} loading={loading} error={error} />
            <EventLog events={events} onClear={clearEvents} />
          </>
        )}
      </main>
    </div>
  );
}
