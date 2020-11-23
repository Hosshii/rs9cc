#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    local bin="./target/x86_64-unknown-linux-musl/debug/rs9cc"
    if [ -n "$RS9CC_ON_WORKFLOW" ]; then
        bin="./target/release/rs9cc"
    fi
    $bin "$input" >tmp.s
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

assert 0 '0;'
assert 4 '4;'
assert 10 "4+9-3;"
assert 91 " 4 +     90 -3;   "
assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 ' (  3 +  5 )/2  ;'
assert 10 '  -10 + 20 ;'
assert 100 ' -(-  40) + 60;'

assert 1 '0==0;'
assert 1 ' -39 == -39;'
assert 0 '-210 == 932;'

assert 1 '321!=4442;'
assert 0 '33!=33;'

assert 1 ' 2 >   1  ; '
assert 0 ' 40 > 200;'
assert 0 '40>40;'

assert 1 '4<200;'
assert 0 ' 4000 < 500;'
assert 0 '-40<-40;'

assert 1 '0<=1;'
assert 1 '0 <= 0;'
assert 0 '4<= 0;'

assert 1 '0>=0;'
assert 1 '-11>=-11;'
assert 1 '100 >= 3;'
assert 0 '3 >= 100;'
assert 0 '-100 >= 30;'

assert 3 'a=3;'
assert 1 'a = -4; b= 5; a+b;'
assert 2 'a=1;b=1;a+b;'
assert 14 'a =3 ; b = 5*6-8; a+b/2;'
assert 2 'z=h=s=1;z*(h+s);'

assert 2 'foo=1;bar=1;foo+bar;'
assert 63 ' foo  = 13 ; bar = 50 ; foo + bar ;'
assert 10 'foo = -1 ; bar = 9; foo*bar+bar*2+foo*-1;'
assert 18 ' foo = -1 ; bar = 9; foo = foo +bar; foo +10;'

assert 1 'return 1;'
assert 11 'foo = 1; bar = 10  ; return foo + bar;'
assert 11 'foo = 1; bar = 10  ; return foo + bar; hoge = 20;'

assert 10 'if ( 1 ==1 ) return 10;'
assert 20 'foo = 10;bar = 20; if (foo == bar ) return foo; else return bar;'
assert 10 'i = 0; while(i <10) i = i + 1; return i;'
assert 8 'i = 1;  while (i <=1024) i = i + i; i/256;'
assert 57 'foo = 12;for(i = 0;i<10;i = i+1)foo = foo+i;return foo; '
assert 50 'result = 0;for(i=1;i<=100;i=i+1) result = result+i;return result/101;'

echo OK
