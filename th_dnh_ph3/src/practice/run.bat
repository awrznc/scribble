del /q %~dp0\..\..\.tmp\th_dnh_ph3\script\practice\*
rmdir %~dp0\..\..\.tmp\th_dnh_ph3\script\practice\

mkdir %~dp0\..\..\.tmp\th_dnh_ph3\script\practice\
copy %~dp0\*.ph3 %~dp0\..\..\.tmp\th_dnh_ph3\script\practice\
copy %~dp0\*.png %~dp0\..\..\.tmp\th_dnh_ph3\script\practice\

start %~dp0\..\..\.tmp\th_dnh_ph3\th_dnh.exe
