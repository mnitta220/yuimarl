# ガントチャート表示用フロントエンドモジュール

## Vite

https://ja.vitejs.dev/

インストール

```
npm install
```

ローカル起動

```
npm run dev
```

ビルド

```
npm run build
```

`gantt-chart\dist\assets` の下に出力された `*.js` と `*.css` ファイルを、 `static\js\gantt-chart` にコピーし、既存の `*.js` と `*.css` ファイルを削除する。

`src\components\head.rs` にある `*.js` と `*.css` のインポートを書き換える。
