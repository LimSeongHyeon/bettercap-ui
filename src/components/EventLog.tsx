import type { BettercapEvent } from "../hooks/useBettercap";

interface Props {
  events: BettercapEvent[];
  onClear: () => void;
}

export default function EventLog({ events, onClear }: Props) {
  return (
    <div className="event-log">
      <div className="event-log-header">
        <h2>Events ({events.length})</h2>
        <button onClick={onClear} disabled={events.length === 0}>
          Clear
        </button>
      </div>
      <div className="event-list">
        {events.map((ev, i) => (
          <div key={i} className="event-item">
            <span className="event-tag">{ev.tag}</span>
            <span className="event-time">{ev.time}</span>
            <pre className="event-data">{JSON.stringify(ev.data, null, 2)}</pre>
          </div>
        ))}
        {events.length === 0 && <div className="empty">No events yet.</div>}
      </div>
    </div>
  );
}
