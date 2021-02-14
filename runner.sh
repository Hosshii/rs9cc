#!/bin/bash

build() {
    docker build -t rs9cc .
}

runner() {
    local target="$1"
    shift

    case $target in
    "test")
        build && docker run rs9cc
        ;;
    "test_sh")
        build && docker run rs9cc "sh" $@
        ;;
    "build") build ;;
    "execute") docker run --rm -v $PWD:/rs9cc -w /rs9cc gcc $@ ;;
    *) echo "no target found" ;;

    esac
}

runner "$@"
