import re
from playwright.sync_api import Page, expect

"""
# テストシナリオ1
- タイトルに"Yuimarl"が含まれることを確認する
"""


class Senario1:
    def __init__(self, page: Page, url):
        self.page = page
        self.url = url

    def run(self):
        self.page.goto(self.url)

        # タイトルに"Yuimarl"が含まれることを確認する
        expect(self.page).to_have_title(re.compile("Yuimarl"))

        # タイトルが"Yuimarl ログイン"の場合、"login"と出力
        title = self.page.title()
        assert title == "Yuimarl"

        # title属性が「プロジェクト一覧」であるリンクをクリックする
        self.page.get_by_title("プロジェクト一覧").click()
        # self.page.get_by_test_id("project_list").click()
        # self.page.get_by_test_id("directions").click()
        # self.page.locator("//button[@id='directions']").click()
        # self.page.get_by_role("link", title="プロジェクト一覧").click()

        # タイトルに"プロジェクト一覧"が含まれることを確認する
        expect(self.page).to_have_title(re.compile("プロジェクト一覧"))
