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

int alloc4(int **p, int x,int y,int z , int a) {
    *p = malloc(sizeof(int)*4);
    (*p)[0] = x; (*p)[1] = y; (*p)[2] = z; (*p)[3] = a;
    return 1;
}
EOF

test() {
    local bin="./target/debug/rs9cc"
    if [ -n "$RS9CC_ON_WORKFLOW" ]; then
        bin="./target/release/rs9cc"
    fi

    $bin bin/test.c >$HOME/test.s
    cc -no-pie -o "${HOME}/test" $HOME/test.s
    $HOME/test
}

assert() {
    expected="$1"
    input="$2"

    local bin="./target/debug/rs9cc"
    if [ -n "$RS9CC_ON_WORKFLOW" ]; then
        bin="./target/release/rs9cc"
    fi
    echo "$input" >test.c
    $bin "test.c" >tmp.s
    cc -no-pie -o tmp tmp.s tmp2.o
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
    assert 0 ' int main ( ) {return 0;}'
    assert 4 'int main(){ return 4;}'
    assert 10 "int main() {return  4+9-3; }"
    assert 91 "int main(){return  4 +     90 -3;   }"
    assert 47 'int main(){return  5+6*7;}'
    assert 15 'int main(){ return 5*(9-6);}'
    assert 4 'int main(){return  (  3 +  5 )/2  ;}'
    assert 10 'int main(){return   -10 + 20 ;}'
    assert 100 'int main(){return  -(-  40) + 60;}'
}

# 2
eq() {
    assert 1 'int main(){return  0==0;}'
    assert 1 'int main(){return  -39 == -39;}'
    assert 0 'int main(){return  -210 == 932;}'
}

# 3
neq() {
    assert 1 'int main(){return 321!=4442;}'
    assert 0 'int main(){return 33!=33;}'
}

# 4
greater() {
    assert 1 'int main(){return  2 >   1  ; }'
    assert 0 'int main(){return  40 > 200;}'
    assert 0 'int main(){return 40>40;}'
}

# 5
lesser() {
    assert 1 'int main(){return 4<200;}'
    assert 0 'int main(){return  4000 < 500;}'
    assert 0 'int main(){return -40<-40;}'
}

# 6
leq() {
    assert 1 'int main(){return 0<=1;}'
    assert 1 'int main(){return 0 <= 0;}'
    assert 0 'int main(){return 4<= 0;}'
}

# 7
geq() {
    assert 1 'int main() {return 0>=0;}'
    assert 1 'int main() {return -11>=-11;}'
    assert 1 'int main() {return 100 >= 3;}'
    assert 0 'int main() {return 3 >= 100;}'
    assert 0 'int main() {return -100 >= 30;}'
}

# 8
single_char_variable() {
    assert 3 'int main(){int a;return a=3;}'
    assert 1 'int main(){int a;a = -4; int b;b= 5; return a+b;}'
    assert 2 'int main(){int a;a=1;int b;b=1;return a+b;}'
    assert 14 'int main(){int a; a =3 ;int b; b = 5*6-8; return a+b/2;}'
    assert 2 'int main(){int z; int h; int s;z=h=s=1;return z*(h+s);}'
}

# 9
multi_char_variable() {
    assert 2 'int main(){int foo;foo=1;int bar;bar=1;return foo+bar;}'
    assert 63 'int main(){int foo; int bar; foo  = 13 ; bar = 50 ;return  foo + bar ;}'
    assert 10 'int main(){int foo; int bar;foo = -1 ; bar = 9;return  foo*bar+bar*2+foo*-1;}'
    assert 18 'int main(){int foo; int bar; foo = -1 ; bar = 9; foo = foo +bar; return foo +10;}'
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
    assert 8 'int main(){int i; i = 1;  while (i <=1024) i = i + i; return i/256;}'
    assert 57 'int main(){int foo;int i; foo = 12;for(i = 0;i<10;i = i+1)foo = foo+i;return foo; }'
    assert 50 'int main(){int result; int i;result = 0;for(i=1;i<=100;i=i+1) result = result+i;return result/101;}'
}

# 12
block_stmt() {
    assert 4 'int main(){int foo; foo=1;{foo= foo+foo;foo=foo+foo;}return foo;}'
    assert 233 'int main(){int n ;n=13;int current; current = 0; int next; next = 1;int i; i = 0; int tmp; tmp = 0; while ( i < n ) { tmp = current; current = next; next = next + tmp; i=i+1;} return current;}'
    assert 233 'int main(){int n; int current; int next; int i;int tmp;n=13; current = 0;next = 1; for(i =0;i<n;i=i+1){tmp=current;current = next;next = next +tmp;}return current;}'
}

# 13
func_call() {
    assert 3 'int ret3();  int main(){return ret3();}'
    assert 3 'int ret3(); int main(){return ret3();}'
    assert 5 'int ret5(); int main(){return ret5();}'
    assert 8 'int add(int x, int y);  int main(){return add(3, 5);}'
    assert 2 'int sub(int x, int y); int main(){return sub(5, 3);}'
    assert 10 'int mul(int x, int y); int main(){return mul(2, 5);}'
    assert 6 'int add3(int x, int y, int z); int main(){return add3(1,2,3);}'
    assert 21 'int add6(int a, int b, int c ,int d, int e ,int f); int main(){return add6(1,2,3,4,5,6);}'
}

# 14
zero_arity_func_def() {
    assert 3 'int myfunc(){return 3;}int main(){return myfunc();}'
    assert 33 'int myfunc(){int a; int b;a = 1; b =2; return a+b;} int main(){int a; int b;a = 10; b = 20; return a + b + myfunc();}'
    # assert 8 'int main(){int foo; foo = 10; int bar; bar = 20; return -1 - foo + bar + myfunc();} int myfunc () {int foo; foo = -1; return foo;}'
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
    assert 5 'int main() { int x; int y; x=3; y=5; return *(&x-1); }' # コンパイラ依存
    assert 3 'int main() { int x; int y; x=3; y=5; return *(&y+1); }' # コンパイラ依存
    assert 5 'int main() { int x; int *y; x=3; y=&x; *y=5; return x; }'
    assert 7 'int main() { int x; int y; x=3; y=5; *(&x-1)=7; return y; }' # コンパイラ依存
    assert 7 'int main() { int x; int y; x=3; y=5; *(&y+1)=7; return x; }' # コンパイラ依存
}

# 17
int_keyword() {
    assert 1 'int foo(int x) {int intx; return x;} int main() { return foo(1);}'
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
    assert 1 'int alloc4(int *p,int x,int y, int z,int a);int main(){int *p; alloc4(&p,1,2,4,8); return *p;}'
    assert 1 'int alloc4(int *p,int x,int y, int z,int a);int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p;return *q;}'
    assert 4 'int alloc4(int *p,int x,int y, int z,int a);int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p+2;return *q;}'
    assert 8 'int alloc4(int *p,int x,int y, int z,int a);int main(){int *p; alloc4(&p,1,2,4,8); int *q; q = p+3;return *q;}'
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

# 24
char() {
    assert 1 'int main(){char a; a = 1; return a;}'
    assert 2 'int main(){char a; int b; a =1; b =a +1; return b;}'
    assert 10 'int main(){char hoge[10]; hoge[9] = 10; return hoge[9];}'
    assert 3 'int main(){char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; return x[0] + y;}'
    assert 5 'int main(){char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; return y - x[0];}'
    assert 10 'char hoge[2]; int main(){hoge[0] =1; hoge[hoge[0]]= 10; return hoge[1];}  '

    assert 97 "int main(){char p = 'a';return p; }"
    assert 10 "int main(){return '\\n';}"
}

# 25
string() {
    assert 97 'int main(){return "abc"[0];}'
    assert 97 'int main() { return "abc"[0]; }'
    assert 98 'int main() { return "abc"[1]; }'
    assert 99 'int main() { return "abc"[2]; }'
    assert 100 'int main() { return "abcd"[3]; }'
    assert 4 'int main() { return sizeof("abc"); }'
    assert 12 'int printf(char *x); int main(){return printf("hello world!"); }'
    assert 6 'int printf(char *x); int main(){printf("hello world!\n");return printf(" oops\\"); }'
    assert 6 'int main(){char p[] = "hello"; return sizeof p;}'
}

# 26
init() {
    assert 1 'int main(){int x = 1; return x;}'
    assert 1 'int main(){int x = 1; int *y = &x; return *y;}'
    assert 3 'int main(){int x[2] = {1,2}; return x[0]+x[1];} '
    assert 19 'int main(){int x[10] = {10,9}; int result = 0; int i=0; for ( i ; i< 10; i = i+1){result = result +x[i];}return result;}'
    assert 0 'int main(){int x[2] = {}; return x[0]+x[1];}'
    assert 99 'int printf(char *x); int main(){char p[10] = "cello";return p[0]; }'
    assert 111 'int main(){char *p = "hello"; return p[4];}'
    assert 3 'int three() { return 3; }int arity(int x) { return x; }int main() { return arity(three()); }'
    assert 0 'int printf(char *x); int main(){char p[10] = "cello";return p[9]; }'
    assert 5 'int printf(char *x); int main(){char p[10] = "hello";return printf(p); }'
    assert 19 'int main(){int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}return result;}'
    assert 5 'int printf(char *x); int main(){char p[] = "hello";return printf(p); }'
    assert 19 'int main(){int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}return result;}'
    assert 8 'int main(){int x[] = {1,2}; return sizeof (x);}'
    assert 19 'int main(){int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< sizeof(x)/4; i = i+1){result = result +x[i];}return result;}'
    assert 19 'int main(){int x[] = {10,9}; int result = 0;  for ( int i = 0 ; i< sizeof(x)/4; i = i+1){result = result +x[i];}return result;}'
    assert 10 'int a = 10; int main(){return a;}'
    # assert 10 'int a ; int y = a; int main(){return 1;}' # err
    assert 5 'int main(){int a = 5; int  *b = &a; return *b;}'
    assert 3 'int a[]= {1,2}; int main(){return a[1]+a[0];}'
    assert 3 'int a[3]= {1,2}; int main(){return a[1]+a[0]+a[2];}'
    assert 13 'int a[3]= {1,2}; int main(){a[2]=10;return a[1]+a[0]+a[2];}'
    assert 20 'int a =20; int *b = &a; int main(){return *b;}'
    assert 104 'char p[]="hello"; int main(){return p[0];}'
    assert 104 'char *p = "hello"; int main(){return p[0];}'
}

# 27
stmt_expr() {
    assert 0 'int main() { return ({ 0; }); }'
    assert 2 'int main() { return ({ 0; 1; 2; }); }'
    assert 1 'int main() { ({ 0; return 1; 2; }); return 3; }'
    assert 3 'int main() { return ({ int x=3; x; }); }'
    assert 1 'int main(){ return 0 + ({int x = 1; x;}); }'
}

# 28
var_scope() {
    assert 2 'int main(){int x =1; return ({int x = 2; x; }); }'
    assert 2 'int main() { int x=2; { int x=3; } return x; }'
    assert 2 'int main() { int x=2; { int x=3; } { int y=4; return x; }}'
    assert 3 'int main() { int x=2; { x=3; } return x; }'
}

# 29
multi_dimension_arr() {
    assert 0 'int main() { int x[2][3]; int *y=x; *y=0; return **x; }'
    assert 1 'int main() { int x[2][3]; int *y=x; *(y+1)=1; return *(*x+1); }'
    assert 2 'int main() { int x[2][3]; int *y=x; *(y+2)=2; return *(*x+2); }'
    assert 3 'int main() { int x[2][3]; int *y=x; *(y+3)=3; return **(x+1); }'
    assert 4 'int main() { int x[2][3]; int *y=x; *(y+4)=4; return *(*(x+1)+1); }'
    assert 5 'int main() { int x[2][3]; int *y=x; *(y+5)=5; return *(*(x+1)+2); }'
    assert 6 'int main() { int x[2][3]; int *y=x; *(y+6)=6; return **(x+2); }'
    assert 11 'int main(){int hoge[2][3]; hoge[0][0]=1;hoge[1][2]= 10;return hoge[0][0]+hoge[1][2];}'
    assert 72 'int main() {int hoge[2][3][4]; for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){hoge[i][j][k]=i+k+j;}}}  int result = 0;for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){result = result + hoge[i][j][k];}}} return result; }'
    assert 96 'int main(){int hoge[2][3][4]; return sizeof hoge;}'
    assert 48 'int main(){int hoge[2][3][4]; return sizeof hoge[0];}'
    assert 16 'int main(){int hoge[2][3][4]; return sizeof hoge[0][0];}'
    assert 4 'int main(){int hoge[2][3][4]; return sizeof hoge[0][0][0];}'
}

# 30
struct() {
    assert 8 'int main(){struct square {int x; int y;} square; return sizeof square;}'
    assert 3 'int main(){struct square {int x; int y;} square; square.x = 3; square.y = 2; return square.x;}'
    assert 2 'int main(){struct square {int x; int y;} square; square.x = 3; square.y = 2; return square.y;}'
    assert 6 'int main(){struct square {int x; int y;} square; square.x = 3; square.y = 2; return square.y *square.x;}'
    assert 6 'int main(){struct  {int x; int y;} square; square.x = 3; square.y = 2; return square.y *square.x;}'
    assert 80 'int main(){struct  subject {int math[10]; int English[10];} subject; return sizeof(subject);}'
    assert 1 'int main(){struct  subject {int math[10]; int English[10];} subject; subject.math[0]=1; return subject.math[0];}'
    assert 90 'int main(){struct  subject {int math[10]; int English[10];} subject; for(int i = 0; i < 10; i = i+1){subject.math[i]= i; subject.English[9-i]=i;} int result = 0;for(int i = 0;i<10;i=i+1){result = result + subject.math[i] + subject.English[i];} return result;}'
    assert 32 'int main(){ struct hoge {struct {int a; int b[10]; }hoge; int a;  } hoge; hoge.hoge.a = 19; hoge.hoge.b[0] = 1; hoge.hoge.b[2]= 2; hoge.hoge.b[9]=10;return hoge.hoge.a + hoge.hoge.b[0]+hoge.hoge.b[2] +hoge.hoge.b[9];}'
    assert 12 'int main(){struct hoge{int a; int b;}hoge[10]; hoge[1].a = 2; hoge[2].b =  10; return hoge[1].a + hoge[2].b;}'
    assert 8 'int main(){struct {char a; int b;}hoge; return sizeof(hoge);}'
    assert 16 'int main(){struct {char a; int b; char c; }hoge; return sizeof(hoge);}'
    assert 30 'int main(){struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;(*obj).x = 10;(*obj).y = 20; return a.x+a.y;}'
    assert 30 'int main(){struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;obj->x = 10;obj->y = 20; return a.x+a.y;}'
}

# 31
typedef() {
    assert 1 'int main(){typedef int INT; INT x = 1; return x;}'
    assert 1 'int main(){ struct hoge {int a;}; typedef struct hoge HOGE; HOGE x; x.a = 1; return x.a;}'
    assert 1 'int main(){typedef struct hoge {int a;} HOGE; HOGE x; x.a = 1; return x.a;}'
    assert 1 'int main(){typedef int t; t t = 1; return t;}'
    assert 2 'int main(){typedef struct {int a;} t; { typedef int t; } t x; x.a=2; return x.a; }'
}

# 32
short_long() {
    assert 2 'int main(){short a = 2; return a;}'
    assert 10 'int main(){long a = 10; return a;}'
    assert 2 'int main(){short a; return sizeof(a);}'
    assert 8 'int main(){long a; return sizeof(a);}'
    assert 20 'int main(){short a[10]; return sizeof a;}'
    assert 80 'int main(){long a[10]; return sizeof a;}'
    assert 1 'short sub_short(short a,  short c) {return a-c;} int main(){return sub_short(4,3);}'
    assert 1 'long sub_long(long a,  long c) {return a-c;} int main(){return sub_long(4,3);}'
    assert 1 'short rt_short(short a){return a;} int rt_int(int a){return a;}int main(){return rt_int(({rt_short(1);}));}'
    assert 1 'short sub_short(short a,  short c) {return a-c;} int main(){return sub_short(4,3);}'
    assert 20 'int test(int a, int b, int c){return c;} short ttt(){return 1;} int main(){return test(10,ttt(),20);}'
    assert 1 'short test(short a){return a;} int main(){return test(1)==1;}'
}

# 33
complex_type() {
    assert 24 'int main(){int *x[3]; return sizeof(x);}'
    assert 8 'int main(){int (*x)[3]; return sizeof(x);}'
    assert 3 'int main(){int *x[3]; int y; x[0]=&y; y=3; return x[0][0];}'
    assert 4 'int main(){int x[3]; int (*y)[3]=x; y[0][0]=4;return  y[0][0];}'

    assert 1 'int g = 1; int *test(){return &g;} int main(){return *test();}'
}

# 34
bool() {
    assert 0 'int main(){_Bool x = 0; return x;}'
    assert 1 'int main(){_Bool x = 1; return x;}'
    assert 1 'int main(){_Bool x = 2; return x;}'
    assert 1 'int main(){_Bool x = 2==2; return x;}'
    assert 0 'int main(){_Bool x = 2==3; return x;}'
    assert 1 'int main(){_Bool x = 1; return sizeof (_Bool);}'
}

# 35
complex_type2() {
    assert 1 'int main(){char x = 1; return sizeof x;}'
    assert 2 'int main(){short int x = 1; return sizeof(x);}'
    assert 2 'int main(){int short x = 1; return sizeof(x);}'
    assert 4 'int main(){int x = 1; return sizeof(x);}'
    assert 8 'int main(){long x = 1; return sizeof(x);}'
    assert 8 'int main(){long int x = 1; return sizeof(x);}'
    assert 8 'int main(){int long x = 1; return sizeof(x);}'
    assert 1 'int main(){char typedef CHAR; CHAR x = 1; return sizeof x;}'
    assert 4 'int main(){typedef A ; A x = 1; return sizeof x;}'
}

# 36
sizeof2() {
    assert 1 'int main(){return sizeof(char);}'
    assert 2 'int main(){return sizeof(short);}'
    assert 2 'int main(){return sizeof(short int);}'
    assert 2 'int main(){return sizeof(int short);}'
    assert 4 'int main(){return sizeof(int);}'
    assert 8 'int main(){return sizeof(long);}'
    assert 8 'int main(){return sizeof(long int);}'
    assert 8 'int main(){return sizeof(int long);}'
    assert 4 'int main(){return sizeof(0);}'

    assert 8 'int main(){return sizeof(4294967297);}'
}

# 37
cast() {
    assert 1 'int main(){ return (char)8590066177;}'
    assert 1 'int main(){ return (_Bool)1;}'
    assert 1 'int main(){ return (_Bool)2;}'
    assert 0 'int main(){ return (_Bool)(char)256;}'
    assert 1 'int main(){ return (long)1;}'
    assert 0 'int main(){ return (long)&*(int *)0;}'
    assert 5 'int main(){ int x=5; long y=(long)&x; return *(int*)y;}'
}

# 38
enum() {
    assert 0 'int main(){enum {zero,one,two}; return zero;}'
    assert 1 'int main(){enum {zero,one,two}; return one;}'
    assert 2 'int main(){enum {zero,one,two}; return two;}'
    assert 5 'int main(){enum {five=5,six,seven,}; return five;}'
    assert 6 'int main(){enum {five=5,six,seven,}; return six;}'
    assert 7 'int main(){enum {five=5,six,seven,}; return seven;}'
    assert 0 'int main(){enum{zero, ten = 10 , five = 5}; return zero;}'
    assert 10 'int main(){enum{zero, ten = 10 , five = 5}; return ten;}'
    assert 5 'int main(){enum{zero, ten = 10 , five = 5}; return five;}'
    assert 4 'int main(){enum hoge {zero} x; return sizeof(x);}'
    assert 4 'int main(){enum hoge {zero} ; enum hoge x; return sizeof(x);}'
}

# 39
static() {
    assert 1 'int count(){static int cnt; cnt = cnt+1; return cnt;} int main(){return count(); }'
    assert 3 'int count(){static int cnt; cnt = cnt+1; return cnt;} int main(){count();count();return count(); }'
}

# 40
comma() {
    assert 3 'int main(){return (1,2,3);}'
}

# 41
pp_mm() {
    assert 1 'int main(){int i =1; return i++;}'
    assert 2 'int main(){int i =1; return ++i;}'
    assert 1 'int main(){int i =1; return i--;}'
    assert 0 'int main(){int i =1; return --i;}'
    assert 2 'int main(){int i =1; i++; return i;}'
    assert 2 'int main(){int i =1; ++i; return i;}'
    assert 0 'int main(){int i =1; i--; return i;}'
    assert 0 'int main(){int i =1; --i; return i;}'
    assert 3 'int main(){int a[] = {1,3,5};int *p = a+1; return *p++;}'
    assert 4 'int main(){int a[] = {1,3,5};int *p = a+1; return ++*p;}'
    assert 3 'int main(){int a[] = {1,3,5};int *p = a+1; return *p--;}'
    assert 2 'int main(){int a[] = {1,3,5};int *p = a+1; return --*p;}'
    assert 5 'int main(){int a[] = {1,3,5};int *p = a+1; *p++; return *p;}'
    assert 1 'int main(){int a[] = {1,3,5};int *p = a+1; *--p; return *p;}'
}

# 42
cpx_assign() {
    assert 6 'int main(){int i = 3; i+=3; return i;}'
    assert 0 'int main(){int i = 3; i-=3; return i;}'
    assert 9 'int main(){int i = 3; i*=3; return i;}'
    assert 1 'int main(){int i = 3; i/=3; return i;}'
    assert 6 'int main(){int i = 3;return i+=3; }'
    assert 0 'int main(){int i = 3;return i-=3; }'
    assert 9 'int main(){int i = 3;return i*=3; }'
    assert 1 'int main(){int i = 3;return i/=3; }'
    assert 45 'int main(){int result = 0;for (int i =0;i<10 ;i++){result +=i;}return result;}'
}

# 43
not() {
    assert 1 'int main(){int i = 0; return !i;}'
    assert 0 'int main(){int i = 0; return !1;}'
    assert 0 'int main(){int i = 0; return !9;}'
    assert 1 'int main(){int i = 0; return !0;}'
}

# 44
bitnot() {
    assert 11 'int main(){int i =-12; return ~i;}'
    assert 1 'int main(){return ~~1;}'
}

# 45
bit_op() {
    assert 1 'int main(){return 1|0;}'
    assert 3 'int main(){return 2|1;}'
    assert 3 'int main(){return 3|1;}'
    assert 0 'int main(){return 1&0;}'
    assert 0 'int main(){return 2&1;}'
    assert 1 'int main(){return 3&1;}'
    assert 1 'int main(){return 1^0;}'
    assert 3 'int main(){return 2^1;}'
    assert 0 'int main(){return 0^0;}'
    assert 0 'int main(){return 5^5;}'
    assert 1 'int main(){return 1|1^2&0;}'
}

# 46
log_and_or() {
    assert 1 'int main(){return 1||0;}'
    assert 0 'int main(){return 0||0;}'
    assert 1 'int main(){return 1||(1-1)||0;}'
    assert 0 'int main(){return 0||(1-1)||0;}'
    assert 1 'int main(){return 2&&2;}'
    assert 0 'int main(){return 0&&2;}'
    assert 0 'int main(){return 2&&0;}'
    assert 0 'int main(){return 1&&(2-2)&&2;}'
}

# 47
fn_param_arr() {
    assert 0 'int arr_param(int x[]){return x[0];} int main(){int x[2] ={}; return arr_param(x);}'
    assert 3 'int arr_param(int x[]){return x[2];} int main(){int x[] ={1,2,3}; return arr_param(x);}'
}

# 48
incomplete_struct() {
    assert 8 'int main(){struct *foo; return sizeof foo;}'
    assert 8 'int main(){struct T *foo; struct T {int x;} ; return sizeof (struct T); }'
    assert 1 'int main(){struct T { struct T *next; int x; } a; struct T b; b.x=1; a.next=&b; return a.next->x;}'
}

# 49
break_fn() {
    assert 3 'int main(){int i = 0;for (; i<10; i++){if (i==3){break;}} return i;}'
    assert 1 'int main(){int i = 0; return i++ == 0;}'
    assert 0 'int main(){int i = 0; return ++i == 0;}'
    assert 0 'int main(){int i =0; if (i++ == 0){return 0;}else {return 1;}}'
    assert 4 'int main(){int i = 0;int j = 0; while(j<10) {if (i++==3)break; j++;} return i;}'
    assert 0 'int main(){int i = 0; for (;;)break; return i;}'
    assert 3 'int main(){int i = 0; for(;i<10;i++) { for (;;) break; if (i == 3) break; } return i;}'
    assert 1 'int main(){int i =0; for (;;){for (;;) break; i++; break;}return i;}'
    assert 4 'int main(){int i = 0; while(1) { while(1) break; if (i++==3)break;} return i;}'
}

# 50
_continue() {
    assert 10 'int main(){int i = 0; for (;i<10;i++){if (i==3)continue; if (i==3){return i;}} return i;}'
    assert 10 'int main(){int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; }return i;}'
    assert 6 'int main(){int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; }return j;}'
    assert 10 'int main(){int i=0; int j=0; for(;!i;) { for (;j!=10;j++) continue; break; } return j;}'
    assert 10 'int main(){int i = 0; while(i<10){if (i==3){i++;continue;} if (i==3){break;} i++;} return i;}'
    assert 11 'int main(){int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; } return i;}'
    assert 5 'int main(){int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; } return j;}'
    assert 11 'int main(){int i=0; int j=0; while(!i) { while (j++!=10) continue; break; } return j;}'
}

# 51
goto() {
    assert 3 'int main(){int i =0; goto a; a: i++; b: i++; c: i++; return i;}'
    assert 2 'int main(){int i =0; goto e; d: i++; e: i++; f: i++; return i;}'
    assert 1 'int main(){int i =0; goto j; g: i++; h: i++; j: i++; return i;}'
    assert 1 'int test(){a:return 0;} int main(){a:return 1;}'
}

# 52
switch() {
    assert 1 'int main(){int i = 0; switch(0){case 0: i = 1;break; case 1: i = 2;break; case 3: i=3;break;} return i;}'
    assert 6 'int main(){int i=0; switch(1) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } return i;}'
    assert 7 'int main(){int i=0; switch(2) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } return i;}'
    assert 1 'int main(){int i=1; switch(3) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } return i;}'
    assert 5 'int main(){int i=0; switch(0) { case 0:i=5;break; default:i=7; } return i;}'
    assert 7 'int main(){int i=0; switch(1) { case 0:i=5;break; default:i=7; } return i;}'
    assert 2 'int main(){int i = 0;switch(0){case 0: i++; case 1: i++;} return i;}'
    assert 20 'int main(){int i=0; switch(1) { case 0:i=5;break; default:i=7; switch(i){case 0: i = 11; default: i = 20;} } return i;}'
    assert 11 'int main(){int i = 0; switch(1){default: i = 10; case 0: i++;}return i;}'
    assert 9 'int main(){int i = 0; int j = 0;for(;i<10;i++){switch(i){case 5: break; default: j++;break;} if (j==5){ break;}  } return  i+j;}'
}

# 53
void_fn() {
    assert 0 'void void_fn(){} int main(){void_fn(); return 0;}'
}

# 54
_shift() {
    assert 1 'int main(){return 1<<0;}'
    assert 8 'int main(){return 1<<3;}'
    assert 10 'int main(){return 5<<1;}'
    assert 2 'int main(){return 5>>1;}'
    assert 1 'int main(){int i =1; i<<= 0; return i;}'
    assert 8 'int main(){int i =1; i<<= 3; return i;}'
    assert 10 'int main(){int i =5; i<<= 1; return i;}'
    assert 2 'int main(){int i =5; i>>= 1; return i;}'
}

# 55
ternary() {
    assert 2 'int main(){return 0?1:2;}'
    assert 1 'int main(){return 1?1:2;}'
}

# 56
const_expression() {
    assert 10 'int main(){enum { ten=1+2+3+4, }; return ten;}'
    assert 1 'int main(){int i=0; switch(3) { case 5-2+0*3: i++; } return i;}'
    assert 8 'int main(){int x[1+1]; return sizeof(x);}'
    assert 2 'int main(){char x[1?2:3]; return sizeof(x);}'
    assert 3 'int main(){char x[0?2:3]; return sizeof(x);}'
}

# 57
lvar_initialize() {
    assert 1 'int main(){int x[3]={1,2,3}; return x[0];}'
    assert 2 'int main(){int x[3]={1,2,3}; return x[1];}'
    assert 3 'int main(){int x[3]={1,2,3}; return x[2];}'
    assert 3 'int main(){int x[3]={1,2,3,}; return x[2];}'
    assert 2 'int main(){int x[2][3]={{1,2,3},{4,5,6}}; return x[0][1];}'
    assert 4 'int main(){int x[2][3]={{1,2,3},{4,5,6}}; return x[1][0];}'
    assert 6 'int main(){int x[2][3]={{1,2,3},{4,5,6}}; return x[1][2];}'
}

# 58
arr_zero_ini() {
    assert 2 'int main(){int x[2][3]={{1,2}}; return x[0][1];}'
    assert 0 'int main(){int x[2][3]={{1,2}}; return x[1][0];}'
    assert 0 'int main(){int x[2][3]={{1,2}}; return x[1][2];}'
}

# 59
string_arr_ini() {
    assert 104 'int main(){char p[6] = "hello"; return p[0];}'
    assert 108 'int main(){char p[6] = "hello"; return p[3];}'
    assert 0 'int main(){char p[6] = "hello"; return p[5];}'
    assert 97 'int main(){char x[2][4]={"abc","def"};return x[0][0];}'
    assert 0 'int main(){char x[2][4]={"abc","def"};return x[0][3];}'
    assert 100 'int main(){char x[2][4]={"abc","def"};return x[1][0];}'
    assert 102 'int main(){char x[2][4]={"abc","def"};return x[1][2];}'
}

# 60
unsized_arr() {
    assert 3 'int main(){int x[]={1,2,3}; return x[2];}'
    assert 16 'int main(){int x[]={1,2,3,4}; return sizeof x;}'
    assert 6 'int main(){char p[] = "Hello"; return sizeof p;}'
}

# 61
struct_ini() {
    assert 1 'int main(){ struct {int a; int b; int c;} x={1,2,3}; return x.a;}'
    assert 2 'int main(){ struct {int a; int b; int c;} x={1,2,3}; return x.b;}'
    assert 3 'int main(){ struct {int a; int b; int c;} x={1,2,3}; return x.c;}'
    assert 1 'int main(){struct {int a; int b; int c;} x={1}; return x.a;}'
    assert 0 'int main(){struct {int a; int b; int c;} x={1}; return x.b;}'
    assert 0 'int main(){struct {int a; int b; int c;} x={1}; return x.c;}'
    assert 1 'int main(){struct {int a; int b;} x[2]={{1,2},{3,4}}; return x[0].a;}'
    assert 2 'int main(){struct {int a; int b;} x[2]={{1,2},{3,4}}; return x[0].b;}'
    assert 3 'int main(){struct {int a; int b;} x[2]={{1,2},{3,4}}; return x[1].a;}'
    assert 4 'int main(){struct {int a; int b;} x[2]={{1,2},{3,4}}; return x[1].b;}'
    assert 0 'int main(){struct {int a; int b;} x[2]={{1,2}}; return x[1].b; }'
}

# 62
gvar_scalar_ini() {
    assert 0 'int x; int y; int main(){return x;}'
    assert 3 'char g = 3; int main(){return g;}'
    assert 3 'short g = 3; int main(){return g;}'
    assert 3 'int g = 3; int main(){return g;}'
    assert 3 'long g = 3; int main(){return g;}'
    assert 3 'int g1 = 3;int *g = &g1; int main(){return *g;}'
    assert 97 ' char *g = "abc"; int main(){return g[0];}'
}

# 63
gvar_arr_ini() {
    assert 2 'int g[3] = {0,1,2}; int main(){return g[2];}'
    assert 98 'char *g[] = {"foo","bar"}; int main(){return g[1][0];}'
    assert 3 'struct {char a; int b;}g[2]={{1,2},{3,4}}; int main(){return g[1].a;}'
    assert 4 'struct {int a[2];}g[2] = {{{1,2}}, {{3,4}}}; int main(){return g[1].a[1];}'
}

# 64
omit_paran() {
    assert 1 'struct {int a[2];}g[2] = {{1,2},3,4}; int main(){return g[0].a[0];}'
    assert 2 'struct {int a[2];}g[2] = {{1,2},3,4}; int main(){return g[0].a[1];}'
    assert 3 'struct {int a[2];}g[2] = {{1,2},3,4}; int main(){return g[1].a[0];}'
    assert 4 'struct {int a[2];}g[2] = {{1,2},3,4}; int main(){return g[1].a[1];}'

    assert 1 'struct {int a[2];}g[2] = {1,2,3,4}; int main(){return g[0].a[0];}'
    assert 2 'struct {int a[2];}g[2] = {1,2,3,4}; int main(){return g[0].a[1];}'
    assert 3 'struct {int a[2];}g[2] = {1,2,3,4}; int main(){return g[1].a[0];}'
    assert 4 'struct {int a[2];}g[2] = {1,2,3,4}; int main(){return g[1].a[1];}'

    assert 102 'char *g = {"foo"}; int main(){return g[0];}'
    assert 102 "char g[][4] = {'f', 'o', 'o', 0, 'b', 'a', 'r', 0}; int main(){return g[0][0];}"
    assert 0 "char g[][4] = {'f', 'o', 'o', 0, 'b', 'a', 'r', 0}; int main(){return g[0][3];}"
    assert 98 "char g[][4] = {'f', 'o', 'o', 0, 'b', 'a', 'r', 0}; int main(){return g[1][0];}"
    assert 0 "char g[][4] = {'f', 'o', 'o', 0, 'b', 'a', 'r', 0}; int main(){return g[1][3];}"

}

build() {
    cargo build
}

if [ -z "$RS9CC_ON_WORKFLOW" ]; then
    build
fi

if [ $# -eq 0 ]; then
    test
    __my_code=$?
    echo ""
    echo "exit code: $__my_code"
    exit $__my_code
fi

if [ "$1" == "sh" ]; then
    shift
fi

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
    char
    string
    init
    stmt_expr
    var_scope
    multi_dimension_arr
    struct
    typedef
    short_long
    complex_type
    bool
    complex_type2
    sizeof2
    cast
    enum
    static
    comma
    pp_mm
    cpx_assign
    not
    bitnot
    bit_op
    log_and_or
    fn_param_arr
    incomplete_struct
    break_fn
    _continue
    goto
    switch
    void_fn
    _shift
    ternary
    const_expression
    lvar_initialize
    arr_zero_ini
    string_arr_ini
    unsized_arr
    struct_ini
    gvar_scalar_ini
    gvar_arr_ini
    omit_paran
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
    "24") char ;;
    "25") string ;;
    "26") init ;;
    "27") stmt_expr ;;
    "28") var_scope ;;
    "29") multi_dimension_arr ;;
    "30") struct ;;
    "31") typedef ;;
    "32") short_long ;;
    "33") complex_type ;;
    "34") bool ;;
    "35") complex_type2 ;;
    "36") sizeof2 ;;
    "37") cast ;;
    "38") enum ;;
    "39") static ;;
    "40") comma ;;
    "41") pp_mm ;;
    "42") cpx_assign ;;
    "43") not ;;
    "44") bitnot ;;
    "45") bit_op ;;
    "46") log_and_or ;;
    "47") fn_param_arr ;;
    "48") incomplete_struct ;;
    "49") break_fn ;;
    "50") _continue ;;
    "51") goto ;;
    "52") switch ;;
    "53") void_fn ;;
    "54") _shift ;;
    "55") ternary ;;
    "56") const_expression ;;
    "57") lvar_initialize ;;
    "58") arr_zero_ini ;;
    "59") string_arr_ini ;;
    "60") unsized_arr ;;
    "61") struct_ini ;;
    "62") gvar_scalar_ini ;;
    "63") gvar_arr_ini ;;
    "64") omit_paran ;;
    esac
    shift
done

echo OK
