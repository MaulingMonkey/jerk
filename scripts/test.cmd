@pushd "%~dp0.." && setlocal

cargo build --all --target=x86_64-pc-windows-msvc
cargo test  --all --target=x86_64-pc-windows-msvc

cargo build --all --target=i686-pc-windows-msvc
cargo test  --all --target=i686-pc-windows-msvc

@set RUSTUP_TOOLCHAIN=
@call :doc "%~dp0..\jerk"
@call :doc "%~dp0..\jerk-build"
@call :doc "%~dp0..\jerk-test"

@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh

@popd && endlocal && goto :EOF



:doc
@pushd "%~1"
cargo +nightly doc --no-deps
@popd && exit /b %ERRORLEVEL%
