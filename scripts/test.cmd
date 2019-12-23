@pushd "%~dp0.." && setlocal
@call :build
@call :test
@call :doc "%~dp0..\jerk"
@call :doc "%~dp0..\jerk-build"
@call :doc "%~dp0..\jerk-test"
@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh
@popd && endlocal && goto :EOF



:build
cargo build --all
@exit /b %ERRORLEVEL%

:test
@set CLASSPATH=%~dp0..\target\debug\java\jars\example-hello-world-jar.jar
cargo test --all
@exit /b %ERRORLEVEL%

:doc
@pushd "%~1"
cargo +nightly doc --no-deps --features="nightly"
@popd && exit /b %ERRORLEVEL%
