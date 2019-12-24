@pushd "%~dp0.." && setlocal

cargo +stable-x86_64 build --all
cargo +stable-x86_64 test  --all

cargo +stable-i686 build --all
cargo +stable-i686 test  --all

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
