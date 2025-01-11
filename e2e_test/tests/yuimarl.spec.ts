import { test, expect } from "@playwright/test";
import dotenv from "dotenv";
import path from "path";
dotenv.config({ path: path.resolve(__dirname, ".env") });

test("scenario1", async ({ page }) => {
  console.log("scenario1");
  const password = process.env.E2E_TEST_PASSWORD || "";
  const url = process.env.PAGE_URL || "";
  await page.goto(`${url}/e2e_test`);

  await expect(page).toHaveTitle(/E2Eテスト - Yuimarl/);
  await page.getByLabel("E2E_TEST_PASSWORD").fill(password);
  await page
    .getByRole("row", { name: "masami.iwaki@e2e_test.com 岩鬼正美" })
    .locator("#user")
    .check();
  await page.getByRole("button", { name: "実行" }).click();
  await expect(page).toHaveTitle(/Yuimarl/);
  await page.getByRole("link", { name: "プロジェクトを作成" }).click();
  await expect(page.getByRole("heading")).toContainText("プロジェクトを作成");
  await page.getByLabel("プロジェクト名").fill("文化祭実行プロジェクト");
  await page.getByLabel("チケットID接頭辞").fill("BN");
  await page.getByRole("img", { name: "メンバーを追加" }).click();
  await page.getByLabel("メールアドレス").fill("taro.yamada@e2e_test.com");
  await page.locator("#search-email").click();
  await page.getByRole("button", { name: "メンバーに追加" }).click();
  await page.getByRole("img", { name: "メンバーを追加" }).click();
  await page.getByLabel("メールアドレス").fill("kazuto.tonoma@e2e_test.com");
  await page.locator("#search-email").click();
  await page.locator("#role0").selectOption("4");
  await page.getByRole("button", { name: "メンバーに追加" }).click();
  await page.getByLabel("日本の祝日を赤表示").check();
  await page.getByLabel("イテレーション / スプリント 番号を表示").check();
  await page.getByLabel("開始日").fill("2025-01-15");
  await page.getByLabel("開始番号").fill("2");
  await page.getByLabel("2週").check();
  await page.getByRole("button", { name: "作成" }).click();
  await expect(
    page.getByRole("link", { name: "文化祭実行プロジェクト" })
  ).toBeVisible();
  await page.getByRole("link", { name: "文化祭実行プロジェクト" }).click();
  await expect(page.getByRole("heading")).toContainText("プロジェクト");
  await page.getByRole("link", { name: "ノート" }).click();
  await page.locator("#markdown").fill("# 見出し1\n- リスト1\n\n本文");
  await page.getByRole("button", { name: "更新" }).click();
  await expect(
    page.getByRole("link", { name: "文化祭実行プロジェクト" })
  ).toBeVisible();
  await page.getByRole("link", { name: "チケットを追加" }).click();
  await expect(page.getByRole("heading")).toContainText("チケットを作成");
  await expect(page.locator("#post_ticket")).toContainText(
    "文化祭実行プロジェクト"
  );
  await page.getByLabel("チケット名").fill("たこやき模擬店");
  await page
    .getByLabel("内容")
    .fill("たこやき模擬店を出店するために、やるべきことを検討する。");
  await page.getByRole("img", { name: "担当者を追加" }).click();
  await page.locator("#check0").check();
  await page.locator("#check1").check();
  await page.getByRole("button", { name: "担当者に追加" }).click();
  await page.locator("#start_date").fill("2025-01-13");
  await page.locator("#end_date").fill("2025-01-31");
  await page.locator("#progress").fill("10");
  await page.getByLabel("高").check();
  await page.getByLabel("ガントチャートに表示する").check();
  await page.locator("a:nth-child(3)").click();
  await page.getByRole("button", { name: "作成" }).click();
  await page.getByRole("link", { name: "BN1" }).click();
  await page.locator("#progress").fill("20");
  await page.getByRole("button", { name: "更新" }).click();
  await page.getByRole("link", { name: "BN1" }).click();
  await page.getByRole("link", { name: "ノート" }).click();
  await page.locator("#markdown").fill("ノート");
  await page.getByRole("button", { name: "更新" }).click();
  await expect(page).toHaveTitle(/Yuimarl/);
});
