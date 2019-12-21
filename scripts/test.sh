export JAVA_HOME=${JAVA_HOME:-/usr/lib/jvm/java-8-openjdk-amd64}
export PATH=${JAVA_HOME}/bin/:${PATH}
export LIBRARY_PATH=${JAVA_HOME}/jre/lib/amd64/server/:${LIBRARY_PATH}
export LD_LIBRARY_PATH=${JAVA_HOME}/jre/lib/amd64/server/:${LD_LIBRARY_PATH}
export RUST_BACKTRACE=1

function main {
    pushd `dirname $0`/..
    export ROOT=`pwd`
    build
    test
    doc jerk
    doc jerk-build
    doc jerk-test
    popd
}

function build {
    print_run cargo build --all
}

function test {
    export CLASSPATH=${ROOT}/target/debug/java/jars/example-hello-world-jar.jar
    export PATH=${ROOT}/target/debug:${PATH}
    print_run cargo test --all
}

function doc {
    cd $1
    print_run cargo +nightly doc --no-deps --features="nightly"
}

function print_run {
    printf "\033[1;32m$(whoami)@$(hostname)\033[0m:\033[1;34m$(pwd)\033[0m$ "
    echo "$@"
    "$@"
}

main
