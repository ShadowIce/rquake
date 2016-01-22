@echo off
setlocal enableextensions enabledelayedexpansion

rem Only works if "release" is not part of any path/parameter in debug builds.

for %%i in (%*) do (
    set t1=%%i
	set t2=!t1:release=foundit!
	if not !t1!==!t2! goto :release
)
endlocal


:debug
gcc -static-libgcc res/rquake.res %*
goto :eof

:release
gcc -mwindows -static-libgcc res/rquake.res %*
goto :eof
