#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/x86_64-unknown-linux-musl/debug/rs9cc "$input" >tmp.s
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
assert 4 4
assert 10 "4+9-3"
assert 91 " 4 +     90 -3"

echo OK
