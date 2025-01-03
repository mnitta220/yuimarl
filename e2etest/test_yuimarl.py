import os
import re
from dotenv import load_dotenv
from playwright.sync_api import sync_playwright, Playwright, expect
from senario import senario1 as scenario_module


# .envファイルを読み込む
load_dotenv()

# 環境変数を取得
DOMAIN = os.getenv("DOMAIN")
SESSION_ID = os.getenv("SESSION_ID")
PAGE_URL = os.getenv("PAGE_URL")


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
        senario1 = scenario_module.Senario1(page, PAGE_URL)
        senario1.run()

        context.close()
        browser.close()
