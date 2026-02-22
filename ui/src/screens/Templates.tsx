import { useCallback, useEffect, useMemo, useState } from "react";
import { MarkdownViewer } from "../components/MarkdownViewer";
import { invokeSafe } from "../lib/tauri";
import type { MutationResponse, TemplateKey } from "../lib/types";

interface Props {
  onNotify: (message: string, tone: "info" | "success" | "error") => void;
}

interface TemplatesPreviewResponse {
  resume1pgBase: string;
  resume2pgBase: string;
  recruiterTemplate: string;
  hiringManagerTemplate: string;
  coverShortTemplate: string;
}

const templateOptions: Array<{ key: TemplateKey; label: string }> = [
  { key: "resume_1pg_base", label: "Resume 1pg Base" },
  { key: "resume_2pg_base", label: "Resume 2pg Base" },
  { key: "recruiter", label: "Recruiter Message" },
  { key: "hiring_manager", label: "Hiring Manager Message" },
  { key: "cover_short", label: "Cover Short Message" }
];

export function Templates({ onNotify }: Props) {
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [preview, setPreview] = useState<TemplatesPreviewResponse | null>(null);
  const [selected, setSelected] = useState<TemplateKey>("resume_1pg_base");
  const [drafts, setDrafts] = useState<Partial<Record<TemplateKey, string>>>({});
  const [validationError, setValidationError] = useState<string | null>(null);

  const load = useCallback(async () => {
    const response = await invokeSafe<TemplatesPreviewResponse>("get_templates_preview_cmd", {});
    setPreview(response);
    setError(null);
  }, []);

  useEffect(() => {
    let mounted = true;
    void (async () => {
      try {
        await load();
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err.message : "Failed to load template previews");
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    })();
    return () => {
      mounted = false;
    };
  }, [load]);

  const templateMap = useMemo(() => {
    if (!preview) {
      return null;
    }
    return {
      resume_1pg_base: preview.resume1pgBase,
      resume_2pg_base: preview.resume2pgBase,
      recruiter: preview.recruiterTemplate,
      hiring_manager: preview.hiringManagerTemplate,
      cover_short: preview.coverShortTemplate
    } satisfies Record<TemplateKey, string>;
  }, [preview]);

  const current = drafts[selected] ?? (templateMap ? templateMap[selected] : "");

  const setCurrentDraft = (value: string) => {
    setDrafts((prev) => ({ ...prev, [selected]: value }));
  };

  return (
    <section className="stack-lg">
      <section className="card">
        <h2>Templates Editor</h2>
        <p className="subtle">Edit resume/message templates with anchor and placeholder validation.</p>
      </section>

      {loading ? <section className="card">Loading template preview...</section> : null}
      {error ? <section className="card">{error}</section> : null}

      {templateMap ? (
        <section className="template-grid">
          <section className="card stack-sm">
            <label>
              Template
              <select
                value={selected}
                onChange={(e) => {
                  const key = e.target.value as TemplateKey;
                  setSelected(key);
                  setValidationError(null);
                }}
              >
                {templateOptions.map((option) => (
                  <option key={option.key} value={option.key}>
                    {option.label}
                  </option>
                ))}
              </select>
            </label>

            <textarea
              value={current}
              onChange={(e) => setCurrentDraft(e.target.value)}
              rows={20}
              className="template-editor"
            />

            {validationError ? <p className="error-text">{validationError}</p> : null}

            <div className="row end">
              <button
                className="btn"
                onClick={() => {
                  setDrafts((prev) => ({ ...prev, [selected]: templateMap[selected] }));
                  setValidationError(null);
                }}
              >
                Revert
              </button>
              <button
                className="btn btn-primary"
                disabled={saving}
                onClick={async () => {
                  setSaving(true);
                  setValidationError(null);
                  try {
                    const response = await invokeSafe<MutationResponse>("save_template_cmd", {
                      input: { templateKey: selected, content: current }
                    });
                    onNotify(response.message, response.ok ? "success" : "error");
                    await load();
                  } catch (err) {
                    const message =
                      err instanceof Error ? err.message : "Failed to save template";
                    setValidationError(message);
                    onNotify(message, "error");
                  } finally {
                    setSaving(false);
                  }
                }}
              >
                Save Template
              </button>
            </div>
          </section>

          <section className="card stack-sm">
            <h3>Markdown Preview</h3>
            <MarkdownViewer markdown={current} />
          </section>
        </section>
      ) : null}
    </section>
  );
}
