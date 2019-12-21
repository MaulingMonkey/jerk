export JAVA_HOME=${JAVA_HOME:-/usr/lib/jvm/java-8-openjdk-amd64}
export PATH=${JAVA_HOME}/bin/:${PATH}
export LIBRARY_PATH=${JAVA_HOME}/jre/lib/amd64/server/:${LIBRARY_PATH}
export LD_LIBRARY_PATH=${JAVA_HOME}/jre/lib/amd64/server/:${LD_LIBRARY_PATH}
export RUST_BACKTRACE=1

function main {
    export ROOT=`pwd`
    build           || exit 1
    test            || exit 1
    doc jerk        || exit 1
    doc jerk-build  || exit 1
    doc jerk-test   || exit 1
}

function build {
    print_run cargo build --all
}

function test {
    export CLASSPATH=${ROOT}/target/debug/java/jars/example-hello-world-jar.jar
    export PATH=${ROOT}/target/debug:${PATH}
    print_run cargo test --all || exit 1
}

function doc {
    pushd $1 >/dev/null
    if [[ -z "${CI}" ]]; then
        # Not a CI build, assume you have a sane nightly installed
        print_run cargo +nightly doc --no-deps --features="nightly" || (popd >/dev/null && exit 1)

    elif [ "${RUSTUP_TOOLCHAIN}" = "nightly" ]; then
        # CI, but nightly
        print_run cargo doc --no-deps --features="nightly" || (popd >/dev/null && exit 1)

    else
        # CI, !nightly
        print_run cargo doc --no-deps || (popd >/dev/null && exit 1)

    fi
    popd >/dev/null
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
