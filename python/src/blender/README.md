Blender
===

Blender APIについてのメモ。

## 環境構築

Blenderへのパスが通っているかどうか確認すること。


### Windows

```
"C:\Program Files\Blender Foundation\Blender 2.81\blender.exe" --version
```


## Run

Pythonスクリプトを実行する。

```bash
# 新規にファイルを作成する場合
blender --background --python ./src/export_image/main.py
# => ./src/export_image/build/ にファイルと画像が書き出される

# 既存のファイルを流用する場合
blender --background --python ./src/sample_project/main.py ./src/sample_project/example.blend
```
