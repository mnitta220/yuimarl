import re
from playwright.sync_api import Page, expect


# テストシナリオ1
class Senario1:
    def __init__(self, page: Page, url: str, e2e_password: str):
        self.page = page
        self.url = url
        self.e2e_password = e2e_password

    def run(self):
        # URLを組み立てる
        test_url = f"{self.url}/e2e_test"
        self.page.goto(test_url)

        # タイトルに"E2Eテスト"が含まれることを確認する
        # expect(self.page).to_have_title(re.compile("E2Eテスト"))

        title = self.page.title()
        assert title == "E2Eテスト - Yuimarl"
        self.page.get_by_label("E2E_TEST_PASSWORD").fill(self.e2e_password)
        self.page.get_by_role("button", name="実行").click()

        # タイトルに"Yuimarl"が含まれることを確認する
        # expect(self.page).to_have_title(re.compile("Yuimarl"))
        title = self.page.title()
        assert title == "Yuimarl"

        """
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
        """

        # 新しいプロジェクトを作成する
        self.page.get_by_title("プロジェクトを作成").click()
        expect(self.page).to_have_title(re.compile("プロジェクトを作成"))
        self.page.get_by_label("プロジェクト名").fill("文化祭実行プロジェクト")
        self.page.get_by_label("チケットID接頭辞").fill("BN")
        # 「メンバーを追加」をクリックする。
        self.page.get_by_title("メンバーを追加").click()
        # 名前に「岩鬼正美」を入力する。
        self.page.get_by_label("名前").fill("岩鬼正美")
        # idが"search-name"のボタンをクリックする。
        self.page.locator("//button[@id='search-name']").click()
        # self.page.get_by_id("search-name").click()
        # idが"searched"のdivの中にあるtableのtrの行数が2でなければエラーにする
        # expect(
        #    self.page.get_by_id("searched").get_by_tag("table").get_by_tag("tr").count()
        # ).to_be(3)
        expect(self.page.locator("#searched > table > tr")).to_have_count(3)

        # 「日本の祝日を赤表示」にチェックを入れる
        self.page.get_by_label("日本の祝日を赤表示").check()
        # 「イテレーション / スプリント 番号を表示」にチェックを入れる
        self.page.get_by_label("イテレーション / スプリント 番号を表示").check()
        # 開始日を2025-01-01に設定する
        self.page.get_by_label("開始日").fill("2025-01-01")
        # 開始番号を2に設定する
        self.page.get_by_label("開始番号").fill("2")
        # 単位を2週に設定する
        # self.page.get_by_label("単位：  ").select_option("2週")
        # 単位のラジオボタンで2週を選択する
        self.page.get_by_label("2週").click()
        self.page.get_by_role("button", name=" 作成").click()
        self.page.get_by_title("プロジェクト一覧").click()
        expect(self.page).to_have_title(re.compile("プロジェクト一覧"))
        self.page.get_by_role("link", name="文化祭実行プロジェクト").click()
        expect(self.page).to_have_title(re.compile("プロジェクト - Yuimarl"))
