# Rust

## version check

```.bash
$ docker-compose run rust rustc --version
```

or

```.bash
$ # Git Bash の場合は wimpty を先頭につける
$ winpty docker exec -it rust_rust_1 bash
$ rustc --version
```

## build

```
$ cargo build --release
$ cargo run
```
