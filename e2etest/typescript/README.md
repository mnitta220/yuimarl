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

## テストの実行

ブラウザを起動する場合

```
npx playwright test --headed
```

ブラウザを起動しない場合

```
npx playwright test
```

## (参考情報) テストコードの生成

https://playwright.dev/docs/codegen-intro

```
npx playwright codegen
```
