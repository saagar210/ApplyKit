import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test.describe("UI accessibility smoke", () => {
  test("dashboard shell is keyboard-usable and has no serious violations", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("heading", { name: "ApplyKit Dashboard" })).toBeVisible();
    const skipLink = page.getByRole("link", { name: "Skip to main content" });
    await expect(skipLink).toBeAttached();

    await page.keyboard.press("Tab");
    await expect(skipLink).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(page.getByRole("button", { name: "Jobs" })).toBeFocused();

    await page.keyboard.press("Control+k");
    const dialog = page.getByRole("dialog", { name: "Command palette" });
    await expect(dialog).toBeVisible();
    await expect(page.getByPlaceholder("Type a command...")).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();

    const results = await new AxeBuilder({ page }).analyze();
    const blocking = results.violations.filter((violation) =>
      ["critical", "serious"].includes(violation.impact ?? "")
    );
    expect(blocking).toEqual([]);
  });
});
