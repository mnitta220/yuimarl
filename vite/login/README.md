# ログイン画面用フロントエンドモジュール

このフォルダは、ログイン画面を表示するためのフロントエンドモジュールです。  
[Vite](https://ja.vitejs.dev/) を使用して作成しました。

プログラミング言語は、JavaScript が使用されています。  
npm コマンドで ビルドが行われます。  
出力された JavaScript ファイルを、Yuimarl のプロジェクトに組み込みます。

## インストール

1. `vite\login` フォルダで、次のコマンドを実行します。

```
npm install
```

## ビルド

1. `vite\login` フォルダで、次のコマンドを実行します。

```
npm run build
```

2. `vite\login\dist\assets` の下に出力された `index-XXXXXXXX.js` を、 `static\js\login` にコピーし、既存の `index-XXXXXXXX.js` を削除します。
3. `src\pages\login_page.rs` にある `<script ・・・ index-XXXXXXXX.js` を書き換えます。

```Rust
    buf += r#"<script type="module" crossorigin src="/static/js/login/index-XXXXXXXX.js"></script>"#;
```
