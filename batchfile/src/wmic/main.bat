@echo off

@REM wmic: Windows Management Instrumentation Command line interface.
@REM Windows 10 version 21H1 では非推奨となっている ( 代替は PowerShell の WMI )

@REM BIOS のシリアルナンバーを取得する
wmic bios get serialnumber
@REM ...
@REM   SerialNumber
@REM   0000000

@REM プロセス一覧の表示
wmic process get name,workingsetsize
@REM ...
@REM   System                          147456
@REM    :

@REM ディスク情報の表示
wmic logicaldisk get Caption,FileSystem,Freespace,Size
@REM ...
@REM   Caption  FileSystem  FreeSpace    Size
@REM   C:       NTFS        30156226560  119408750592
