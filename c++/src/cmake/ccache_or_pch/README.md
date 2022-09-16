ccache or pch
===

## Setup

```bash
$ docker run --rm -it -v "$(pwd):/opt" debian:11 bash
$ apt update -y && apt upgrade -y && apt install -y gcc g++ ninja-build cmake ccache
```

## Build

```
$ cmake -S . -B ./build/ -G Ninja
$ time cmake --build ./build/
```

```bash
# show ccache stats
$ ccache -s

# clear cache
$ ccache -C

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
