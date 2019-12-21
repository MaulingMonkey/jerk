@pushd "%~dp0.." && setlocal
@set PUBLISH_ARGS=%*
@if not defined JAVA_HOME set JAVA_HOME=C:\Program Files\Android\Android Studio\jre\
@if defined DELAY set /A DELAY = %DELAY% + 1
@set PATH=%JAVA_HOME%\jre\bin\server\;%PATH%
@call :publish "%~dp0../jerk"       || goto :err
@call :publish "%~dp0../jerk-build" || goto :err
@call :publish "%~dp0../jerk-test"  || goto :err
:err
@popd && endlocal && goto :EOF



:publish
@cd "%~1"
cargo publish %PUBLISH_ARGS%
@if ERRORLEVEL 1 exit /b %ERRORLEVEL%
@if defined DELAY ping localhost -n %DELAY% >NUL 2>NUL
@exit /b 0
