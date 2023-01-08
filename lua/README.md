Lua
===

スクリプト言語。

他の言語で書かれたソフトウェアと統合しやすいように設計されているらしい（いわゆるグルー言語）


## 環境構築

```bash
docker run --rm -it -v "$(pwd):/root/workspace" debian:11 bash
apt update -y && apt upgrade -y
apt install -y lua5.4
```


## Run

```bash
lua ./src/hello_world/main.lua
```
