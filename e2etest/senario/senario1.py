import re
from playwright.sync_api import Page, expect


# テストシナリオ1
class Senario1:
    def __init__(self, page: Page, url):
        self.page = page
        self.url = url

    def run(self):
        self.page.goto(self.url)

        # タイトルに"Yuimarl"が含まれることを確認する
        expect(self.page).to_have_title(re.compile("Yuimarl"))

        title = self.page.title()
        assert title == "Yuimarl"

        self.page.get_by_title("プロジェクト一覧").click()
        # self.page.get_by_test_id("project_list").click()
        # self.page.get_by_test_id("directions").click()
        # self.page.locator("//button[@id='directions']").click()
        # self.page.get_by_role("link", title="プロジェクト一覧").click()

        # タイトルに"プロジェクト一覧"が含まれることを確認する
        expect(self.page).to_have_title(re.compile("プロジェクト一覧"))

        if self.page.get_by_role("link", name="__E2E_TEST__").count() > 0:
            # 既存のプロジェクトを削除する
            self.page.get_by_role("link", name="__E2E_TEST__").click()
            expect(self.page).to_have_title(re.compile("プロジェクト - Yuimarl"))
            self.page.get_by_role("button", name=" 削除").click()
            self.page.locator("//button[@id='btnProjectDel']").click()

        # 新しいプロジェクトを作成する
        self.page.get_by_title("プロジェクトを作成").click()
        expect(self.page).to_have_title(re.compile("プロジェクトを作成"))
        self.page.get_by_label("プロジェクト名").fill("__E2E_TEST__")
        self.page.get_by_label("チケットID接頭辞").fill("ET")
        self.page.get_by_role("button", name=" 作成").click()
