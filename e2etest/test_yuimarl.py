import os
import re
from dotenv import load_dotenv
from playwright.sync_api import Page, expect
from senario import senario1 as scenario_module


# .envファイルを読み込む
load_dotenv()

# 環境変数を取得
# DOMAIN = os.getenv("DOMAIN")
# SESSION_ID = os.getenv("SESSION_ID")
PAGE_URL = os.getenv("PAGE_URL")
E2E_PASSWORD = os.getenv("E2E_PASSWORD")

"""
def test_senario1():
    with sync_playwright() as playwright:
        chromium = playwright.chromium
        browser = chromium.launch()
        context = browser.new_context()
        context.add_cookies(
            [
                {
                    "name": "yuimarl_session-id",
                    "value": SESSION_ID,
                    "domain": DOMAIN,
                    "path": "/",
                },
            ]
        )
        page = context.new_page()
        senario1 = scenario_module.Senario1(page, PAGE_URL, E2E_PASSWORD)
        senario1.run()

        context.close()
        browser.close()
"""


def test_has_title(page: Page):
    # page.goto("https://playwright.dev/")

    # Expect a title "to contain" a substring.
    # expect(page).to_have_title(re.compile("Playwright"))
    # URLを組み立てる
    test_url = f"{PAGE_URL}/e2e_test/{E2E_PASSWORD}"
    print(f"***test_url: {test_url}")
    page.goto(test_url)
    senario1 = scenario_module.Senario1(page)
    senario1.run()
