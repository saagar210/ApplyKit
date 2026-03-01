import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ToastHost } from "./ToastHost";

describe("ToastHost", () => {
  it("renders region and tone-specific live roles", () => {
    render(
      <ToastHost
        toasts={[
          { id: "1", tone: "info", message: "Heads up" },
          { id: "2", tone: "error", message: "Something failed" }
        ]}
      />
    );

    expect(screen.getByRole("region", { name: /notifications/i })).toBeInTheDocument();
    expect(screen.getByRole("status")).toHaveTextContent("Heads up");
    expect(screen.getByRole("alert")).toHaveTextContent("Something failed");
  });
});
