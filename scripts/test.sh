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
    print_run cargo +nightly doc --no-deps --features="nightly"
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
