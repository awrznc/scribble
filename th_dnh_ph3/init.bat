@echo off
setlocal
set workdir=%~dp0
powershell -NoProfile -ExecutionPolicy Unrestricted %workdir%init.ps1
endlocal
exit
