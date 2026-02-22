import { useMemo, useState } from "react";
import type { JobSummary } from "../lib/types";

interface Props {
  jobs: JobSummary[];
  onNewJob: () => void;
  onOpenJob: (jobId: string) => void;
  insights?: {
    repliesByTrack: [string, number][];
    commonGaps: [string, number][];
    keywordCorrelations: [string, number][];
  };
}

export function Dashboard({ jobs, onNewJob, onOpenJob, insights }: Props) {
  const [daysFilter, setDaysFilter] = useState<"all" | "7" | "30">("all");
  const [trackFilter, setTrackFilter] = useState("all");
  const [statusFilter, setStatusFilter] = useState("all");
  const [search, setSearch] = useState("");

  const filteredJobs = useMemo(() => {
    let out = jobs.slice();

    if (daysFilter !== "all") {
      const days = Number(daysFilter);
      const threshold = Date.now() - days * 24 * 60 * 60 * 1000;
      out = out.filter((job) => new Date(job.updatedAt).getTime() >= threshold);
    }

    if (trackFilter !== "all") {
      out = out.filter((job) => (job.track ?? "").toLowerCase() === trackFilter.toLowerCase());
    }

    if (statusFilter !== "all") {
      out = out.filter((job) => job.status.toLowerCase() === statusFilter.toLowerCase());
    }

    const query = search.trim().toLowerCase();
    if (query) {
      out = out.filter((job) => {
        const haystack = [job.company, job.role, job.track ?? "", job.source, job.status]
          .join(" ")
          .toLowerCase();
        return haystack.includes(query);
      });
    }

    return out;
  }, [jobs, daysFilter, trackFilter, statusFilter, search]);

  const trackOptions = useMemo(() => {
    return Array.from(new Set(jobs.map((job) => job.track).filter(Boolean) as string[])).sort((a, b) =>
      a.localeCompare(b)
    );
  }, [jobs]);

  const statusOptions = useMemo(() => {
    return Array.from(new Set(jobs.map((job) => job.status).filter(Boolean))).sort((a, b) =>
      a.localeCompare(b)
    );
  }, [jobs]);

  return (
    <section className="stack-lg">
      <div className="card hero">
        <div>
          <h2>ApplyKit Dashboard</h2>
          <p className="subtle">Fewer, better applications with deterministic outputs.</p>
        </div>
        <button className="btn btn-primary" onClick={onNewJob}>
          New Job
        </button>
      </div>

      <section className="card">
        <div className="row between wrap">
          <h3>Recent Packets</h3>
          <div className="row wrap">
            <input
              aria-label="Search jobs"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search company, role, track, source"
            />
            <label className="row">
              Window
              <select
                value={daysFilter}
                onChange={(e) => setDaysFilter(e.target.value as "all" | "7" | "30")}
              >
                <option value="all">All</option>
                <option value="7">Last 7 days</option>
                <option value="30">Last 30 days</option>
              </select>
            </label>
            <label className="row">
              Track
              <select value={trackFilter} onChange={(e) => setTrackFilter(e.target.value)}>
                <option value="all">All tracks</option>
                {trackOptions.map((track) => (
                  <option value={track} key={track}>
                    {track}
                  </option>
                ))}
              </select>
            </label>
            <label className="row">
              Status
              <select value={statusFilter} onChange={(e) => setStatusFilter(e.target.value)}>
                <option value="all">All status</option>
                {statusOptions.map((status) => (
                  <option value={status} key={status}>
                    {status}
                  </option>
                ))}
              </select>
            </label>
          </div>
        </div>
        <table className="table">
          <thead>
            <tr>
              <th>Company</th>
              <th>Role</th>
              <th>Track</th>
              <th>Status</th>
              <th>Fit</th>
              <th>Updated</th>
            </tr>
          </thead>
          <tbody>
            {filteredJobs.length === 0 ? (
              <tr>
                <td colSpan={6}>No jobs yet. Generate your first packet.</td>
              </tr>
            ) : (
              filteredJobs.map((job) => (
                <tr key={job.id} className="clickable-row" onClick={() => onOpenJob(job.id)}>
                  <td>{job.company}</td>
                  <td>{job.role}</td>
                  <td>{job.track ?? "-"}</td>
                  <td>{job.status}</td>
                  <td>{job.fitTotal ?? "-"}</td>
                  <td>{new Date(job.updatedAt).toLocaleString()}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </section>

      <section className="card">
        <h3>Insights</h3>
        <p className="subtle">Replies by track, common gaps, and keyword correlations.</p>
        <div className="insights-grid">
          <div>
            <strong>Replies by track</strong>
            {(insights?.repliesByTrack ?? []).length === 0 ? (
              <p className="subtle">No reply data yet.</p>
            ) : (
              <ul>
                {(insights?.repliesByTrack ?? []).map(([track, count]) => (
                  <li key={track}>
                    {track}: {count}
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div>
            <strong>Common gaps</strong>
            {(insights?.commonGaps ?? []).length === 0 ? (
              <p className="subtle">No gap trends yet.</p>
            ) : (
              <ul>
                {(insights?.commonGaps ?? []).slice(0, 5).map(([gap, count]) => (
                  <li key={gap}>
                    {gap}: {count}
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div>
            <strong>Keyword correlations</strong>
            {(insights?.keywordCorrelations ?? []).length === 0 ? (
              <p className="subtle">No correlation data yet.</p>
            ) : (
              <ul>
                {(insights?.keywordCorrelations ?? []).slice(0, 5).map(([keyword, count]) => (
                  <li key={keyword}>
                    {keyword}: {count}
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>
      </section>
    </section>
  );
}
