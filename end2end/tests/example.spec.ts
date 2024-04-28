import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Loss Comparison");

  await expect(page.locator("h1")).toHaveText("Loss Comparison!");
});
