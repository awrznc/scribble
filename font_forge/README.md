FontForge
===

フォントエディター及びスクリプト言語。

https://fontforge.org/docs/old/ja/scripting.html

FontForge's legacy scripting language なる独自の言語で自動化が可能。
Python で記述することも可能。


## Install

### Debian

```bash
apt update -y
apt install -y fontforge
```


## Run

```bash
fontforge -quiet -lang=ff -script ./src/hello_world/main.pe
```
