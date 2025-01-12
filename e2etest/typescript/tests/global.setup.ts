import { chromium, expect, test as setup } from "@playwright/test";
import dotenv from "dotenv";
import path from "path";
dotenv.config({ path: path.resolve(__dirname, ".env") });

setup("initialize e2e test data", async ({}) => {
  console.log("initialize e2e test data...");
  const password = process.env.E2E_TEST_PASSWORD;
  const url = process.env.PAGE_URL;
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto(`${url}/e2e_init/${password}`);
  // ページの内容がOKになっていることを確認
  const bodyText = await page.evaluate(() => document.body.textContent?.trim());
  expect(bodyText).toBe("OK");

  await browser.close();
});
