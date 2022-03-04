
REM comment

@REM `@` を付けると対象のコマンドが実行ログに表示されなくなる

@REM `echo off` で以降のコマンドが実行ログが表示されなくなる
@echo off


@REM 標準出力
echo Hello, Batchfile!
@REM ... Hello, Batchfile!


@REM 変数
set value=sample


@REM 変数値の参照
echo %value%
@REM ... sample


@REM 四則演算
set /a culc_result= 2 + 3 * 3 / 2 - 0
echo %culc_result%
@REM ... 6


@REM 条件分岐
@REM   等価: equ
@REM   不等: neq
@REM   以下: leq
@REM   以上: geq
@REM   未満: lss
@REM   超過: gtr
if 1 equ 1 (
  echo ok
) else (
  echo no good
)
@REM ... ok


@REM 繰り返し
@REM for [オプション] [変数] (初期値,ステップ値,終了値) do (処理)
for /l %%i in (1,1,5) do (
  echo count: %%i
  @REM ... count: 1
  @REM ... count: 2
  @REM ... count: 3
  @REM ... count: 4
  @REM ... count: 5
)


@REM カレントディレクトリの情報を表示
dir . /b
@REM ... カレントディレクトリのファイルとディレクトリ一覧


@REM 関数（アセンブリのラベルっぽい）
call :sample_function 16
echo %errorlevel%
@REM ... 24
exit /b

:sample_function
  echo %1
  @REM ... 16
  exit /b 24
