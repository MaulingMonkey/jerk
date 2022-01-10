@pushd "%~dp0.." && setlocal

cargo build --all --target=x86_64-pc-windows-msvc   || goto :die-pop1
cargo build --all --target=i686-pc-windows-msvc     || goto :die-pop1

:: set necessary env vars for metabuild script in Readme.md
@set PROFILE=debug
@set OUT_DIR=target\readme
cargo test  --all --target=x86_64-pc-windows-msvc   || goto :die-pop1
::cargo test  --all --target=i686-pc-windows-msvc     || goto :die-pop1
@set OUT_DIR=
@set PROFILE=


cargo +nightly doc --no-deps --features="nightly" || goto :die-pop1

@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh || goto :die-pop1

@popd && endlocal && exit /b 0

:die-pop1
@set OUT_DIR=
@set PROFILE=
@popd && endlocal && exit /b 1
