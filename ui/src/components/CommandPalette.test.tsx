import { useState } from "react";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { CommandPalette } from "./CommandPalette";

afterEach(() => {
  cleanup();
});

describe("CommandPalette", () => {
  it("navigates selected command and closes palette", () => {
    const onNavigate = vi.fn();
    const onOpenChange = vi.fn();

    render(<CommandPalette open onOpenChange={onOpenChange} onNavigate={onNavigate} />);

    fireEvent.click(screen.getByText("Go to Dashboard"));

    expect(onNavigate).toHaveBeenCalledWith("dashboard");
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("restores focus and clears search on close", async () => {
    function Harness() {
      const [open, setOpen] = useState(false);
      return (
        <>
          <button type="button" onClick={() => setOpen(true)}>
            Open palette
          </button>
          <CommandPalette open={open} onOpenChange={setOpen} onNavigate={vi.fn()} />
        </>
      );
    }

    render(<Harness />);

    const opener = screen.getByRole("button", { name: "Open palette" });
    opener.focus();
    fireEvent.click(opener);

    const dialog = screen.getByRole("dialog", { name: /command palette/i });
    const input = within(dialog).getByPlaceholderText("Type a command...");
    fireEvent.change(input, { target: { value: "Dashboard" } });
    fireEvent.keyDown(document, { key: "Escape" });

    await waitFor(() => {
      expect(screen.queryByRole("dialog", { name: /command palette/i })).not.toBeInTheDocument();
      expect(opener).toHaveFocus();
    });
  });
});
