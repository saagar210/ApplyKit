import { useMemo, useState } from "react";
import type { BulletCandidate } from "../lib/types";

interface Props {
  bullets: BulletCandidate[];
  approvedOnly: boolean;
}

export function BulletPicker({ bullets, approvedOnly }: Props) {
  const [query, setQuery] = useState("");
  const [track, setTrack] = useState("all");

  const filtered = useMemo(() => {
    return bullets.filter((b) => {
      if (approvedOnly && !b.approved) {
        return false;
      }
      const haystack = `${b.text} ${b.tags.join(" ")} ${b.reason}`.toLowerCase();
      const queryOk = haystack.includes(query.toLowerCase());
      const trackOk = track === "all" || b.trackHint === track;
      return queryOk && trackOk;
    });
  }, [bullets, query, track, approvedOnly]);

  return (
    <section className="card">
      <div className="row between">
        <h3>Bullet Picker</h3>
        <select value={track} onChange={(e) => setTrack(e.target.value)}>
          <option value="all">All Tracks</option>
          <option value="support">Support/Ops</option>
          <option value="identity">Identity/Endpoint</option>
          <option value="security">Security/Compliance</option>
          <option value="automation">Automation/AIOps</option>
          <option value="managerish">Manager-ish</option>
          <option value="general">General</option>
        </select>
      </div>
      <input
        aria-label="Search bullets"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Filter by text, tags, or reason"
      />
      <div className="stack-sm">
        {filtered.length === 0 ? <p className="subtle">No matching bullets.</p> : null}
        {filtered.map((bullet) => (
          <article key={bullet.id} className="bullet-item">
            <p>{bullet.text}</p>
            <div className="chips">
              <span className="chip">score:{bullet.score}</span>
              <span className="chip">{bullet.trackHint}</span>
              {bullet.tags.map((tag) => (
                <span className="chip" key={`${bullet.id}-${tag}`}>
                  {tag}
                </span>
              ))}
              <span className="chip chip-reason">{bullet.reason}</span>
              {!bullet.approved ? <span className="chip chip-danger">unapproved</span> : null}
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
