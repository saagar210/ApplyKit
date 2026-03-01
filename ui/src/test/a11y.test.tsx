import { render, screen } from "@testing-library/react";
import { axe } from "jest-axe";
import { describe, expect, it, vi } from "vitest";
import { CommandPalette } from "../components/CommandPalette";
import { Dashboard } from "../screens/Dashboard";
import { JobReview } from "../screens/JobReview";

describe("accessibility", () => {
  it("renders dashboard without critical a11y violations", async () => {
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
            fitTotal: 77,
            status: "new",
            updatedAt: new Date().toISOString()
          }
        ]}
        onNewJob={vi.fn()}
        onOpenJob={vi.fn()}
        insights={{ repliesByTrack: [], commonGaps: [], keywordCorrelations: [] }}
      />
    );

    expect(await axe(container)).toHaveNoViolations();
  });

  it("renders command palette with dialog semantics", async () => {
    render(<CommandPalette open onOpenChange={vi.fn()} onNavigate={vi.fn()} />);

    expect(screen.getByRole("dialog", { name: /command palette/i })).toBeInTheDocument();
    expect(await axe(document.body)).toHaveNoViolations();
  });

  it("renders job review tabs without critical a11y violations", async () => {
    const { container } = render(
      <JobReview
        detail={{
          packetDir: "/tmp/packet-a",
          extractedKeywords: ["support"],
          extractedTools: ["Okta"],
          extractedRequirements: ["Incident response"],
          fitBreakdown: {
            roleMatch: 25,
            stackMatch: 10,
            scaleMatch: 5,
            rigorMatch: 5,
            signalBoost: 5,
            total: 50,
            whyMatch: ["Primary track aligned"],
            gaps: []
          },
          track: "Support/Ops Core",
          trackScores: [["Support/Ops Core", 30, ["support"]]],
          tailorPlan: {
            maxResumeEdits: 3,
            maxBulletSwaps: 2,
            edits: []
          },
          bulletCandidates: [],
          messages: {
            recruiter: "Recruiter message",
            hiringManager: "Hiring manager message",
            coverShort: "Cover short"
          },
          resume1pg: "Resume",
          diff: "# Diff",
          trackerRow: {
            date: "2026-02-14",
            company: "Acme",
            role: "Role",
            source: "manual",
            track: "Support/Ops Core",
            fitTotal: 50,
            status: "new",
            nextAction: "follow up",
            packetDir: "/tmp/packet-a"
          },
          truthReport: {
            passed: true,
            violations: [],
            unknownTools: [],
            claimIssues: [],
            provenanceComplete: true
          }
        }}
        approvedOnly
        onCopy={vi.fn().mockResolvedValue(undefined)}
        onOpenFolder={vi.fn().mockResolvedValue(undefined)}
        onExportMarkdown={vi.fn().mockResolvedValue(undefined)}
        onExportDocx={vi.fn().mockResolvedValue(undefined)}
        onExportPdf={vi.fn().mockResolvedValue(undefined)}
        onUpdateTracker={vi.fn().mockResolvedValue(undefined)}
      />
    );

    expect(screen.getByRole("tablist", { name: /review sections/i })).toBeInTheDocument();
    expect(await axe(container)).toHaveNoViolations();
  });
});
