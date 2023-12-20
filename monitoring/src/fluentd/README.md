fluentd
===

https://www.fluentd.org/

データコレクタ。

## Document

* 設定ファイル
    * https://docs.fluentd.org/configuration/config-file

## 環境構築

### Docker with Debian Image

```bash
docker run --rm -it -v "$(pwd)/src/hello_world/fluent/:/opt/fluent/" debian:12 bash
apt update -y && apt upgrade -y && apt install -y rbenv ruby-dev
rbenv install 3.1.2
gem install fluentd -v 1.16.1 -N

# 設定ファイルの作成
# fluentd --setup /opt/fluent

fluentd -c /opt/fluent/fluent.conf -vv
```

### Docker with fluentd Image

```bash
docker run --rm -it -p "8888:8888" -v "$(pwd)/src/hello_world/fluent/:/opt/fluent/" fluent/fluentd:v1.16-1 fluentd -c /opt/fluent/fluent.conf -vv
```
