import type { Host } from "../hooks/useBettercap";

interface Props {
  hosts: Host[];
  loading: boolean;
  error: string | null;
}

export default function HostList({ hosts, loading, error }: Props) {
  return (
    <div className="host-list">
      <h2>
        Hosts {hosts.length > 0 && <span>({hosts.length})</span>}
        {loading && <span className="spinner" />}
      </h2>
      {error && <div className="status-error">{error}</div>}
      <table>
        <thead>
          <tr>
            <th>IP</th>
            <th>MAC</th>
            <th>Hostname</th>
            <th>Vendor</th>
          </tr>
        </thead>
        <tbody>
          {hosts.map((h) => (
            <tr key={h.mac}>
              <td>{h.ipv4}</td>
              <td>{h.mac}</td>
              <td>{h.hostname || "-"}</td>
              <td>{h.vendor || "-"}</td>
            </tr>
          ))}
          {hosts.length === 0 && (
            <tr>
              <td colSpan={4} className="empty">
                {loading ? "Scanning..." : "No hosts found. Start a scan."}
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}
