

## Build

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

# Linuxディストリビュージュン上でコンパイルしたい場合は以下のような環境構築が必要
apt install -y gcc

cargo build --manifest-path ./galm_wrapper/Cargo.toml --release
CGO_ENABLED="1" CC="gcc" go build --ldflags '-linkmode external -extldflags "-static"' ./main.go

# macOS上でLinux向けにコンパイルしたい場合は以下のような環境構築が必要
brew install FiloSottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl

cargo build --manifest-path ./galm_wrapper/Cargo.toml --release --target x86_64-unknown-linux-musl
CGO_ENABLED="1" GOOS="linux" CC="x86_64-linux-musl-gcc" go build --ldflags '-linkmode external -extldflags "-static"' ./main.go
```

## Test

```bash
go test
```
