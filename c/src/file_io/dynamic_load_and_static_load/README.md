Dynamic Load and Static Load
===

データ読み込み及び埋め込みのサンプル。

```bash
# ビルド
mkdir -p ./build
cc ./src/write_binary.c -o ./build/wb
./build/wb

cc ./src/write_header.c -o ./build/wh
./build/wh ./color.dat > ./src/color.h
# xxd -i ./color.dat > ./src/color.h

cc ./src/dynamic_read_binary.c -o ./build/drb
cc ./src/static_read_binary.c -o ./build/srb

# 動的読み込み
./build/drb

# 静的読み込み
./build/srb
```
