# Rust

## version check

```.bash
$ docker-compose run server /root/.cargo/bin/rustc --version
```

or

```.bash
$ # Git Bash の場合は wimpty を先頭につける
$ winpty docker exec -it rust_rust_1 bash --login
$ rustc --version
```

## build

### compile to linux

```
$ cargo build --release
$ cargo run
```

### compile to windows

```
$ cargo build --release --target x86_64-pc-windows-gnu
```
