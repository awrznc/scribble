# gRPC

異なるマシン上で動作するサービス間で情報をやり取りする手法のひとつ。


## Init

```bash
# *.protoファイルを作成する。
touch sample.proto

# 中身をよしなに書き換える
vi sample.proto

# .proto ファイルからserver、client、interface等のコードを生成する

## golang
protoc --go_out=plugins=grpc:golang/sample sample.proto

# ドキュメント作成
protoc --doc_out=html,index.html:./ *.proto

# => 作成したファイルを利用してクライアントとサーバを作成
```


## Run

```bash
# server

## golang
go run golang/server.go

# client

## golang
go run golang/client.go
```
