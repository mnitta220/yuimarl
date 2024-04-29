# Yuimarl（ゆいまーる）

ホームページ:  
https://mnitta220.github.io/yuimarl/index2.html

# Command

## VSCode のワークスペースを起動

```sh
code ./workspace.code-workspace
```

## pug-cli のインストール

```sh
npm install pug-cli -g
```

## pug ファイルのコンパイル

index.pug をコンパイル

```sh
pug -P index.pug
```

proto フォルダーの pug をすべてコンパイル

```sh
pug -P proto
```

## ローカルでの実行

```sh
cargo run
```

## Docker イメージのビルドと Docker Hub へのプッシュ

```sh
docker image build -t masahironitta/yuimarl:0.0.12 .
docker image push masahironitta/yuimarl:0.0.12
```

# Links

- [Rust](https://www.rust-lang.org/ja/)
- [axum](https://github.com/tokio-rs/axum)
- [Google Cloud](https://cloud.google.com/?hl=ja)
- [Cloud Run](https://cloud.google.com/run?hl=ja)
- [Firebase](https://firebase.google.com/)
- [Firebase Authentication](https://github.com/firebase/firebaseui-web)
- [FirebaseUI Auth](https://github.com/firebase/firebaseui-web)
- [Firestore](https://cloud.google.com/firestore?hl=ja)
- [Firestore for Rust](https://crates.io/crates/firestore)
- [Bootstrap](https://getbootstrap.jp/)
- [jQuery](http://jquery.com/)
- [Pug](https://pugjs.org/api/getting-started.html)
- [pug-cli](https://github.com/pugjs/pug-cli)
