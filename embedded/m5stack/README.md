M5Stack
===

## Arduino Studio

### 環境構築

1. 機材を購入
    * M5Stack Core2 IoT開発キット
        * https://www.switch-science.com/catalog/6530/
1. IDEをインストール
    * https://www.arduino.cc/en/software
1. CP2104 Driverをインストール
    * https://shop.m5stack.com/pages/download
1. M5Stack Core2をPCに繋ぐ
1. IDEの `ファイル > 環境設定 > 設定 > 追加のボードマネージャ` からURLを追加
    * https://m5stack.oss-cn-shenzhen.aliyuncs.com/resource/arduino/package_m5stack_index.json
1. IDEの `ツール > ボード > ボードマネージャ` で「M5Stack (by M5Stack official)」をインストール
1. IDEの `スケッチ > ライブラリをインクルード > ライブラリを管理` で「M5Core2 (by M5Stack)」をインストール
1. IDEの `ツール > ボード:"" > M5Stack Arduino > M5Stack-Core2` を選択
1. IDEの `ツール > シリアルポート:""` で適切なポートを選択
    * Windowsの場合は `スタート` を右クリックしたメニューの `デバイスマネージャー > [ユーザ名] > ポート(COMとLPT)` から確認できる


### 実行

1. IDEの `ファイル > 開く` から適切なファイルを開く
1. IDEの `スケッチ > マイコンボードに書き込む` を選択するとビルドが走る
1. 少し待つとM5Stack Core2上でプログラムが実行される。
