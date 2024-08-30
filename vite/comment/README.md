# コメント画面用フロントエンドモジュール

このフォルダは、コメント画面を表示するためのフロントエンドモジュールです。  
[Vite](https://ja.vitejs.dev/) を使用して作成しました。

プログラミング言語は、TypeScript が使用されています。  
npm コマンドで JavaScript にコンパイルされます。  
出力された JavaScript ファイルを、Yuimarl のプロジェクトに組み込みます。

## インストール

1. `vite\comment` フォルダで、次のコマンドを実行します。

```
npm install
```

## ビルド

1. `vite\comment` フォルダで、次のコマンドを実行します。

```
npm run build
```

2. `vite\comment\dist\assets` の下に出力された `index-XXXXXXXX.js` を、 `static\js\comment` にコピーし、既存の `index-XXXXXXXX.js` を削除します。
3. `src\components\head.rs` にある `<script ・・・ index-XXXXXXXX.js` を書き換えます。

```Rust
    *buf += r#"<script type="module" crossorigin src="/static/js/comment/index-XXXXXXXX.js"></script>"#;
```
