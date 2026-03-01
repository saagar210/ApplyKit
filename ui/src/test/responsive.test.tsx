import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { AppShell } from "../components/AppShell";
import { Dashboard } from "../screens/Dashboard";

describe("responsive layout hooks", () => {
  it("provides shell classes for with/without preview layouts", () => {
    const { container, rerender } = render(
      <AppShell
        sidebar={<div>Sidebar</div>}
        main={<div>Main</div>}
        preview={<div>Preview pane</div>}
        diffPreview={<div>Diff pane</div>}
        showPreview
        previewMode="preview"
        onPreviewModeChange={vi.fn()}
        onTogglePreview={vi.fn()}
      />
    );

    expect(container.querySelector(".app-shell.with-preview")).toBeInTheDocument();
    expect(container.querySelector("#main-content")).toBeInTheDocument();

    rerender(
      <AppShell
        sidebar={<div>Sidebar</div>}
        main={<div>Main</div>}
        preview={<div>Preview pane</div>}
        diffPreview={<div>Diff pane</div>}
        showPreview={false}
        previewMode="preview"
        onPreviewModeChange={vi.fn()}
        onTogglePreview={vi.fn()}
      />
    );

    expect(container.querySelector(".app-shell.without-preview")).toBeInTheDocument();
  });

  it("renders dashboard with responsive table wrapper and explicit actions", () => {
    const onOpenJob = vi.fn();
    const { container } = render(
      <Dashboard
        jobs={[
          {
            id: "job-1",
            company: "Acme",
            role: "Support Engineer",
            source: "LinkedIn",
            baseline: "1pg",
            track: "Support/Ops Core",
            fitTotal: 80,
            status: "new",
            updatedAt: new Date().toISOString()
          }
        ]}
        onNewJob={vi.fn()}
        onOpenJob={onOpenJob}
        insights={{ repliesByTrack: [], commonGaps: [], keywordCorrelations: [] }}
      />
    );

    expect(container.querySelector(".recent-packets-card")).toBeInTheDocument();
    expect(container.querySelector(".table-wrapper")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /open packet for acme support engineer/i }));
    expect(onOpenJob).toHaveBeenCalledWith("job-1");
  });
});
