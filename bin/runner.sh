#!/bin/bash

runner() {
    local target="$1"
    shift

    case $target in
    "test")
        docker run --rm -v $PWD:/rs9cc:cached -w /rs9cc rust ./bin/test.sh
        ;;
    "test_sh")
        docker run --rm -v $PWD:/rs9cc:cached -w /rs9cc rust ./bin/test.sh "sh" $@
        ;;
    "build") build ;;
    "execute") docker run --rm -v $PWD:/rs9cc -w /rs9cc gcc $@ ;;
    *) echo "no target found" ;;

    esac
}

runner "$@"
