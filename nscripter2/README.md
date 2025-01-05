NScripter2
===

2D ゲームエンジンです。

https://www.nscripter.com/

## Init

スクリプトを利用して実行環境を構築します。<br>
下記のコマンドを実行すると、 `.tmp` フォルダを作成し、ダウンロードしてきたものを配置します。

```powershell
.\init.bat
```


## Documents

NScripter2 のドキュメントはダウンロードしてきた zip ファイルの中に入っています。

```bat
start .\.tmp\NScripter2\マニュアル\index.html
```


## Run

```powershell
.\src\hello_world\run.bat
```


## Error

スクリプトを配置せずに `nscr2.exe` を実行すると以下のエラーが表示されます。

```txt
Luaスクリプトのエラーです。
root.lua:32: 実行エラー：BASICスクリプトが読み込まれていません。
```
