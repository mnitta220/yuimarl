# E2E テスト (TypeScript 版)

Playwright の公式ドキュメント  
https://playwright.dev/

## インストール

次のコマンドを実行してください。

```
npm install
```

## 環境変数の設定

### ■ ローカル環境(http://localhost:8080)で起動した Yuimarl の E2E テストを実行する場合

1. プロジェクトのルートフォルダにある.env ファイルに「E2E_TEST_PASSWORD」を設定してください。

```
E2E_TEST_PASSWORD=(テスト用パスワード)
```

2. /e2etest/typescript フォルダにある .env.template ファイルをコピーして .env ファイルを作成し、PAGE_URL と E2E_TEST_PASSWORD を設定してください。

```
PAGE_URL=(yuimarlのURL)
E2E_TEST_PASSWORD=(テスト用パスワード(1.で設定したパスワード))
```

(例)

```
PAGE_URL=http://localhost:8080
E2E_TEST_PASSWORD=password
```

### ■ CloudRun で起動した Yuimarl の E2E テストを実行する場合

1. CloudRun のコンソールで、環境変数に「E2E_TEST_PASSWORD」を設定してください。

```
E2E_TEST_PASSWORD=(テスト用パスワード)
```

2. /e2etest/typescript フォルダにある .env.template ファイルをコピーして .env ファイルを作成し、PAGE_URL と E2E_TEST_PASSWORD を設定してください。

```
PAGE_URL=(yuimarlのURL)
E2E_TEST_PASSWORD=(テスト用パスワード(1.で設定したパスワード))
```

(例)

```
PAGE_URL=https://yuimarl-2u5rvkqlq-an.a.run.app/
E2E_TEST_PASSWORD=password
```

## ブラウザの設定

初期状態では、Chrome で E2E テストが実行されます。  
[e2etest\typescript\playwright.config.ts](playwright.config.ts) の「projects:」の設定を変えると、Firefox または Safari を起動することができます。  
その場合は、"chromium" をコメントアウトして、"firefox" または "webkit" のコメントアウトを解除してください。

```
  projects: [
    {
      name: "setup db",
      testMatch: /global\.setup\.ts/,
    },

    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
      dependencies: ["setup db"],
    },

    /*
    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
      dependencies: ["setup db"],
    },
    */

    /*
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
      dependencies: ["setup db"],
    },
    */

```

## テストの実行

ブラウザを起動する場合

```
npx playwright test --headed
```

ブラウザを起動しない場合

```
npx playwright test
```

## テストシナリオ

上記のテストでは、以下のテスト用ユーザーを自動的に作成してテストを実行します。

| ユーザー名 | メールアドレス               |
| ---------- | ---------------------------- |
| 山田太郎   | taro.yamada@e2e_test.com     |
| 殿馬一人   | kazuto.tonoma@e2e_test.com   |
| 岩鬼正美   | masami.iwaki@e2e_test.com    |
| 里中智     | satoru.satonaka@e2e_test.com |

以下の URL にアクセスして、テストデータの初期化とログインを行います。

- E2E テストデータ初期化 URL

```
(PAGE_URL)/e2e_init/(E2E_TEST_PASSWORD)
```

- E2E テストログイン URL

```
(PAGE_URL)/e2e_test
```

## (参考情報) テストコードの生成

https://playwright.dev/docs/codegen-intro

```
npx playwright codegen
```
