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
protoc --go_out=golang/sample --go_opt=paths=source_relative \
    --go-grpc_out=golang/sample --go-grpc_opt=paths=source_relative \
    ./sample.proto

## c++
protoc -I ./ --grpc_out=cxx --plugin=protoc-gen-grpc=/home/grpc/cmake/build/grpc_cpp_plugin ./sample.proto
protoc -I ./ --cpp_out=cxx ./sample.proto

# ドキュメント作成
protoc --doc_out=html,index.html:./ *.proto

# => 作成したファイルを利用してクライアントとサーバを作成
```


## Run

```bash
# server

## golang
go run /home/src/sample/golang/server.go

## c++
mkdir -p /home/src/sample/cxx/build &&  cd /home/src/sample/cxx/build && \
    rm -rf ./* && CC=gcc CXX=g++ cmake -G Ninja .. && ninja -j4 && ./server

# client
mkdir -p /home/src/sample/cxx/build &&  cd /home/src/sample/cxx/build && \
    rm -rf ./* && CC=gcc CXX=g++ cmake -G Ninja .. && ninja -j4 && ./client

## golang
go run /home/src/sample/golang/client.go
```
