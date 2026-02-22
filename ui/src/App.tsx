import { useEffect, useState } from "react";
import { AppShell } from "./components/AppShell";
import { CommandPalette } from "./components/CommandPalette";
import { DiffViewer } from "./components/DiffViewer";
import { MarkdownViewer } from "./components/MarkdownViewer";
import { ToastHost, type Toast } from "./components/ToastHost";
import { invokeSafe } from "./lib/tauri";
import type {
  ExportResponse,
  GenerateRequest,
  GenerateResponse,
  JobSummary,
  PacketDetail,
  SettingsModel,
  UpdateJobStatusResponse
} from "./lib/types";
import { Dashboard } from "./screens/Dashboard";
import { Banks } from "./screens/Banks";
import { JobReview } from "./screens/JobReview";
import { NewJob } from "./screens/NewJob";
import { Templates } from "./screens/Templates";

type View = "dashboard" | "new-job" | "job-review" | "banks" | "templates" | "settings";

function uid() {
  return crypto.randomUUID();
}

const defaultSettings: SettingsModel = {
  allowUnapproved: false,
  llmEnabled: true,
  llmProvider: "ollama",
  llmBaseUrl: "http://127.0.0.1:11434",
  llmModel: "llama3.2",
  llmAllowedTasks: ["rewrite_message", "rewrite_bullet", "summarize_jd"]
};

export default function App() {
  const [view, setView] = useState<View>("dashboard");
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [busy, setBusy] = useState(false);
  const [jobs, setJobs] = useState<JobSummary[]>([]);
  const [insights, setInsights] = useState<{
    repliesByTrack: [string, number][];
    commonGaps: [string, number][];
    keywordCorrelations: [string, number][];
  } | null>(null);
  const [selectedDetail, setSelectedDetail] = useState<PacketDetail | undefined>(undefined);
  const [selectedJobId, setSelectedJobId] = useState<string | undefined>(undefined);
  const [settings, setSettings] = useState<SettingsModel>(defaultSettings);
  const [showPreview, setShowPreview] = useState(true);
  const [previewMode, setPreviewMode] = useState<"preview" | "diff">("preview");
  const [toasts, setToasts] = useState<Toast[]>([]);
  const llmTasks: Array<"rewrite_message" | "rewrite_bullet" | "summarize_jd"> = [
    "rewrite_message",
    "rewrite_bullet",
    "summarize_jd"
  ];

  const pushToast = (message: string, tone: Toast["tone"]) => {
    const toast = { id: uid(), message, tone };
    setToasts((prev) => [...prev, toast]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== toast.id));
    }, 3000);
  };

  const loadJobs = async () => {
    try {
      const response = await invokeSafe<{ jobs: JobSummary[] }>("list_jobs_cmd", {});
      setJobs(response.jobs ?? []);
    } catch {
      setJobs([]);
    }
  };

  const loadInsights = async () => {
    try {
      const response = await invokeSafe<{
        repliesByTrack: [string, number][];
        commonGaps: [string, number][];
        keywordCorrelations: [string, number][];
      }>("insights_cmd", {});
      setInsights(response);
    } catch {
      setInsights(null);
    }
  };

  const loadSettings = async () => {
    try {
      const response = await invokeSafe<SettingsModel>("get_settings_cmd", {});
      setSettings(response);
    } catch {
      setSettings(defaultSettings);
    }
  };

  const openJobById = async (jobId: string) => {
    try {
      const detail = await invokeSafe<PacketDetail>("get_packet_detail_cmd", {
        input: { jobId }
      });
      setSelectedDetail(detail);
      setSelectedJobId(jobId);
      setView("job-review");
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to open job detail";
      pushToast(message, "error");
    }
  };

  useEffect(() => {
    void loadJobs();
    void loadInsights();
    void loadSettings();
  }, []);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setPaletteOpen((prev) => !prev);
      }
      if ((event.metaKey || event.ctrlKey) && event.key === "Enter" && view === "new-job") {
        event.preventDefault();
        const button = document.querySelector<HTMLButtonElement>(".btn-primary");
        button?.click();
      }
      if (
        (event.metaKey || event.ctrlKey) &&
        event.shiftKey &&
        event.key.toLowerCase() === "c" &&
        view === "job-review"
      ) {
        event.preventDefault();
        void navigator.clipboard.writeText(selectedDetail?.packetDir ?? "");
        pushToast("Copied packet path", "success");
      }
    };

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [selectedDetail?.packetDir, view]);

  const sidebar = (
    <div className="stack-lg">
      <h1>ApplyKit</h1>
      <nav className="stack-sm">
        {[
          ["dashboard", "Jobs"],
          ["new-job", "New Job"],
          ["job-review", "Review"],
          ["banks", "Banks"],
          ["templates", "Templates"],
          ["settings", "Settings"]
        ].map(([id, label]) => (
          <button
            key={id}
            className={`nav-btn ${view === id ? "active" : ""}`}
            onClick={() => setView(id as View)}
          >
            {label}
          </button>
        ))}
      </nav>
      <div className="card subtle-card">
        <strong>Shortcuts</strong>
        <p>⌘K Command Palette</p>
        <p>⌘Enter Generate</p>
        <p>⌘⇧C Copy</p>
      </div>
    </div>
  );

  const main = (() => {
    if (view === "dashboard") {
      return (
        <Dashboard
          jobs={jobs}
          onNewJob={() => setView("new-job")}
          onOpenJob={(jobId) => void openJobById(jobId)}
          insights={{
            repliesByTrack: insights?.repliesByTrack ?? [],
            commonGaps: insights?.commonGaps ?? [],
            keywordCorrelations: insights?.keywordCorrelations ?? []
          }}
        />
      );
    }

    if (view === "new-job") {
      return (
        <NewJob
          busy={busy}
          onGenerate={async (req: GenerateRequest) => {
            setBusy(true);
            try {
              const response = await invokeSafe<GenerateResponse>("generate_packet_cmd", {
                input: {
                  company: req.company,
                  role: req.role,
                  source: req.source,
                  baseline: req.baseline,
                  jdText: req.jdText,
                  allowUnapproved: settings.allowUnapproved
                }
              });
              setSelectedDetail(response.packetDetail);
              setSelectedJobId(undefined);
              setView("job-review");
              pushToast("Packet generated", "success");
              await loadJobs();
              await loadInsights();
            } catch (err) {
              const message = err instanceof Error ? err.message : "Generation failed";
              pushToast(message, "error");
            } finally {
              setBusy(false);
            }
          }}
        />
      );
    }

    if (view === "job-review") {
      return (
        <JobReview
          detail={selectedDetail}
          approvedOnly={!settings.allowUnapproved}
          onCopy={async (text) => {
            await navigator.clipboard.writeText(text);
            pushToast("Copied", "info");
          }}
          onOpenFolder={async (path) => {
            try {
              await invokeSafe<void>("open_output_folder", { path });
            } catch (err) {
              const message = err instanceof Error ? err.message : "Open folder failed";
              pushToast(message, "error");
            }
          }}
          onExportMarkdown={async (packetDir) => {
            const response = await invokeSafe<ExportResponse>("export_markdown_cmd", {
              input: { packetDir }
            });
            pushToast(response.message, response.ok ? "success" : "error");
          }}
          onExportDocx={async (packetDir) => {
            const response = await invokeSafe<ExportResponse>("export_docx_cmd", {
              input: { packetDir }
            });
            pushToast(response.message, response.ok ? "success" : "error");
          }}
          onExportPdf={async (packetDir) => {
            const response = await invokeSafe<ExportResponse>("export_pdf_cmd", {
              input: { packetDir }
            });
            pushToast(response.message, response.ok ? "success" : "error");
          }}
          onUpdateTracker={async (status, nextAction, notes) => {
            if (!selectedDetail) {
              return;
            }
            try {
              const jobId = selectedJobId ?? jobs.find((j) => j.outputDir === selectedDetail.packetDir)?.id;
              if (!jobId) {
                pushToast("Could not resolve job id for tracker update", "error");
                return;
              }

              const result = await invokeSafe<UpdateJobStatusResponse>("update_job_status_cmd", {
                input: {
                  id: jobId,
                  status,
                  nextAction,
                  notes
                }
              });
              if (!result.ok) {
                throw new Error("Tracker update failed");
              }

              pushToast("Tracker updated", "success");
              await loadJobs();
              await loadInsights();
              await openJobById(jobId);
            } catch (err) {
              const message = err instanceof Error ? err.message : "Tracker update failed";
              pushToast(message, "error");
              throw err instanceof Error ? err : new Error(message);
            }
          }}
        />
      );
    }

    if (view === "banks") {
      return <Banks onNotify={(message, tone) => pushToast(message, tone)} />;
    }

    if (view === "templates") {
      return <Templates onNotify={(message, tone) => pushToast(message, tone)} />;
    }

    return (
      <section className="card stack-sm">
        <h2>Settings</h2>
        <label>
          <input
            type="checkbox"
            checked={settings.allowUnapproved}
            onChange={(e) => setSettings((prev) => ({ ...prev, allowUnapproved: e.target.checked }))}
          />
          Allow unapproved bullets (default off)
        </label>
        <label>
          <input
            type="checkbox"
            checked={settings.llmEnabled}
            onChange={(e) => setSettings((prev) => ({ ...prev, llmEnabled: e.target.checked }))}
          />
          Enable local LLM rewrite
        </label>
        <label>
          LLM provider
          <input
            value={settings.llmProvider}
            onChange={(e) => setSettings((prev) => ({ ...prev, llmProvider: e.target.value }))}
          />
        </label>
        <label>
          LLM base URL
          <input
            value={settings.llmBaseUrl}
            onChange={(e) => setSettings((prev) => ({ ...prev, llmBaseUrl: e.target.value }))}
          />
        </label>
        <label>
          LLM model
          <input
            value={settings.llmModel}
            onChange={(e) => setSettings((prev) => ({ ...prev, llmModel: e.target.value }))}
          />
        </label>
        <fieldset className="stack-sm">
          <legend>Allowed LLM tasks</legend>
          {llmTasks.map((task) => (
            <label key={task}>
              <input
                type="checkbox"
                checked={settings.llmAllowedTasks.includes(task)}
                onChange={(e) => {
                  setSettings((prev) => {
                    const next = e.target.checked
                      ? [...prev.llmAllowedTasks, task]
                      : prev.llmAllowedTasks.filter((t) => t !== task);
                    return {
                      ...prev,
                      llmAllowedTasks: Array.from(new Set(next)).sort()
                    };
                  });
                }}
              />
              {task}
            </label>
          ))}
        </fieldset>
        <div className="row end">
          <button
            className="btn btn-primary"
            onClick={async () => {
              try {
                const saved = await invokeSafe<SettingsModel>("save_settings_cmd", {
                  input: {
                    allowUnapproved: settings.allowUnapproved,
                    llmEnabled: settings.llmEnabled,
                    llmProvider: settings.llmProvider,
                    llmBaseUrl: settings.llmBaseUrl,
                    llmModel: settings.llmModel,
                    llmAllowedTasks: settings.llmAllowedTasks
                  }
                });
                setSettings(saved);
                pushToast("Settings saved", "success");
              } catch (err) {
                const message = err instanceof Error ? err.message : "Failed to save settings";
                pushToast(message, "error");
              }
            }}
          >
            Save Settings
          </button>
        </div>
      </section>
    );
  })();

  const preview = (
    <div className="stack-lg">
      <section className="card">
        <h3>Preview</h3>
        <p className="subtle">
          {selectedDetail
            ? `Packet: ${selectedDetail.packetDir}`
            : "Generate or open a packet to preview tailored artifacts."}
        </p>
      </section>
      {selectedDetail ? (
        <>
          <section className="card stack-sm">
            <h4>Extraction Source</h4>
            <span className="chip">{selectedDetail.extractionSource ?? "deterministic"}</span>
            {selectedDetail.extractionDiagnostics?.summarizeFallbackReasons?.length ? (
              <p className="subtle">
                summarize_jd fallback:{" "}
                {selectedDetail.extractionDiagnostics.summarizeFallbackReasons.join(", ")}
              </p>
            ) : null}
          </section>
          <section className="card stack-sm">
            <h4>Resume Preview</h4>
            <MarkdownViewer markdown={selectedDetail.resume1pg} />
          </section>
        </>
      ) : (
        <section className="card code-preview">
          <pre>No packet selected yet.</pre>
        </section>
      )}
    </div>
  );

  const diffPreview = (
    <div className="stack-lg">
      <section className="card">
        <h3>Diff</h3>
        <p className="subtle">Inline and side-by-side comparison for the tailored resume.</p>
      </section>
      {selectedDetail ? (
        <DiffViewer diff={selectedDetail.diff} />
      ) : (
        <section className="card code-preview">
          <pre>No diff available yet.</pre>
        </section>
      )}
    </div>
  );

  return (
    <>
      <AppShell
        sidebar={sidebar}
        main={main}
        preview={preview}
        diffPreview={diffPreview}
        showPreview={showPreview}
        previewMode={previewMode}
        onPreviewModeChange={setPreviewMode}
        onTogglePreview={() => setShowPreview((prev) => !prev)}
      />
      <CommandPalette open={paletteOpen} onOpenChange={setPaletteOpen} onNavigate={(v) => setView(v as View)} />
      <ToastHost toasts={toasts} />
    </>
  );
}
