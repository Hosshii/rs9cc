#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    echo $input | ./target/x86_64-unknown-linux-musl/debug/rs9cc >tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42

echo OK
