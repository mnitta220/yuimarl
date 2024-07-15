# ガントチャート表示用フロントエンドモジュール

このフォルダは、ガントチャートを表示するためのフロントエンドモジュールです。  
[Vite](https://ja.vitejs.dev/) を使用して作成しました。

プログラミング言語は、TypeScript が使用されています。  
npm コマンドで JavaScript にコンパイルされます。  
出力された JavaScript と CSS ファイルを、Yuimarl のプロジェクトに組み込みます。

## インストール

1. `gantt-chart` フォルダで、次のコマンドを実行します。

```
npm install
```

## ローカル起動

1. `gantt-chart` フォルダで、次のコマンドを実行します。

```
npm run dev
```

2. ブラウザで、次の URL にアクセスして動作を確認します。  
   http://localhost:5173/

## ビルド

1. `gantt-chart` フォルダで、次のコマンドを実行します。

```
npm run build
```

2. `gantt-chart\dist\assets` の下に出力された `index-XXXXXXXX.js` と `index-XXXXXXXX.css` を、 `static\js\gantt-chart` にコピーし、既存の `index-XXXXXXXX.js` と `index-XXXXXXXX.css` を削除します。
3. `src\components\head.rs` にある `<script ・・・ index-XXXXXXXX.js` と `<link ・・・ index-XXXXXXXX.css` を書き換えます。

```Rust
    *buf += r#"<script type="module" crossorigin src="/static/js/gantt-chart/index-XXXXXXXX.js"></script>"#;
    *buf += r#"<link rel="stylesheet" crossorigin href="/static/js/gantt-chart/index-XXXXXXXX.css">"#;
```
