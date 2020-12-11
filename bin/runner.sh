#!/bin/bash
build() {
    cross build --target x86_64-unknown-linux-musl
}

runner() {
    local target="$1"
    shift
    local builded="./target/x86_64-unknown-linux-musl/debug/rs9cc"

    case $target in
    "test")
        build
        docker run --rm -v $PWD:/rs9cc -w /rs9cc gcc ./bin/test.sh $@
        ;;
    "build") build ;;
    "execute") docker run --rm -v $PWD:/rs9cc -w /rs9cc gcc $@ ;;
    *) echo "no target found" ;;

    esac
}

runner "$@"
