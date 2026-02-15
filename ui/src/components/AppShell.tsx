import type { ReactNode } from "react";

type PreviewMode = "preview" | "diff";

interface Props {
  sidebar: ReactNode;
  main: ReactNode;
  preview: ReactNode;
  diffPreview: ReactNode;
  showPreview: boolean;
  previewMode: PreviewMode;
  onPreviewModeChange: (mode: PreviewMode) => void;
  onTogglePreview: () => void;
}

export function AppShell({
  sidebar,
  main,
  preview,
  diffPreview,
  showPreview,
  previewMode,
  onPreviewModeChange,
  onTogglePreview
}: Props) {
  return (
    <div className={`app-shell ${showPreview ? "with-preview" : "without-preview"}`}>
      <aside className="sidebar">{sidebar}</aside>
      <main className="main-pane">
        {!showPreview ? (
          <section className="card row end">
            <button className="btn" onClick={onTogglePreview}>
              Show Preview Pane
            </button>
          </section>
        ) : null}
        {main}
      </main>
      {showPreview ? (
        <section className="preview-pane">
          <section className="card row between preview-controls">
            <div className="row">
              <button
                className={`btn ${previewMode === "preview" ? "btn-primary" : ""}`}
                onClick={() => onPreviewModeChange("preview")}
              >
                Preview
              </button>
              <button
                className={`btn ${previewMode === "diff" ? "btn-primary" : ""}`}
                onClick={() => onPreviewModeChange("diff")}
              >
                Diff
              </button>
            </div>
            <button className="btn" onClick={onTogglePreview}>
              Hide Pane
            </button>
          </section>
          {previewMode === "preview" ? preview : diffPreview}
        </section>
      ) : null}
    </div>
  );
}
