@pushd "%~dp0.." && setlocal
@call :build
@call :test
@call :doc "%~dp0..\jerk"
@call :doc "%~dp0..\jerk-build"
@call :doc "%~dp0..\jerk-test"
@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh
@popd && endlocal && goto :EOF



:build
@if not defined JAVA_HOME set JAVA_HOME=C:\Program Files\Android\Android Studio\jre\
@set PATH=%JAVA_HOME%\jre\bin\server\;%PATH%
cargo build --all
@exit /b %ERRORLEVEL%

:test
@set CLASSPATH=%~dp0..\target\debug\java\jars\example-hello-world-jar.jar
@set JAVA_OPTS=-Djava.class.path=%~dp0..\target\debug\java\jars\example-hello-world-jar.jar
@set PATH=%~dp0..\target\debug\;%PATH%
cargo test --all
@exit /b %ERRORLEVEL%

:doc
@pushd "%~1"
cargo +nightly doc --no-deps --features="nightly"
@popd && exit /b %ERRORLEVEL%
