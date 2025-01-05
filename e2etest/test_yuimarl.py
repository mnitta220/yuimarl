import os
from dotenv import load_dotenv
from playwright.sync_api import Page
from senario import senario1 as scenario_module


# .envファイルを読み込む
load_dotenv()

# 環境変数を取得
PAGE_URL = os.getenv("PAGE_URL")
E2E_PASSWORD = os.getenv("E2E_PASSWORD")


def test_senario1(page: Page):
    senario1 = scenario_module.Senario1(page, PAGE_URL, E2E_PASSWORD)
    senario1.run()
