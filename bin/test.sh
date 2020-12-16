#!/bin/bash
cat <<EOF | gcc -xc -c -o tmp2.o -
#include <stdlib.h>
#include <stdio.h>
int ret3() { return 3; }
int ret5() { return 5; }

int add(int x, int y) { return x+y; }
int sub(int x, int y) { return x-y; }
int mul(int x, int y) { return x*y; }
int add3(int a,int b, int c){
    return a+b+c;
}

int add6(int a, int b, int c, int d, int e, int f) {
  return a+b+c+d+e+f;
}

void alloc4(int **p, int x,int y,int z , int a) {
    *p = malloc(sizeof(int)*4);
    (*p)[0] = x; (*p)[1] = y; (*p)[2] = z; (*p)[3] = a;
}
EOF

assert() {
    expected="$1"
    input="$2"

    local bin="./target/x86_64-unknown-linux-musl/debug/rs9cc"
    if [ -n "$RS9CC_ON_WORKFLOW" ]; then
        bin="./target/release/rs9cc"
    fi
    $bin "$input" >tmp.s
    cc -o tmp tmp.s tmp2.o
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

# 1
four_op() {
    assert 0 ' int main ( ) { 0;}'
    assert 4 'int main(){ 4;}'
    assert 10 "int main() { 4+9-3; }"
    assert 91 "int main(){ 4 +     90 -3;   }"
    assert 47 'int main(){ 5+6*7;}'
    assert 15 'int main(){ 5*(9-6);}'
    assert 4 'int main(){ (  3 +  5 )/2  ;}'
    assert 10 'int main(){  -10 + 20 ;}'
    assert 100 'int main(){ -(-  40) + 60;}'
}

# 2
eq() {
    assert 1 'int main(){ 0==0;}'
    assert 1 'int main(){ -39 == -39;}'
    assert 0 'int main(){ -210 == 932;}'
}

# 3
neq() {
    assert 1 'int main(){321!=4442;}'
    assert 0 'int main(){33!=33;}'
}

# 4
greater() {
    assert 1 'int main(){ 2 >   1  ; }'
    assert 0 'int main(){ 40 > 200;}'
    assert 0 'int main(){40>40;}'
}

# 5
lesser() {
    assert 1 'int main(){4<200;}'
    assert 0 'int main(){ 4000 < 500;}'
    assert 0 'int main(){-40<-40;}'
}

# 6
leq() {
    assert 1 'int main(){0<=1;}'
    assert 1 'int main(){0 <= 0;}'
    assert 0 'int main(){4<= 0;}'
}

# 7
geq() {
    assert 1 'int main() {0>=0;}'
    assert 1 'int main() {-11>=-11;}'
    assert 1 'int main() {100 >= 3;}'
    assert 0 'int main() {3 >= 100;}'
    assert 0 'int main() {-100 >= 30;}'
}

# 8
single_char_variable() {
    assert 3 'int main(){int a;a=3;}'
    assert 1 'int main(){int a;a = -4; int b;b= 5; return a+b;}'
    assert 2 'int main(){int a;a=1;int b;b=1;a+b;}'
    assert 14 'int main(){int a; a =3 ;int b; b = 5*6-8; a+b/2;}'
    assert 2 'int main(){int z; int h; int s;z=h=s=1;z*(h+s);}'
}

# 9
multi_char_variable() {
    assert 2 'int main(){int foo;foo=1;int bar;bar=1;foo+bar;}'
    assert 63 'int main(){int foo; int bar; foo  = 13 ; bar = 50 ; foo + bar ;}'
    assert 10 'int main(){int foo; int bar;foo = -1 ; bar = 9; foo*bar+bar*2+foo*-1;}'
    assert 18 'int main(){int foo; int bar; foo = -1 ; bar = 9; foo = foo +bar; foo +10;}'
}

# 10
return_stmt() {
    assert 1 'int main(){return 1;}'
    assert 11 'int main(){int foo; foo = 1;int bar; bar = 10  ; return foo + bar;}'
    assert 11 'int main(){int foo; foo = 1;int bar; bar = 10  ; return foo + bar; int hoge;hoge = 20;}'
}

# 11
control_stmt() {
    assert 10 'int main(){if ( 1 ==1 ) return 10;}'
    assert 20 'int main(){int foo; foo = 10;int bar; bar = 20; if (foo == bar ) return foo; else return bar;}'

    assert 10 'int main(){int i; i = 0; while(i <10) i = i + 1; return i;}'
    assert 8 'int main(){int i; i = 1;  while (i <=1024) i = i + i; i/256;}'
    assert 57 'int main(){int foo;int i; foo = 12;for(i = 0;i<10;i = i+1)foo = foo+i;return foo; }'
    assert 50 'int main(){int result; int i;result = 0;for(i=1;i<=100;i=i+1) result = result+i;return result/101;}'
}

# 12
block_stmt() {
    assert 4 'int main(){int foo; foo=1;{foo= foo+foo;foo=foo+foo;}foo;}'
    assert 233 'int main(){int n ;n=13;int current; current = 0; int next; next = 1;int i; i = 0; int tmp; tmp = 0; while ( i < n ) { tmp = current; current = next; next = next + tmp; i=i+1;} current;}'
    assert 233 'int main(){int n; int current; int next; int i;int tmp;n=13; current = 0;next = 1; for(i =0;i<n;i=i+1){tmp=current;current = next;next = next +tmp;}current;}'
}

# 13
func_call() {
    assert 3 'int main(){ret3();}'
    assert 3 'int main(){return ret3();}'
    assert 5 'int main(){return ret5();}'
    assert 8 'int main(){return add(3, 5);}'
    assert 2 'int main(){return sub(5, 3);}'
    assert 10 'int main(){return mul(2, 5);}'
    assert 6 'int main(){return add3(1,2,3);}'
    assert 21 'int main(){return add6(1,2,3,4,5,6);}'
}

# 14
zero_arity_func_def() {
    assert 3 'int myfunc(){3;}int main(){myfunc();}'
    assert 33 'int myfunc(){int a; int b;a = 1; b =2; return a+b;} int main(){int a; int b;a = 10; b = 20; return a + b + myfunc();}'
    assert 8 'int main(){int foo; foo = 10; int bar; bar = 20; return -1 - foo + bar + myfunc();} int myfunc () {int foo; foo = -1; return foo;}'
}

# 15
six_arity_func_def() {
    assert 11 'int myfunc(int x) {return x +1;}int main(){return myfunc(10);}'
    assert 15 'int myfunc(int x,int y,int z){int foo; foo=10;return x*2+y+z+foo;} int main(){int foo; foo = 1;return foo+myfunc(foo,foo,foo);}'
    assert 55 'int fib(int n){if (n == 0) {return 0;} else if (n == 1) {return 1;}else {return fib(n-1)+fib(n-2);}} int main(){return fib(10);}'
}

# 16
unary_deref_addr() {
    assert 1 'int main(){int foo; int *bar; foo=1; bar = &foo; return *bar;}'
    assert 2 'int main(){int foo; int *bar; foo=1; bar = &foo; return *bar+1;}'
    assert 3 'int main() {int x; x=3; return *&x; }'
    assert 3 'int main() {int x; x=3; int *y;y=&x;  int **z;z=&y; return **z; }'
    assert 5 'int main() { int x; int y; x=3; y=5; return *(&x-8); }'
    assert 3 'int main() { int x; int y; x=3; y=5; return *(&y+8); }'
    assert 5 'int main() { int x; int *y; x=3; y=&x; *y=5; return x; }'
    assert 7 'int main() { int x; int y; x=3; y=5; *(&x-8)=7; return y; }'
    assert 7 'int main() { int x; int y; x=3; y=5; *(&y+8)=7; return x; }'
}

# 17
int_keyword() {
    assert 1 'int foo(int x) {int intx; return x;} int main() { foo(1);}'
    assert 10 'int main(){int *a; int x; x = 10; a = &x; return *a; }'
    # assert 127 'int foo(int x){int x; return x;}'  this cause already defined error
}

# 18
pointer_type() {
    assert 3 'int main(){int x; int *y; y = &x; *y = 3; return x;}'
    assert 3 'int main() {int x; int *y; int **z; x = 3; y = &x; z = &y; return **z;}'
    assert 11 'int main(){int x; int *y; x = 1; y = &x; return *y + 10;}'
}

# 19
pointer_operation() {
    assert 1 'int main(){int *p; alloc4(&p,1,2,4,8); return *p;}'
    assert 1 'int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p;return *q;}'
    assert 4 'int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p+2;return *q;}'
    assert 8 'int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p+3;return *q;}'
}

# 20
sizeof() {
    assert 4 'int main(){return sizeof(1);}'
    assert 8 'int main(){int *p; return sizeof(p);}'
    assert 4 'int main() {return sizeof (1+2);} '
    assert 8 'int main(){int *p; int x ; x = 8; p = &x; return sizeof (p +2);}'
    assert 4 'int echo(int n){return n;} int main(){return sizeof(echo(1)); }'
    assert 4 'int main(){int *y; return sizeof *y;}'
}

# 21
array() {
    assert 1 'int main(){int a[1]; *a = 1; return *a;}'
    assert 1 'int main(){ int y[2]; *y = 10; int x; x = 1; return x;}'
    assert 10 'int main(){int x[10]; *x = 1; *(x+9) = 10; return *(x+9); }' # intのサイズは8だけどポインタ演算は4なので変になってる
    assert 2 'int main(){int a[2]; *a = 1; *(a+1) = 2; int *p ;p =a; return  *(p+1);}'
    assert 1 'int main(){int x ; x = 1; int y[2]; *(y+1) = 10; return  x;}'
    assert 11 'int main(){int x ; x = 1; int y[2]; *(y+1) = 10; return  *(y+1) + x;}'
    assert 8 'int main(){int x; x = 1; int y[10]; int i; for(i =0; i<10; i = i+1){*(y+i)=i;} int z ; z = 20; return x + *(y+7) ; }'
    assert 12 'int main(){int x[3]; return sizeof x;}'
    assert 24 'int main(){int *x[3]; return sizeof x;}'
}

# 22
array_idx() {
    assert 1 'int main(){int a[10]; a[1] = 1; return a[1];}'
    assert 32 'int main(){int a[10]; int i; i = 2; a[0]= 10; a[9] = 20; return i+ a[0] + a[9]; } '
    assert 45 'int main(){int a[10]; int i; for(i=0;i<10;i=i+1){a[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + a[i]; }return result    ; } '
    assert 10 'int main(){int hoge[2]; int x; x = 2; hoge[x-1] = 10; return hoge[1];}'
}

# 23
global_variable() {
    assert 2 'int main(){int a; a=2; return a;}'
    assert 1 'int a; int main(){a = 1; return 1;}'
    assert 1 ' int a[10]; int main(){a[0] = 1; return a[0];}'
    assert 45 'int a[10];int main(){int i; for(i=0;i<10;i=i+1){a[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + a[i]; }return result    ; } '
    assert 10 'int hoge[2]; int main(){ int x; x = 2; hoge[x-1] = 10; return hoge[1];}'
    assert 3 'int a; int b; int add_a_b(){a = 1; b = 2; return a+b;} int main(){add_a_b(); return a + b;} '
    assert 5 'int a; int b; int add_a_b(){a = 1; b = 2; return a+b;} int main(){ int a ; a = add_a_b(); return a + b ;}'
}

if [ $# -eq 0 ]; then
    four_op
    eq
    neq
    greater
    lesser
    leq
    geq
    single_char_variable
    multi_char_variable
    return_stmt
    control_stmt
    block_stmt
    func_call
    zero_arity_func_def
    six_arity_func_def
    unary_deref_addr
    int_keyword
    pointer_type
    pointer_operation
    sizeof
    array
    array_idx
    global_variable
fi

while [ $# -ne 0 ]; do
    case $1 in
    "1") four_op ;;
    "2") eq ;;
    "3") neq ;;
    "4") greater ;;
    "5") lesser ;;
    "6") leq ;;
    "7") geq ;;
    "8") single_char_variable ;;
    "9") multi_char_variable ;;
    "10") return_stmt ;;
    "11") control_stmt ;;
    "12") block_stmt ;;
    "13") func_call ;;
    "14") zero_arity_func_def ;;
    "15") six_arity_func_def ;;
    "16") unary_deref_addr ;;
    "17") int_keyword ;;
    "18") pointer_type ;;
    "19") pointer_operation ;;
    "20") sizeof ;;
    "21") array ;;
    "22") array_idx ;;
    "23") global_variable ;;
    esac
    shift
done

echo OK
