FROM alpine:latest

# 確認などに必要なパッケージのインストール
RUN apk add git curl

# hugo のインストール
RUN wget https://github.com/gohugoio/hugo/releases/download/v0.57.2/hugo_0.57.2_Linux-64bit.tar.gz
RUN tar -xzf hugo_0.57.2_Linux-64bit.tar.gz
RUN mv hugo /usr/local/bin/hugo

# 作業ディレクトリへ移動
WORKDIR /root/src/
