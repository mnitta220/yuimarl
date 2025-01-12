# import pytest
import os

# import re
# from playwright.sync_api import Playwright, sync_playwright, Page, expect
from playwright.sync_api import Page, expect
from dotenv import load_dotenv

# from playwright.sync_api import Page
# from senario import senario1 as scenario_module


# .envファイルを読み込む
load_dotenv()

# 環境変数を取得
PAGE_URL = os.getenv("PAGE_URL")
E2E_TEST_PASSWORD = os.getenv("E2E_TEST_PASSWORD")

# def test_senario1(page: Page):
#     senario1 = scenario_module.Senario1(page, PAGE_URL, E2E_PASSWORD)
#     senario1.run()


def test_senario1(page: Page):
    # 初期化
    test_url = f"{PAGE_URL}/e2e_init/{E2E_TEST_PASSWORD}"
    page.goto(test_url)

    # テスト
    test_url = f"{PAGE_URL}/e2e_test"
    page.goto(test_url)
    expect(page.get_by_role("heading")).to_contain_text("E2Eテスト")
    page.get_by_label("E2E_TEST_PASSWORD").fill(E2E_TEST_PASSWORD)
    page.get_by_role("row", name="kazuto.tonoma@e2e_test.com").locator("#user").check()
    page.get_by_role("button", name="実行").click()
    expect(page.locator("#project")).to_contain_text("プロジェクト")
    page.get_by_role("link", name="プロジェクトを作成").click()
    expect(page.get_by_role("heading")).to_contain_text("プロジェクトを作成")
    page.get_by_label("プロジェクト名").fill("文化祭実行プロジェクト")
    page.get_by_label("チケットID接頭辞").fill("BN")
    page.get_by_role("img", name="メンバーを追加").click()
    page.get_by_label("メールアドレス").fill("taro.yamada@e2e_test.com")
    page.locator("#search-email").click()
    page.locator("#role0").select_option("3")
    page.get_by_role("button", name="メンバーに追加").click()
    page.get_by_role("img", name="メンバーを追加").click()
    page.get_by_label("メールアドレス").fill("masami.iwaki@e2e_test.com")
    page.locator("#search-email").click()
    page.locator("#role0").select_option("4")
    page.get_by_role("button", name="メンバーに追加").click()
    page.get_by_label("日本の祝日を赤表示").check()
    page.get_by_label("イテレーション / スプリント 番号を表示").check()
    page.get_by_label("開始日").fill("2025-01-14")
    page.get_by_label("開始番号").fill("10")
    page.get_by_label("2週").check()
    page.get_by_role("button", name="作成").click()
    expect(page.get_by_role("link", name="文化祭実行プロジェクト")).to_be_visible()
    page.get_by_role("link", name="文化祭実行プロジェクト").click()
    page.get_by_role("link", name="ノート").click()
    page.locator("#markdown").fill("# 見出し1\n- リスト1\n\n本文")
    page.get_by_role("button", name="更新").click()
    page.get_by_role("link", name="チケットを追加").click()
    page.get_by_label("チケット名").fill("たこやき模擬店")
    page.get_by_label("内容").fill(
        "たこやき模擬店を出店するために、やるべきことを検討する。"
    )
    page.get_by_role("img", name="担当者を追加").click()
    page.locator("#check0").check()
    page.locator("#check1").check()
    page.locator("#check2").check()
    page.get_by_role("button", name="担当者に追加").click()
    page.locator("#start_date").fill("2025-01-15")
    page.locator("#end_date").fill("2025-01-31")
    page.locator("#progress").fill("10")
    page.get_by_label("高").check()
    page.get_by_label("ガントチャートに表示する").check()
    page.locator(".col-md-9 > a:nth-child(3)").click()
    page.get_by_role("button", name="作成").click()
    page.get_by_role("link", name="BN1").click()
    page.get_by_role("link", name="ノート").click()
    page.locator("#markdown").fill("ノート")
    page.get_by_role("button", name="更新").click()
    page.get_by_role("link", name="ログアウト").click()

    page.goto(test_url)
    expect(page.get_by_role("heading")).to_contain_text("E2Eテスト")
    page.get_by_label("E2E_TEST_PASSWORD").fill(E2E_TEST_PASSWORD)
    page.get_by_role("row", name="masami.iwaki@e2e_test.com 岩鬼正美").locator(
        "#user"
    ).check()
    page.get_by_role("button", name="実行").click()
    expect(page.get_by_role("rowgroup")).to_contain_text(
        "プロジェクト 「 文化祭実行プロジェクト 」 に追加されました。"
    )
    expect(page.get_by_role("rowgroup")).to_contain_text(
        "チケット 「 BN1 : たこやき模擬店 」 の担当者に追加されました。"
    )
    page.get_by_role("link", name="BN1", exact=True).click()
    expect(page.get_by_label("チケット名")).to_have_value("たこやき模擬店")
    expect(page.get_by_role("button", name="更新")).to_be_visible()
    page.locator("#progress").fill("20")
    page.get_by_role("button", name="更新").click()
    page.get_by_role("link", name="閉じる").first.click()
    page.get_by_role("link", name="閉じる").first.click()
    page.get_by_role("link", name="閉じる").click()
