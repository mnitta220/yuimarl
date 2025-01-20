import { test, expect } from "@playwright/test";
import dotenv from "dotenv";
import path from "path";
dotenv.config({ path: path.resolve(__dirname, ".env") });

test("scenario1", async ({ page }) => {
  test.slow();
  const password = process.env.E2E_TEST_PASSWORD || "";
  const url = process.env.PAGE_URL || "";
  await page.goto(`${url}/e2e_test`);

  await expect(page).toHaveTitle(/E2Eテスト - Yuimarl/);
  await page.getByLabel("E2E_TEST_PASSWORD").fill(password);
  await page
    .getByRole("row", { name: "kazuto.tonoma@e2e_test.com" })
    .locator("#user")
    .check();
  await page.getByRole("button", { name: "実行" }).click();
  await page.getByRole("link", { name: "プロジェクトを作成" }).click();
  await page.getByLabel("プロジェクト名").click();
  await page.getByLabel("プロジェクト名").fill("文化祭実行プロジェクト");
  await page.getByLabel("チケットID接頭辞").click();
  await page.getByLabel("チケットID接頭辞").fill("BN");
  //await page.getByRole("img", { name: "メンバーを追加" }).click();
  await page.getByTestId("addMember").click();
  await page.getByLabel("メールアドレス").click();
  await page.getByLabel("メールアドレス").fill("taro.yamada@e2e_test.com");
  await page.locator("#search-email").click();
  await page.locator("#role0").selectOption("3");
  await page.getByRole("button", { name: "メンバーに追加" }).click();
  //await page.getByRole("img", { name: "メンバーを追加" }).click();
  await page.getByTestId("addMember").click();
  await page.getByLabel("メールアドレス").click();
  await page.getByLabel("メールアドレス").fill("masami.iwaki@e2e_test.com");
  await page.locator("#search-email").click();
  await page.locator("#role0").selectOption("4");
  await page.getByRole("button", { name: "メンバーに追加" }).click();
  await page.getByLabel("日本の祝日を赤表示").check();
  await page.getByLabel("イテレーション / スプリント 番号を表示").check();
  await page.getByLabel("開始日").fill("2025-01-06");
  await page.getByLabel("開始番号").click();
  await page.getByLabel("開始番号").fill("2");
  await page.getByLabel("2週").check();
  await page.getByRole("button", { name: "作成" }).click();
  await page.getByRole("link", { name: "文化祭実行プロジェクト" }).click();
  await page.getByLabel("開始番号").click();
  await page.getByLabel("開始番号").fill("3");
  await page.getByRole("button", { name: "更新" }).click();
  await page.getByRole("link", { name: "文化祭実行プロジェクト" }).click();
  await page.getByRole("link", { name: "ノート" }).click();
  await page.locator("#markdown").click();
  await page.locator("#markdown").fill("# 見出し\n- リスト");
  await page.getByRole("button", { name: "更新" }).click();
  await expect(page.getByTestId("toast")).toContainText(
    "ノートを更新しました。"
  );
  await page.getByRole("link", { name: "ホーム (current)" }).click();
  await page.getByRole("link", { name: "チケットを追加" }).click();
  await page.getByLabel("チケット名").click();
  await page.getByLabel("チケット名").fill("ToDo");
  await page.getByLabel("ガントチャートに表示する").check();
  await page.getByRole("button", { name: "作成" }).click();
  await page.getByRole("link", { name: "チケットを追加" }).click();
  await page.getByLabel("チケット名").click();
  await page.getByLabel("チケット名").fill("たこやき模擬店");
  await page.getByLabel("内容").click();
  await page
    .getByLabel("内容")
    .fill("たこやき模擬店を出店するために、やるべきことを検討する。");
  //await page.getByRole("img", { name: "担当者を追加" }).click();
  await page.getByTestId("addCharge").click();
  await page.locator("#check0").check();
  await page.locator("#check1").check();
  await page.getByRole("button", { name: "担当者に追加" }).click();
  await page.locator("#start_date").fill("2025-01-20");
  await page.locator("#end_date").fill("2025-01-31");
  await page.locator("#progress").click();
  await page.locator("#progress").fill("1");
  await page.getByLabel("高").check();
  await page.getByLabel("ガントチャートに表示する").check();
  await page.locator("a:nth-child(3)").click();
  await page.getByRole("button", { name: "作成" }).click();
  await page.getByRole("link", { name: "BN2" }).click();
  await page.getByRole("link", { name: "親チケットを追加" }).click();
  await page.getByLabel("チケットID").click();
  await page.getByLabel("チケットID").fill("BN1");
  await page.getByRole("button", { name: "検索" }).click();
  await page.getByRole("button", { name: "親チケットを追加" }).click();
  await page.waitForTimeout(100);
  await expect(page.locator("#parentTicket")).toContainText("BN1 : ToDo");
  //await page.getByRole("img", { name: "成果物を追加" }).click();
  await page.getByTestId("addDeliverables").click();
  await page.getByLabel("成果物名").click();
  await page.getByLabel("成果物名").fill("計画書");
  await page.getByLabel("ファイルパス / URL").click();
  await page.getByLabel("ファイルパス / URL").fill("keikakusyo.txt");
  await page.getByRole("button", { name: "追加" }).click();
  await expect(page.locator("#deliverablesTbl")).toContainText("計画書");
  await expect(page.locator("#deliverablesTbl")).toContainText(
    "keikakusyo.txt"
  );
  await page.getByRole("button", { name: "更新" }).click();
  await expect(page.getByTestId("toast")).toContainText(
    "チケットを更新しました。"
  );
  await page.getByRole("link", { name: "ノート" }).click();
  await page.locator("#markdown").click();
  await page.locator("#markdown").fill("# head\n- list");
  await page.getByRole("button", { name: "更新" }).click();
  await expect(page.getByTestId("toast")).toContainText(
    "ノートを更新しました。"
  );
  await page.getByRole("link", { name: "コメント" }).click();
  await page.waitForTimeout(100);
  //await page.getByRole("img", { name: "コメントを追加" }).click();
  await page.getByTestId("addComment").click();
  await page.waitForTimeout(100);
  await page.locator("#markdown").click();
  await page.locator("#markdown").fill("コメント");
  await page.getByRole("button", { name: "追加" }).click();
  await expect(page.locator("b")).toContainText("殿馬一人");
  await page.getByRole("link", { name: "ログアウト" }).click();

  await page.goto(`${url}/e2e_test`);
  await expect(page).toHaveTitle(/E2Eテスト - Yuimarl/);
  await page.getByLabel("E2E_TEST_PASSWORD").fill(password);
  await page.getByRole("button", { name: "実行" }).click();
  await page
    .locator("#project")
    .getByRole("link", { name: "文化祭実行プロジェクト" })
    .click();
  await page.getByRole("link", { name: "ノート" }).click();
  await page.getByRole("link", { name: "ホーム (current)" }).click();
  await page.getByRole("link", { name: "BN2", exact: true }).click();
  await page.locator("#progress").click();
  await page.locator("#progress").fill("2");
  await page.getByRole("button", { name: "更新" }).click();
  await expect(page.getByTestId("toast")).toContainText(
    "チケットを更新しました。"
  );
  await page.getByRole("link", { name: "コメント" }).click();
  //await page.getByRole("img", { name: "コメントを追加" }).click();
  await page.getByTestId("addComment").click();
  await page.locator("#markdown").click();
  await page.locator("#markdown").fill("# Head\n- list");
  await page.getByRole("button", { name: "追加" }).click();
  await expect(page.locator("#comment1")).toContainText("山田太郎");
  await page.getByRole("link", { name: "ログアウト" }).click();

  await page.goto(`${url}/e2e_test`);
  await expect(page).toHaveTitle(/E2Eテスト - Yuimarl/);
  await page.getByLabel("E2E_TEST_PASSWORD").fill(password);
  await page
    .getByRole("row", { name: "satoru.satonaka@e2e_test.com" })
    .locator("#user")
    .check();
  await page.getByRole("button", { name: "実行" }).click();
  await page.getByRole("link", { name: "プロジェクト一覧" }).click();
});
