import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { NewJob } from "./NewJob";

describe("NewJob import", () => {
  it("imports JD file content into textarea", async () => {
    render(<NewJob busy={false} onGenerate={vi.fn().mockResolvedValue(undefined)} />);

    const fileInput = screen.getByLabelText("Import JD file") as HTMLInputElement;
    const file = new File(["line one\r\nline two"], "jd.txt", { type: "text/plain" });
    fireEvent.change(fileInput, { target: { files: [file] } });

    await waitFor(() => {
      const textarea = screen.getByPlaceholderText("Paste job description here") as HTMLTextAreaElement;
      expect(textarea.value).toBe("line one\nline two");
    });
  });
});
