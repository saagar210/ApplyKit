import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { PacketDetail } from "../lib/types";
import { JobReview } from "./JobReview";

function detail(packetDir: string, status: string, nextAction: string): PacketDetail {
  return {
    packetDir,
    extractedKeywords: ["support"],
    extractedTools: ["Okta"],
    extractedRequirements: ["Experience with incidents"],
    fitBreakdown: {
      roleMatch: 25,
      stackMatch: 10,
      scaleMatch: 5,
      rigorMatch: 5,
      signalBoost: 5,
      total: 50,
      whyMatch: ["Primary track aligned: Support/Ops Core"],
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
      status,
      nextAction,
      packetDir
    },
    truthReport: {
      passed: true,
      violations: [],
      unknownTools: [],
      claimIssues: [],
      provenanceComplete: true
    }
  };
}

describe("JobReview tracker state", () => {
  it("resets tracker inputs when packet detail changes", () => {
    const onUpdateTracker = vi.fn().mockResolvedValue(undefined);
    const { rerender } = render(
      <JobReview
        detail={detail("/tmp/packet-a", "new", "follow up")}
        approvedOnly
        onCopy={vi.fn().mockResolvedValue(undefined)}
        onOpenFolder={vi.fn().mockResolvedValue(undefined)}
        onExportMarkdown={vi.fn().mockResolvedValue(undefined)}
        onExportDocx={vi.fn().mockResolvedValue(undefined)}
        onExportPdf={vi.fn().mockResolvedValue(undefined)}
        onUpdateTracker={onUpdateTracker}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: "tracker" }));

    const statusSelect = screen.getByDisplayValue("new") as HTMLSelectElement;
    fireEvent.change(statusSelect, { target: { value: "reply" } });
    const nextActionInput = screen.getByPlaceholderText("Follow up on Monday") as HTMLInputElement;
    fireEvent.change(nextActionInput, { target: { value: "custom action" } });

    rerender(
      <JobReview
        detail={detail("/tmp/packet-b", "applied", "send follow-up")}
        approvedOnly
        onCopy={vi.fn().mockResolvedValue(undefined)}
        onOpenFolder={vi.fn().mockResolvedValue(undefined)}
        onExportMarkdown={vi.fn().mockResolvedValue(undefined)}
        onExportDocx={vi.fn().mockResolvedValue(undefined)}
        onExportPdf={vi.fn().mockResolvedValue(undefined)}
        onUpdateTracker={onUpdateTracker}
      />
    );

    expect(screen.getByDisplayValue("applied")).toBeInTheDocument();
    expect(screen.getByDisplayValue("send follow-up")).toBeInTheDocument();
  });
});
