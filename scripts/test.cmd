@pushd "%~dp0.." && setlocal

@set JAVA_HOME=C:\Program Files (x86)\Java\jdk1.8.0_231
cargo build --all --target=i686-pc-windows-msvc
cargo test  --all --target=i686-pc-windows-msvc

@set JAVA_HOME=
cargo build --all --target=x86_64-pc-windows-msvc
cargo test  --all --target=x86_64-pc-windows-msvc

@set RUSTUP_TOOLCHAIN=
@call :doc "%~dp0..\jerk"
@call :doc "%~dp0..\jerk-build"
@call :doc "%~dp0..\jerk-test"

@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh

@popd && endlocal && goto :EOF



:doc
@pushd "%~1"
cargo +nightly doc --no-deps --features="nightly"
@popd && exit /b %ERRORLEVEL%
