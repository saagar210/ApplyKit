import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import App from "./App";

describe("App", () => {
  it("renders dashboard title", () => {
    render(<App />);
    expect(screen.getByText("ApplyKit Dashboard")).toBeInTheDocument();
  });

  it("supports preview pane toggle controls", () => {
    render(<App />);
    expect(screen.getAllByRole("button", { name: "Preview" }).length).toBeGreaterThan(0);
    expect(screen.getAllByRole("button", { name: "Diff" }).length).toBeGreaterThan(0);

    fireEvent.click(screen.getAllByRole("button", { name: "Hide Pane" })[0]);
    expect(screen.getAllByRole("button", { name: "Show Preview Pane" }).length).toBeGreaterThan(0);
  });
});
