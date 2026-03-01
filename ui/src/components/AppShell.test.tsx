import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { AppShell } from "./AppShell";

describe("AppShell preview controls", () => {
  it("supports arrow-key and click preview mode changes", () => {
    const onPreviewModeChange = vi.fn();

    const { rerender } = render(
      <AppShell
        sidebar={<div>Sidebar</div>}
        main={<div>Main</div>}
        preview={<div>Preview Content</div>}
        diffPreview={<div>Diff Content</div>}
        showPreview
        previewMode="preview"
        onPreviewModeChange={onPreviewModeChange}
        onTogglePreview={vi.fn()}
      />
    );

    const tabList = screen.getByRole("tablist", { name: "Preview mode" });
    fireEvent.keyDown(tabList, { key: "ArrowRight" });
    fireEvent.keyDown(tabList, { key: "ArrowLeft" });
    fireEvent.keyDown(tabList, { key: "Enter" });
    fireEvent.click(screen.getByRole("tab", { name: "Diff" }));

    expect(onPreviewModeChange).toHaveBeenCalledTimes(2);
    expect(onPreviewModeChange).toHaveBeenNthCalledWith(1, "diff");
    expect(onPreviewModeChange).toHaveBeenNthCalledWith(2, "diff");

    rerender(
      <AppShell
        sidebar={<div>Sidebar</div>}
        main={<div>Main</div>}
        preview={<div>Preview Content</div>}
        diffPreview={<div>Diff Content</div>}
        showPreview
        previewMode="diff"
        onPreviewModeChange={onPreviewModeChange}
        onTogglePreview={vi.fn()}
      />
    );
    fireEvent.keyDown(screen.getByRole("tablist", { name: "Preview mode" }), { key: "ArrowRight" });
    expect(onPreviewModeChange).toHaveBeenNthCalledWith(3, "preview");
  });
});
