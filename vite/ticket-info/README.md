# チケット情報画面基本情報タブ用フロントエンドモジュール

このフォルダは、チケット情報画面の基本情報タブを表示するためのフロントエンドモジュールです。  
[Vite](https://ja.vitejs.dev/) を使用して作成しました。

プログラミング言語は、TypeScript が使用されています。  
npm コマンドで JavaScript にトランスパイルされます。  
出力された JavaScript ファイルを、Yuimarl のプロジェクトに組み込みます。

## インストール

1. `vite\ticket-info` フォルダで、次のコマンドを実行します。

```
npm install
```

## ビルド

1. `vite\ticket-info` フォルダで、次のコマンドを実行します。

```
npm run build
```

2. `vite\ticket-info\dist\assets` の下に出力された `index-XXXXXXXX.js` を、 `static\js\ticket-info` にコピーし、既存の `index-XXXXXXXX.js` を削除します。
3. `src\components\head.rs` にある `<script ・・・ index-XXXXXXXX.js` を書き換えます。

```Rust
    *buf += r#"<script type="module" crossorigin src="/static/js/ticket-info/index-XXXXXXXX.js"></script>"#;
```
