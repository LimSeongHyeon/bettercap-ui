import { useEffect, useState, useCallback, useRef } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

export interface BettercapEvent {
  tag: string;
  time: string;
  data: Record<string, unknown>;
}

export interface Host {
  ipv4: string;
  mac: string;
  hostname: string;
  vendor: string;
}

export function useBettercapEvents(maxEvents = 100) {
  const [events, setEvents] = useState<BettercapEvent[]>([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const unlisteners: UnlistenFn[] = [];

    listen<boolean>("bettercap-connected", (e) => {
      setConnected(e.payload);
    }).then((fn) => unlisteners.push(fn));

    listen<BettercapEvent>("bettercap-event", (e) => {
      setEvents((prev) => {
        const next = [e.payload, ...prev];
        return next.slice(0, maxEvents);
      });
    }).then((fn) => unlisteners.push(fn));

    return () => {
      unlisteners.forEach((fn) => fn());
    };
  }, [maxEvents]);

  const clearEvents = useCallback(() => setEvents([]), []);

  return { events, connected, clearEvents };
}

export function usePollingHosts(fetchFn: () => Promise<unknown>, intervalMs = 3000) {
  const [hosts, setHosts] = useState<Host[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const activeRef = useRef(false);

  const poll = useCallback(async () => {
    if (!activeRef.current) return;
    setLoading(true);
    try {
      const result = (await fetchFn()) as { lan?: { hosts?: Host[] } };
      setHosts(result?.lan?.hosts ?? []);
      setError(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [fetchFn]);

  const start = useCallback(() => {
    activeRef.current = true;
    poll();
    const id = setInterval(poll, intervalMs);
    return () => {
      activeRef.current = false;
      clearInterval(id);
    };
  }, [poll, intervalMs]);

  return { hosts, loading, error, start };
}
