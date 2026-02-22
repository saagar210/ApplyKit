import { useEffect, useMemo, useState } from "react";
import { BulletPicker } from "../components/BulletPicker";
import { DiffViewer } from "../components/DiffViewer";
import { FitScoreCard } from "../components/FitScoreCard";
import { GapList } from "../components/GapList";
import { MarkdownViewer } from "../components/MarkdownViewer";
import type { AppTab, PacketDetail } from "../lib/types";

interface Props {
  detail?: PacketDetail;
  approvedOnly: boolean;
  onCopy: (text: string) => Promise<void>;
  onOpenFolder: (path: string) => Promise<void>;
  onExportMarkdown: (packetDir: string) => Promise<void>;
  onExportDocx: (packetDir: string) => Promise<void>;
  onExportPdf: (packetDir: string) => Promise<void>;
  onUpdateTracker: (status: string, nextAction: string, notes: string) => Promise<void>;
}

const tabs: AppTab[] = ["overview", "resume", "messages", "export", "tracker"];

export function JobReview({
  detail,
  approvedOnly,
  onCopy,
  onOpenFolder,
  onExportMarkdown,
  onExportDocx,
  onExportPdf,
  onUpdateTracker
}: Props) {
  const [tab, setTab] = useState<AppTab>("overview");
  const [status, setStatus] = useState(detail?.trackerRow.status ?? "new");
  const [nextAction, setNextAction] = useState(detail?.trackerRow.nextAction ?? "");
  const [notes, setNotes] = useState("");

  useEffect(() => {
    setStatus(detail?.trackerRow.status ?? "new");
    setNextAction(detail?.trackerRow.nextAction ?? "");
    setNotes("");
  }, [detail?.packetDir, detail?.trackerRow.status, detail?.trackerRow.nextAction]);

  const packetDir = detail?.packetDir;
  const trackerDirty = useMemo(() => {
    return status !== (detail?.trackerRow.status ?? "new") || nextAction !== (detail?.trackerRow.nextAction ?? "") || notes.length > 0;
  }, [status, nextAction, notes, detail]);

  return (
    <section className="stack-lg">
      <header className="card row between">
        <h2>Job Review</h2>
        <div className="row wrap">
          {packetDir ? (
            <button className="btn" onClick={() => onOpenFolder(packetDir)}>
              Open Folder
            </button>
          ) : null}
          <button className="btn" onClick={() => onCopy(packetDir ?? "")}>Copy Packet Path (⌘⇧C)</button>
          {packetDir ? <code>{packetDir}</code> : null}
        </div>
      </header>

      <div className="tabs">
        {tabs.map((item) => (
          <button className={`tab ${tab === item ? "active" : ""}`} key={item} onClick={() => setTab(item)}>
            {item}
          </button>
        ))}
      </div>

      {!detail ? <section className="card"><p className="subtle">No packet selected yet.</p></section> : null}

      {tab === "overview" && detail ? (
        <>
          <FitScoreCard fit={detail.fitBreakdown} track={detail.track} />
          <GapList gaps={detail.fitBreakdown.gaps} />
          <section className="card">
            <h3>Extracted Signals</h3>
            <p>
              <strong>Extraction source:</strong> {detail.extractionSource ?? "deterministic"}
            </p>
            {detail.extractionDiagnostics?.summarizeFallbackReasons?.length ? (
              <p className="subtle">
                summarize_jd fallback: {detail.extractionDiagnostics.summarizeFallbackReasons.join(", ")}
              </p>
            ) : null}
            <p>
              <strong>Keywords:</strong> {detail.extractedKeywords.join(", ") || "-"}
            </p>
            <p>
              <strong>Tools:</strong> {detail.extractedTools.join(", ") || "-"}
            </p>
            <p>
              <strong>Requirements:</strong> {detail.extractedRequirements.join(" | ") || "-"}
            </p>
          </section>
        </>
      ) : null}

      {tab === "resume" && detail ? (
        <>
          <section className="card">
            <h3>Tailor Plan</h3>
            <ol>
              {detail.tailorPlan.edits.map((edit, idx) => (
                <li key={`${edit.kind}-${idx}`}>
                  <strong>{edit.kind}</strong> [{edit.targetSection}] {edit.reason}
                </li>
              ))}
            </ol>
          </section>
          <BulletPicker bullets={detail.bulletCandidates} approvedOnly={approvedOnly} />
          <DiffViewer diff={detail.diff} />
          <section className="card">
            <h3>Resume Preview</h3>
            <MarkdownViewer markdown={detail.resume1pg} />
          </section>
        </>
      ) : null}

      {tab === "messages" && detail ? (
        <section className="card stack-sm">
          <h3>Messages</h3>
          <div className="row between">
            <strong>Recruiter</strong>
            <button className="btn" onClick={() => onCopy(detail.messages.recruiter)}>
              Copy
            </button>
          </div>
          <MarkdownViewer markdown={detail.messages.recruiter} />

          <div className="row between">
            <strong>Hiring Manager</strong>
            <button className="btn" onClick={() => onCopy(detail.messages.hiringManager)}>
              Copy
            </button>
          </div>
          <MarkdownViewer markdown={detail.messages.hiringManager} />

          <div className="row between">
            <strong>Cover Short</strong>
            <button className="btn" onClick={() => onCopy(detail.messages.coverShort)}>
              Copy
            </button>
          </div>
          <MarkdownViewer markdown={detail.messages.coverShort} />
        </section>
      ) : null}

      {tab === "export" && detail ? (
        <section className="card stack-sm">
          <h3>Export</h3>
          <div className="row wrap">
            <button className="btn" onClick={() => onExportMarkdown(detail.packetDir)}>
              Export Markdown Bundle
            </button>
            <button className="btn" onClick={() => onExportDocx(detail.packetDir)}>
              Export DOCX
            </button>
            <button
              className="btn"
              onClick={() => onExportPdf(detail.packetDir)}
            >
              Export PDF
            </button>
          </div>
          <p className="subtle">PDF export is deterministic and uses the same canonical section ordering as DOCX.</p>
          <section>
            <h4>Truth Report</h4>
            <p>Passed: {detail.truthReport.passed ? "yes" : "no"}</p>
            {detail.truthReport.violations.length ? (
              <ul>
                {detail.truthReport.violations.map((v) => (
                  <li key={v}>{v}</li>
                ))}
              </ul>
            ) : (
              <p className="subtle">No violations.</p>
            )}
          </section>
        </section>
      ) : null}

      {tab === "tracker" && detail ? (
        <section className="card stack-sm">
          <h3>Tracker</h3>
          <label>
            Status
            <select value={status} onChange={(e) => setStatus(e.target.value)}>
              <option value="new">new</option>
              <option value="applied">applied</option>
              <option value="reply">reply</option>
              <option value="interview">interview</option>
              <option value="closed">closed</option>
            </select>
          </label>
          <label>
            Next action
            <input value={nextAction} onChange={(e) => setNextAction(e.target.value)} placeholder="Follow up on Monday" />
          </label>
          <label>
            Notes
            <textarea value={notes} onChange={(e) => setNotes(e.target.value)} rows={4} placeholder="Interview prep notes" />
          </label>
          <div className="row end">
            <button
              className="btn btn-primary"
              disabled={!trackerDirty}
              onClick={async () => {
                try {
                  await onUpdateTracker(status, nextAction, notes);
                  setNotes("");
                } catch {
                  // parent handler already surfaces user-facing error feedback
                }
              }}
            >
              Save Tracker
            </button>
          </div>
        </section>
      ) : null}
    </section>
  );
}
