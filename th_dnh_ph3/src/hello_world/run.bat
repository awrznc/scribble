del /q %~dp0\..\..\.tmp\th_dnh_ph3\script\hello_world\*
rmdir %~dp0\..\..\.tmp\th_dnh_ph3\script\hello_world\

mkdir %~dp0\..\..\.tmp\th_dnh_ph3\script\hello_world\
copy %~dp0\*.ph3 %~dp0\..\..\.tmp\th_dnh_ph3\script\hello_world\
copy %~dp0\*.png %~dp0\..\..\.tmp\th_dnh_ph3\script\hello_world\

start %~dp0\..\..\.tmp\th_dnh_ph3\th_dnh.exe
