CMake Build
===

CMakeとbindgenでビルドするサンプル。
bindgenはLLVM周りが必要になるので注意。

```bash
docker build . -t cmake_build:latest
docker run --rm -v "$(pwd):/opt/cmake_build/" -w /opt/cmake_build/ -it cmake_build:latest bash
```
