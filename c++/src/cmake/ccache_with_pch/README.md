ccache with pch
===

## Setup

```bash
$ docker run --rm -it -v "$(pwd):/opt" ubuntu:20.04 bash
$ apt update -y && apt upgrade -y && apt install -y gcc g++ ninja-build cmake ccache

# or

$ apt update -y && apt upgrade -y && apt install -y clang-11 ninja-build cmake ccache
```

## Build

```bash
$ CC=clang CXX=clang++ cmake -S . -B ./build/ -G Ninja
$ time cmake --build ./build/

# or

$ rm -rf ./build/ && cmake -S . -B ./build/ -G Ninja && time cmake --build ./build/ -- -v && ccache -sv
```

キャッシュが効かずUncacheableが発生する場合がある？
Clangだと大丈夫そうだけどMissは変わらず発生している。。

```bash
# show ccache stats
$ ccache -s

# clear cache
$ ccache -C

# zero statistics counters ( reset counter )
$ ccache -z

# disable cache
$ export CCACHE_DISABLE=0

# enable cache
$ export CCACHE_DISABLE=true
```

```bash
# pure build
real    0m0.467s
user    0m0.686s
sys     0m0.056s

# pure build with precompiled header
real    0m2.166s
user    0m1.238s
sys     0m0.256s

# ccache ( first build )
real    0m0.535s
user    0m0.738s
sys     0m0.102s

# ccache ( second build )
real    0m0.126s
user    0m0.059s
sys     0m0.007s
```

プリコンパイル済みヘッダはもうちょっと大きくないと効果が出ないかも(´・ω・｀)
