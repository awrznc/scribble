# webpack

## webpackとは

モジュールバンドラのこと。
（モジュールバンドラとは、複数のモジュールを1つにまとめたファイルを出力するツールのこと）

出力された成果物はバンドルと呼ばれる。

## init

```.sh

# プロジェクトの作成
$ npm init

# webpack のインストール
$ npm install webpack webpack-cli

# src の追加
$ mkdir src
$ touch src/index.js

# webpack の設定ファイルを作成
$ touch webpack.config.js

```

## build

```.sh

# バンドラの生成
$ npx webpack

```

## debug

```.sh

# 出力したバンドルを実行する
$ node dist/bundle.js

# もしくは html ファイルを開いてデバッグする
$ open index.html

```