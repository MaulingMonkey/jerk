export RUST_BACKTRACE=1

function main {
    export ROOT=`pwd`
    build   || exit 1
    test    || exit 1
    doc     || exit 1
}

function build {
    print_run cargo build --all
}

function test {
    # set necessary env vars for metabuild script in Readme.md
    export PROFILE="debug"
    export OUT_DIR=target/readme
    print_run cargo test --all
    ERR=$?
    export OUT_DIR=
    export PROFILE=
    exit $ERR
}

function doc {
    if [[ -z "${CI}" ]]; then
        # Not a CI build, assume you have a sane nightly installed
        print_run cargo +nightly doc --no-deps --features="nightly" || exit 1

    elif [ "${RUSTUP_TOOLCHAIN}" = "nightly" ]; then
        # CI, but nightly
        print_run cargo doc --no-deps --features="nightly" || exit 1

    else
        # CI, !nightly
        print_run cargo doc --no-deps || exit 1

    fi
}

function print_run {
    printf "\033[1;32m$(whoami)@$(hostname)\033[0m:\033[1;34m$(pwd)\033[0m$ "
    echo "$@"
    "$@"
}

pushd `dirname $0`/.. >/dev/null
main
ERR=$?
popd >/dev/null
exit $ERR
