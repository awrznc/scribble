Preset
===

VisualStudio の構成プロパティを切り替えるっぽいことができるやつ。

```shell
# プリセットのある場所まで移動
cd ./c++/src/cmake/preset/

# 利用できるプリセット確認
cmake --list-presets=all .

# Build x64
cmake --preset x64 --fresh
cmake --build --preset x64
./build/x64/Debug/piyo.exe
# => x64

# Build Win32
cmake --preset Win32 --fresh
cmake --build --preset Win32
./build/Win32/Debug/piyo.exe
# => Win32
```
