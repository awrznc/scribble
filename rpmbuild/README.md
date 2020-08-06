# rpmbuild

## RPM Package Manager とは

レッドハットが開発したソフトウェアのパッケージを管理するためのシステムのことを示します。
ダウンロード、コンパイル、インストール、セットアップなど、その環境に合った様々なビルド工程を自動で行ってくれます。

## rpmbuild とは

rpmbuildは、バイナリパッケージとソースパッケージの両方のビルドに利用されるシステムです。
つまりRPM Packageを作成することができます。

## 環境構築

Dockerを利用して環境構築をします。

```.sh
$ docker-compose up -d --build
$ docker exec -it rpmbuild_server_1 /bin/bash
```

## build

cmakeコマンドを実行したカレントディレクトリ内に成果物が吐き出されます。
後々成果物を一括削除するためにディレクトリを作成してその中でコマンドを実行します。

```.sh
$ # 前準備
$ mkdir build
$ cd build

$ # ConfigureとGenerate
$ cmake ..

$ # build
$ cmake --build .

$ # 成果物の確認
$ ./main_app
```
