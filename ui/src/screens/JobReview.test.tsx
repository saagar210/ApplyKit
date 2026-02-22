import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PacketDetail } from "../lib/types";
import { JobReview } from "./JobReview";

afterEach(() => {
  cleanup();
});

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
  it("runs export actions from the export tab", () => {
    const onExportMarkdown = vi.fn().mockResolvedValue(undefined);
    const onExportDocx = vi.fn().mockResolvedValue(undefined);
    const onExportPdf = vi.fn().mockResolvedValue(undefined);

    render(
      <JobReview
        detail={detail("/tmp/packet-a", "new", "follow up")}
        approvedOnly
        onCopy={vi.fn().mockResolvedValue(undefined)}
        onOpenFolder={vi.fn().mockResolvedValue(undefined)}
        onExportMarkdown={onExportMarkdown}
        onExportDocx={onExportDocx}
        onExportPdf={onExportPdf}
        onUpdateTracker={vi.fn().mockResolvedValue(undefined)}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: "export" }));
    fireEvent.click(screen.getByRole("button", { name: "Export Markdown Bundle" }));
    fireEvent.click(screen.getByRole("button", { name: "Export DOCX" }));
    fireEvent.click(screen.getByRole("button", { name: "Export PDF" }));

    expect(onExportMarkdown).toHaveBeenCalledWith("/tmp/packet-a");
    expect(onExportDocx).toHaveBeenCalledWith("/tmp/packet-a");
    expect(onExportPdf).toHaveBeenCalledWith("/tmp/packet-a");
    expect(screen.getByText("No violations.")).toBeInTheDocument();
  });

  it("copies messages from messages tab", () => {
    const onCopy = vi.fn().mockResolvedValue(undefined);
    render(
      <JobReview
        detail={detail("/tmp/packet-a", "new", "follow up")}
        approvedOnly
        onCopy={onCopy}
        onOpenFolder={vi.fn().mockResolvedValue(undefined)}
        onExportMarkdown={vi.fn().mockResolvedValue(undefined)}
        onExportDocx={vi.fn().mockResolvedValue(undefined)}
        onExportPdf={vi.fn().mockResolvedValue(undefined)}
        onUpdateTracker={vi.fn().mockResolvedValue(undefined)}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: "messages" }));
    const copyButtons = screen.getAllByRole("button", { name: "Copy" });
    fireEvent.click(copyButtons[0]);
    fireEvent.click(copyButtons[1]);
    fireEvent.click(copyButtons[2]);

    expect(onCopy).toHaveBeenCalledWith("Recruiter message");
    expect(onCopy).toHaveBeenCalledWith("Hiring manager message");
    expect(onCopy).toHaveBeenCalledWith("Cover short");
  });

  it("saves tracker updates when fields change", () => {
    const onUpdateTracker = vi.fn().mockResolvedValue(undefined);
    render(
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
    const saveButton = screen.getByRole("button", { name: "Save Tracker" });
    expect(saveButton).toBeDisabled();

    fireEvent.change(screen.getByPlaceholderText("Follow up on Monday"), {
      target: { value: "book interview prep" }
    });
    fireEvent.change(screen.getByPlaceholderText("Interview prep notes"), {
      target: { value: "focus on incident examples" }
    });
    expect(saveButton).not.toBeDisabled();
    fireEvent.click(saveButton);

    expect(onUpdateTracker).toHaveBeenCalledWith(
      "new",
      "book interview prep",
      "focus on incident examples"
    );
  });

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
