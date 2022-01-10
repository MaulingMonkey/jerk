@pushd "%~dp0.." && setlocal
@set PUBLISH_ARGS=%*
@if defined DELAY set /A DELAY = %DELAY% + 1
@call :publish "%~dp0../jerk" || goto :err
:err
@popd && endlocal && goto :EOF



:publish
@cd "%~1"
cargo publish %PUBLISH_ARGS%
@if ERRORLEVEL 1 exit /b %ERRORLEVEL%
@if defined DELAY ping localhost -n %DELAY% >NUL 2>NUL
@exit /b 0
