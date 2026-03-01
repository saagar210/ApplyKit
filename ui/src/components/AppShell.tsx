import * as Tabs from "@radix-ui/react-tabs";
import { type KeyboardEvent, type ReactNode } from "react";

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

const modeOrder: PreviewMode[] = ["preview", "diff"];

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
  const onPreviewKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") {
      return;
    }

    event.preventDefault();
    const currentIndex = modeOrder.indexOf(previewMode);
    const delta = event.key === "ArrowRight" ? 1 : -1;
    const nextIndex = (currentIndex + delta + modeOrder.length) % modeOrder.length;
    onPreviewModeChange(modeOrder[nextIndex]);
  };

  return (
    <div className={`app-shell ${showPreview ? "with-preview" : "without-preview"}`}>
      <aside className="sidebar" aria-label="Primary navigation">
        {sidebar}
      </aside>
      <main id="main-content" className="main-pane" tabIndex={-1}>
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
        <section className="preview-pane" aria-label="Preview panel">
          <Tabs.Root
            className="stack-lg"
            value={previewMode}
            onValueChange={(value) => onPreviewModeChange(value as PreviewMode)}
          >
            <section className="card row between preview-controls">
              <Tabs.List
                aria-label="Preview mode"
                className="preview-tablist"
                onKeyDown={onPreviewKeyDown}
              >
                <Tabs.Trigger className="btn preview-trigger" value="preview">
                  Preview
                </Tabs.Trigger>
                <Tabs.Trigger className="btn preview-trigger" value="diff">
                  Diff
                </Tabs.Trigger>
              </Tabs.List>
              <button className="btn" onClick={onTogglePreview}>
                Hide Pane
              </button>
            </section>
            <Tabs.Content value="preview">{preview}</Tabs.Content>
            <Tabs.Content value="diff">{diffPreview}</Tabs.Content>
          </Tabs.Root>
        </section>
      ) : null}
    </div>
  );
}
