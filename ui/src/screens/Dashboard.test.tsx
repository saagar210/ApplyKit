import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { JobSummary } from "../lib/types";
import { Dashboard } from "./Dashboard";

function mkJob(partial: Partial<JobSummary>): JobSummary {
  return {
    id: partial.id ?? "job-1",
    company: partial.company ?? "Acme",
    role: partial.role ?? "Support Engineer",
    source: partial.source ?? "LinkedIn",
    baseline: partial.baseline ?? "1pg",
    track: partial.track ?? "Support/Ops Core",
    fitTotal: partial.fitTotal ?? 70,
    status: partial.status ?? "new",
    nextAction: partial.nextAction,
    notes: partial.notes,
    outputDir: partial.outputDir,
    updatedAt: partial.updatedAt ?? new Date().toISOString()
  };
}

describe("Dashboard filters", () => {
  it("applies date, track, status, and search filters", () => {
    const jobs: JobSummary[] = [
      mkJob({
        id: "1",
        company: "Acme",
        role: "Support Engineer",
        track: "Support/Ops Core",
        status: "new",
        source: "LinkedIn",
        updatedAt: new Date().toISOString()
      }),
      mkJob({
        id: "2",
        company: "Beta",
        role: "Security Engineer",
        track: "Security & Compliance Ops",
        status: "reply",
        source: "Referral",
        updatedAt: new Date(Date.now() - 40 * 24 * 60 * 60 * 1000).toISOString()
      })
    ];

    render(
      <Dashboard
        jobs={jobs}
        onNewJob={vi.fn()}
        onOpenJob={vi.fn()}
        insights={{ repliesByTrack: [], commonGaps: [], keywordCorrelations: [] }}
      />
    );

    fireEvent.change(screen.getByLabelText("Window"), { target: { value: "30" } });
    fireEvent.change(screen.getByLabelText("Track"), { target: { value: "Support/Ops Core" } });
    fireEvent.change(screen.getByLabelText("Status"), { target: { value: "new" } });
    fireEvent.change(screen.getByLabelText("Search jobs"), { target: { value: "LinkedIn" } });

    expect(screen.getByText("Acme")).toBeInTheDocument();
    expect(screen.queryByText("Beta")).not.toBeInTheDocument();
  });
});
