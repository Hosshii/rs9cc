#include "test.h"

int ret3(){return 3;}
int add(int x, int y) { return x + y; }
int sub(int x, int y) { return x - y; }
int mul(int x, int y) { return x * y; }
int add3(int a, int b, int c) { return a + b + c; }

int add6(int a, int b, int c, int d, int e, int f) {
  return a + b + c + d + e + f;
}
int myfunc(){int a; int b;a = 1; b =2; return a+b;}
int two_arity(int x, int y){int z = 10; return x + y + z;}
int six_arity(int a, int b, int c, int d, int e ,int f){return a + b + c + d + e + f;}
int multi_type_fn(int a, long b){return a;}
int multi_type_fn2(int a, long b){return b;}
int multi_type_fn3(char a, long b, short c, int d){return a-b+c*d;}
int fib(int n){
  if (n < 2) {
    return n;
  } else {
    return fib(n-1)+fib(n-2);
  }
}
// int* malloc(int x);
// int alloc4(int **p, int x,int y,int z , int a) {
//     *p = malloc(4*4);
//     (*p)[0] = x; (*p)[1] = y; (*p)[2] = z; (*p)[3] = a;
//     return 1;
// }
int echo(int x){return x;}
char echo2(char x){return x;}
short sub_short(short a, short b){return a-b;}
long sub_long(long a, long b){return a-b;}
char char_fn() { return 257; }
int count(){static int cnt; cnt = cnt+1; return cnt;}
int count2(){static int cnt=2; cnt = cnt+1; return cnt;}
int arr_param(int x[]) {return x[0];}
int label_test(){label_test: return 1;}
void void_fn(){}
void voidfn(void){}
static int static_fn(){}
void return_only(){return;}

int g_1;
int g_2;
int g_arr1[2] = {1,3};
int g_arr2[10];
char g_hoge1[10];
int g_3 = 10;
int *g_ptr1= &g_3;
char *hello = "hello";
char hello2[]="hello2";
int g_arr3[]= {1,2,3};
struct {char a;int b;}g4[2] = {{1,2},{3,4}};
struct {int a[2];}g5[2] = {{{1,2}}, {{3,4}}};
char *g6[] = {"foo","bar"}; 
struct {int a[2];} g7[2] = {{1, 2}, 3, 4};
struct {int a[2];} g8[2] = {1, 2, 3, 4};
char *g9 = {"foo"};
char g10[][4] = {'f', 'o', 'o', 0, 'b', 'a', 'r', 0};
char *g11 = hello2 + 0;
char *g12 = hello2 + 3;
char *g13 = hello2 - 3;
int g14 = 3;
int *g15 = &g14;


typedef struct Tree {
  int val;
  struct Tree *lhs;
  struct Tree *rhs;
} Tree;

Tree *tree = &(Tree){
  1,
  &(Tree){
    2,
    &(Tree){ 3, 0, 0 },
    &(Tree){ 4, 0, 0 },
  },
  0,
};

int main() {
  /*
  this is test script.
  */

  printf("\n-----  TEST  START  -----\n\n");
  // #1
  printf("#1\n");
  assert(0, 0, "0;");
  assert(4, 4, "4");
  assert(10, 4 + 9 - 3, "4 + 9 - 3");
  assert(91, 4 + 90 - 3, "4 + 90 - 3");
  assert(47, 5 + 6 * 7, "5 + 6 * 7");
  assert(15, 5 * (9 - 6), "5 * (9 - 6)");
  assert(4, (3 + 5) / 2, "(3 + 5) / 2");
  assert(10, -10 + 20, "-10 + 20");
  assert(100, -(-40) + 60, "-(-40) + 60");

  // #2
  printf("\n\n#2\n");
  assert(1, 0 == 0, "0==0");
  assert(1, -39 == -39, "-39==-39");
  assert(0, -210 == 932, "-210 == 932");

  // #3
  printf("\n\n#3\n");
  assert(1, 321 != 4442, "321!=4442");
  assert(0, 33 != 33, "33!=33");

  // #4
  printf("\n\n#4\n");
  assert(1, 2 > 1, " 2 >   1  ");
  assert(0, 40 > 200, " 40 > 200");
  assert(0, 40 > 40, "40>40");

  // #5
  printf("\n\n#5\n");
  assert(1, 4 < 200, "4<200");
  assert(0, 4000 < 500, " 4000 < 500");
  assert(0, -40 < -40, "-40<-40");

  // #6
  printf("\n\n#6\n");
  assert(1, 0 <= 1, "0<=1");
  assert(1, 0 <= 0, "0 <= 0");
  assert(0, 4 <= 0, "4<= 0");

  // #7
  printf("\n\n#7\n");
  assert(1, 0 >= 0, "0>=0");
  assert(1, -11 >= -11, "-11>=-11");
  assert(1, 100 >= 3, "100 >= 3");
  assert(0, 3 >= 100, "3 >= 100");
  assert(0, -100 >= 30, "-100 >= 30");

  // #8
  printf("\n\n#8\n");
  assert(3,({int a;a=3;}), "({int a;a=3;})");
  assert(1,({int a;a = -4; int b;b= 5;  a+b;}), "({int a;a = -4; int b;b= 5;  a+b;})");
  assert(2, ({int a;a=1;int b;b=1;a+b;}),"({int a;a=1;int b;b=1;a+b;})");
  assert(14, ({int a; a =3 ;int b; b = 5*6-8; a+b/2;}),"({int a; a =3 ;int b; b = 5*6-8; a+b/2;})");
  assert(2, ({int z; int h; int s; z=h=s=1; z*(h+s); }),"({int z; int h; int s;z=h=s=1;z*(h+s);)}");

  // #9
  printf("\n\n#9\n");
  assert( 2, ({int foo;foo=1;int bar;bar=1; foo+bar;}), "({int foo;foo=1;int bar;bar=1; foo+bar;})");
  assert( 63,({int foo; int bar; foo  = 13 ; bar = 50 ;  foo + bar ;}), "({int foo; int bar; foo  = 13 ; bar = 50 ;  foo + bar ;})");
  assert( 10,({int foo; int bar;foo = -1 ; bar = 9; foo*bar+bar*2+foo*-1;}), "({int foo; int bar;foo = -1 ; bar = 9;  foo*bar+bar*2+foo*-1;})");
  assert( 18,({int foo; int bar; foo = -1 ; bar = 9; foo = foo +bar;  foo +10;}), "({int foo; int bar; foo = -1 ; bar = 9; foo = foo +bar;  foo +10;})");

  // #10
  printf("\n\n#10\n");

  // #11
  printf("\n\n#11\n");
  // assert( 10 ,({if ( 1 ==1 )  10;}),"({if ( 1 ==1 )  10;})");
  assert( 20 ,({int foo; foo = 10;int bar; int result ;bar = 20; if (foo == bar ) {result = foo;} else { result = bar;} result;}),"({int foo; foo = 10;int bar; bar = 20; if (foo == bar )  foo; else  bar;})");

  assert( 10 ,({int i; i = 0; while(i <10) i = i + 1;  i;}),"({int i; i = 0; while(i <10) i = i + 1;  i;})");
  assert( 8 ,({int i; i = 1;  while (i <=1024) i = i + i;  i/256;}),"({int i; i = 1;  while (i <=1024) i = i + i;  i/256;})");
  assert( 57 ,({int foo;int i; foo = 12;for(i = 0;i<10;i = i+1)foo = foo+i; foo; }),"({int foo;int i; foo = 12;for(i = 0;i<10;i = i+1)foo = foo+i; foo; })");
  assert( 50, ({int result; int i;result = 0;for(i=1;i<=100;i=i+1) result = result+i; result/101;}),"({int result; int i;result = 0;for(i=1;i<=100;i=i+1) result = result+i; result/101;})");

  // #12
  printf("\n\n#12\n");
  assert( 4, ({int foo; foo=1;{foo= foo+foo;foo=foo+foo;} foo;}),"({int foo; foo=1;{foo= foo+foo;foo=foo+foo;} foo;})");
  assert( 233, ({int n ;n=13;int current; current = 0; int next; next = 1;int i; i = 0; int tmp; tmp = 0; while ( i < n ) { tmp = current; current = next; next = next + tmp; i=i+1;}  current;}),"({int n ;n=13;int current; current = 0; int next; next = 1;int i; i = 0; int tmp; tmp = 0; while ( i < n ) { tmp = current; current = next; next = next + tmp; i=i+1;}  current;})");
  assert( 233, ({int n; int current; int next; int i;int tmp;n=13; current = 0;next = 1; for(i =0;i<n;i=i+1){tmp=current;current = next;next = next +tmp;} current;}),"({int n; int current; int next; int i;int tmp;n=13; current = 0;next = 1; for(i =0;i<n;i=i+1){tmp=current;current = next;next = next +tmp;} current;})");

  // #13
  printf("\n\n#13\n");
  assert( 3 ,({ ret3();}), " ({ ret3();})");
  assert( 8 ,({ add(3, 5);}), " ({ add(3, 5);})");
  assert( 2 ,({ sub(5, 3);}), " ({ sub(5, 3);})");
  assert( 10, ({ mul(2, 5);}),  " ({ mul(2, 5);})");
  assert( 6 ,({ add3(1,2,3);}), " ({ add3(1,2,3);})");
  assert( 21,({ add6(1,2,3,4,5,6);}), " ({ add6(1,2,3,4,5,6);})");

  // #14
  printf("\n\n#14\n");
  assert( 3, myfunc(),"myfunc();");
  assert( 33, ({int a; int b;a = 10; b = 20;  a + b + myfunc();}),"({int a; int b;a = 10; b = 20;  a + b + myfunc();})");

  // #15
  printf("\n\n#15\n");
  assert( 15,two_arity(2,3), "two_arity(2,3)");
  assert( 21,({six_arity(1,2,3,4,5,6);}), "({six_arity(1,2,3,4,5,6);})");
  assert(55, fib(10), "fib(10)");
  assert(1,({multi_type_fn(1,2);}),"({multi_type_fn(1,2)})");
  assert(2, multi_type_fn2(1,2),"multi_type_fn2(1,2)");
  assert(11,multi_type_fn3(1,2,3,4),"multi_type_fn3(1,2,3,4)");

  // #16
  printf("\n\n#16\n");
  assert( 1,({int foo; int *bar; foo=1; bar = &foo;  *bar;}), "({int foo; int *bar; foo=1; bar = &foo;  *bar;})");
  assert( 2,({int foo; int *bar; foo=1; bar = &foo;  *bar+1;}), "({int foo; int *bar; foo=1; bar = &foo;  *bar+1;})");
  assert( 3,( {int x; x=3; *&x; }), "( {int x; x=3; *&x; })");
  assert( 3,( {int x; x=3; int *y;y=&x;  int **z;z=&y;  **z; }), "( {int x; x=3; int *y;y=&x;  int **z;z=&y;  **z; })");
  assert( 5,( { int x; int y; x=3; y=5;  *(&x-1); }), "( { int x; int y; x=3; y=5;  *(&x-1); })"); // コンパイラ依存
  assert( 3,( { int x; int y; x=3; y=5;  *(&y+1); }), "( { int x; int y; x=3; y=5;  *(&y+1); })"); // コンパイラ依存
  assert( 5,( { int x; int *y; x=3; y=&x; *y=5;  x; }), "( { int x; int *y; x=3; y=&x; *y=5;  x; })");
  assert( 7,( { int x; int y; x=3; y=5; *(&x-1)=7; y; }), "( { int x; int y; x=3; y=5; *(&x-1)=7; y; })"); // コンパイラ依存
  assert( 7,( { int x; int y; x=3; y=5; *(&y+1)=7; x; }), "( { int x; int y; x=3; y=5; *(&y+1)=7; x; })"); // コンパイラ依存


  // #17
  printf("\n\n#17\n");
  assert( 10, ({int *a; int x; x = 10; a = &x;  *a; }),"({int *a; int x; x = 10; a = &x; return *a; })");

  // #18
  printf("\n\n#18\n");
  assert( 3, ({int x; int *y; y = &x; *y = 3;  x;}),"({int x; int *y; y = &x; *y = 3;  x;})");
  assert( 3, ( {int x; int *y; int **z; x = 3; y = &x; z = &y; **z;}),"( {int x; int *y; int **z; x = 3; y = &x; z = &y; **z;})");
  assert( 11, ({int x; int *y; x = 1; y = &x;  *y + 10;}),"({int x; int *y; x = 1; y = &x;  *y + 10;})");

  // #19
  // printf("\n\n#19\n");
  // assert( 1, ({int *p; alloc4(&p,1,2,4,8); *p;}),"({int *p; alloc4(&p,1,2,4,8); *p;})");
  // assert( 1, ({int *p; alloc4(&p,1,2,4,8); int *q; q = p;*q;}),"({int *p; alloc4(&p,1,2,4,8); int *q; q = p;*q;})");
  // assert( 4, ({int *p; alloc4(&p,1,2,4,8); int *q; q = p+2;*q;}),"({int *p; alloc4(&p,1,2,4,8); int *q; q = p+2;*q;})");
  // assert( 8, ({int *p; alloc4(&p,1,2,4,8); int *q; q = p+3;*q;}),"({int *p; alloc4(&p,1,2,4,8); int *q; q = p+3;*q;})");

  // #20
  printf("\n\n#20\n");
  assert( 4, ({ sizeof(1);}),"({ sizeof(1);})");
  assert( 8, ({int *p; sizeof(p);}),"({int *p; sizeof(p);})");
  assert( 4, ( { sizeof (1+2);} ),"( { sizeof (1+2);} )");
  assert( 8, ({int *p; int x ; x = 8; p = &x; sizeof (p +2);}),"({int *p; int x ; x = 8; p = &x; sizeof (p +2);})");
  assert( 4, ({sizeof(echo(1)); }),"({sizeof(echo(1)); })");
  assert( 1, ({sizeof(echo2(1)); }),"({sizeof(echo2(1)); })");
  assert( 4, ({int *y; sizeof *y;}),"({int *y; sizeof *y;})");

  // #21
  printf("\n\n#21\n");
  assert( 1,({int a[1]; *a = 1; *a;}), "({int a[1]; *a = 1; *a;})");
  assert( 1,({ int y[2]; *y = 10; int x; x = 1;x;}), "({ int y[2]; *y = 10; int x; x = 1;x;})");
  assert( 10,({int x[10]; *x = 1; *(x+9) = 10; *(x+9); }), "({int x[10]; *x = 1; *(x+9) = 10; *(x+9); })");
  assert( 2,({int a[2]; *a = 1; *(a+1) = 2; int *p ;p =a; *(p+1);}), "({int a[2]; *a = 1; *(a+1) = 2; int *p ;p =a; *(p+1);})");
  assert( 1,({int x ; x = 1; int y[2]; *(y+1) = 10; x;}), "({int x ; x = 1; int y[2]; *(y+1) = 10; x;})");
  assert( 11,({int x ; x = 1; int y[2]; *(y+1) = 10; *(y+1) + x;}), "({int x ; x = 1; int y[2]; *(y+1) = 10; *(y+1) + x;})");
  assert( 8,({int x; x = 1; int y[10]; int i; for(i =0; i<10; i = i+1){*(y+i)=i;} int z ; z = 20; x + *(y+7) ; }), "({int x; x = 1; int y[10]; int i; for(i =0; i<10; i = i+1){*(y+i)=i;} int z ; z = 20; x + *(y+7) ; })");
  assert( 12,({int x[3]; sizeof x;}), "({int x[3]; sizeof x;})");
  assert( 24,({int *x[3]; sizeof x;}), "({int *x[3]; sizeof x;})");

  // #22
  printf("\n\n#22\n");
  assert( 1,({int a[10]; a[1] = 1; a[1];}), "({int a[10]; a[1] = 1; a[1];})");
  assert( 32,({int a[10]; int i; i = 2; a[0]= 10; a[9] = 20; i+ a[0] + a[9]; } ), "({int a[10]; int i; i = 2; a[0]= 10; a[9] = 20; i+ a[0] + a[9]; } )");
  assert( 45,({int a[10]; int i; for(i=0;i<10;i=i+1){a[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + a[i]; }result    ; } ), "({int a[10]; int i; for(i=0;i<10;i=i+1){a[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + a[i]; }result    ; } )");
  assert( 10,({int hoge[2]; int x; x = 2; hoge[x-1] = 10; hoge[1];}), "({int hoge[2]; int x; x = 2; hoge[x-1] = 10; hoge[1];})");

  // #23
  printf("\n\n#23\n");
  assert( 0,g_1, "g_1");
  assert( 1,  ({g_arr2[0] = 1; g_arr2[0];})," ({g_arr2[0] = 1; g_arr2[0];})");
  assert( 45, ({int i; for(i=0;i<10;i=i+1){g_arr2[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + g_arr2[i]; }result    ; } ),"({int i; for(i=0;i<10;i=i+1){g_arr2[i] = i;}  int result; result = 0; for (i = 0;i<10;i = i+1){result = result + g_arr2[i]; }result    ; } )");
  assert( 10, ({ int x; x = 2; g_arr1[x-1] = 10; g_arr1[1];}),"({ int x; x = 2; g_arr1[x-1] = 10; g_arr1[1];})");
  assert( 3, ({g_1 = 1; g_2  = 2; g_1 + g_2;} ),"({g_1 = 1; g_2  = 2; g_1 + g_2;} )");

  // #24
  printf("\n\n#24\n");
  assert( 1,({char a; a = 1; a;}), "({char a; a = 1; a;})");
  assert( 2,({char a; int b; a =1; b =a +1; b;}), "({char a; int b; a =1; b =a +1; b;})");
  assert( 10,({char hoge[10]; hoge[9] = 10; hoge[9];}), "({char hoge[10]; hoge[9] = 10; hoge[9];})");
  assert( 3,({char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; x[0] + y;}), "({char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; x[0] + y;})");
  assert( 5,({char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; y - x[0];}), "({char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; y - x[0];})");
  assert( 10,({g_hoge1[0] =1; g_hoge1[g_hoge1[0]]= 10; g_hoge1[1];}  ), "({g_hoge1[0] =1; g_hoge1[g_hoge1[0]]= 10; g_hoge1[1];}  )");

  assert(97,'a',"'a'");
  assert(10,'\n',"'\n'");

  // #25
  printf("\n\n#25\n");
  assert( 97, ({"abc"[0];}) , "({abc[0];})");
  assert( 97, ({ "abc"[0]; }) , "({abc[0]; })");
  assert( 98, ({ "abc"[1]; }) , "({ abc[1]; })");
  assert( 99, ({ "abc"[2]; }) , "({ abc[2]; })");
  assert( 100, ( { "abcd"[3]; }) ," ( { abcd[3]; })");
  assert( 4, ( { sizeof("abc"); }) , "( { sizeof(abc); })");
  assert( 12, ({printf("hello world!"); }) , "({printf(hello world!); })");
  assert( 6, ({printf("hello world!\n");printf(" oops\\"); }) , "({printf(hello world!\n);return printf( oops\\); })");
  assert( 6, ({char p[] = "hello";  sizeof p;}), "({char p[] = hello; return sizeof p;})");

  // #26
  printf("\n\n#26\n");
  assert (1,({int x = 1; x;}), "({int x = 1; x;})");
  assert (1,({int x = 1; int *y = &x; *y;}), "({int x = 1; int *y = &x; *y;})");
  assert (3,({int x[2] = {1,2}; x[0]+x[1];} ), "({int x[2] = {1,2}; x[0]+x[1];} )");
  assert (19,({int x[10] = {10,9}; int result = 0; int i=0; for ( i ; i< 10; i = i+1){result = result +x[i];}result;}), "({int x[10] = {10,9}; int result = 0; int i=0; for ( i ; i< 10; i = i+1){result = result +x[i];}result;})");
  assert (0,({int x[2] = {}; x[0]+x[1];}), "({int x[2] = {}; x[0]+x[1];})");
  assert (99,({char p[10] = "cello";p[0]; }), "({char p[10] = cello;p[0]; })");
  assert (111,({char *p = "hello"; p[4];}), "({char *p = hello; p[4];})");
  assert (0,({char p[10] = "cello";p[9]; }), "({char p[10] = cello;p[9]; })");
  assert (5,({char p[10] = "hello";printf(p); }), "({char p[10] = hello;printf(p); })");
  assert (19,({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}result;}), "({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}result;})");
  assert (5,({char p[] = "hello";printf(p); }), "({char p[] = hello;printf(p); })");
  assert (19,({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}result;}), "({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< 2; i = i+1){result = result +x[i];}result;})");
  assert (8,({int x[] = {1,2}; sizeof (x);}), "({int x[] = {1,2}; sizeof (x);})");
  assert (19,({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< sizeof(x)/4; i = i+1){result = result +x[i];}result;}), "({int x[] = {10,9}; int result = 0; int i=0; for ( i ; i< sizeof(x)/4; i = i+1){result = result +x[i];}result;})");
  assert (19,({int x[] = {10,9}; int result = 0;  for ( int i = 0 ; i< sizeof(x)/4; i = i+1){result = result +x[i];}result;}), "({int x[] = {10,9}; int result = 0;  for ( int i = 0 ; i< sizeof(x)/4; i = i+1){result = result +x[i];}result;})");
  assert (1,({g_1=1;g_1;}), "({g_1=1;g_1;})");
  assert (5,({int a = 5; int  *b = &a; *b;}), "({int a = 5; int  *b = &a; *b;})");
  assert (11,({g_arr1[0]=10;g_arr1[1]=1; g_arr1[1]+g_arr1[0];}), "({g_arr1[0]=10;g_arr1[1]=1; g_arr1[1]+g_arr1[0];})");
  assert (6,({g_arr3[1]+g_arr3[0]+g_arr3[2];}), "({g_arr3[1]+g_arr3[0]+g_arr3[2];})");
  assert (13,({g_arr3[2]=10;g_arr3[1]+g_arr3[0]+g_arr3[2];}), "({g_arr3[2]=10;g_arr3[1]+g_arr3[0]+g_arr3[2];})");
  assert (10, ({*g_ptr1;}),"({*g_ptr1;})");

  // #27
  printf("\n\n#27\n");
  assert( 0,( {  ({ 0; }); }), "( {  ({ 0; }); })");
  assert( 2,( {  ({ 0; 1; 2; }); }), "( {  ({ 0; 1; 2; }); })");
  assert( 3,( {  ({ int x=3; x; }); }), "( {  ({ int x=3; x; }); })");
  assert( 1,({  0 + ({int x = 1; x;}); }), "({  0 + ({int x = 1; x;}); })");

  // #28
  printf("\n\n#28\n");
  assert( 2,({int x =1; ({int x = 2; x; }); }), "({int x =1; ({int x = 2; x; }); })");
  assert( 2,( { int x=2; { int x=3; } x; }), "( { int x=2; { int x=3; } x; })");
  assert( 2,( { int x=2; { int x=3; } ({ int y=4; x; });}), "( { int x=2; { int x=3; } { int y=4; x; }})");
  assert( 3,( { int x=2; { x=3; } x; }), "( { int x=2; { x=3; } x; })");

  // #29
  printf("\n\n#29\n");
  assert( 0, ( { int x[2][3]; int *y=x; *y=0; **x; })," ( { int x[2][3]; int *y=x; *y=0; **x; })");
  assert( 1, ( { int x[2][3]; int *y=x; *(y+1)=1; *(*x+1); })," ( { int x[2][3]; int *y=x; *(y+1)=1; *(*x+1); })");
  assert( 2, ( { int x[2][3]; int *y=x; *(y+2)=2;  *(*x+2); })," ( { int x[2][3]; int *y=x; *(y+2)=2;  *(*x+2); })");
  assert( 3, ( { int x[2][3]; int *y=x; *(y+3)=3;  **(x+1); })," ( { int x[2][3]; int *y=x; *(y+3)=3;  **(x+1); })");
  assert( 4, ( { int x[2][3]; int *y=x; *(y+4)=4;  *(*(x+1)+1); })," ( { int x[2][3]; int *y=x; *(y+4)=4;  *(*(x+1)+1); })");
  assert( 5, ( { int x[2][3]; int *y=x; *(y+5)=5;  *(*(x+1)+2); })," ( { int x[2][3]; int *y=x; *(y+5)=5;  *(*(x+1)+2); })");
  assert( 6, ( { int x[2][3]; int *y=x; *(y+6)=6;  **(x+2); })," ( { int x[2][3]; int *y=x; *(y+6)=6;  **(x+2); })");
  assert( 11, ({int hoge[2][3]; hoge[0][0]=1;hoge[1][2]= 10;hoge[0][0]+hoge[1][2];}), " ({int hoge[2][3]; hoge[0][0]=1;hoge[1][2]= 10;hoge[0][0]+hoge[1][2];})");
  assert( 72, ( {int hoge[2][3][4]; for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){hoge[i][j][k]=i+k+j;}}}  int result = 0;for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){result = result + hoge[i][j][k];}}} result; }), " ( {int hoge[2][3][4]; for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){hoge[i][j][k]=i+k+j;}}}  int result = 0;for(int i = 0; i < 2; i=i+1){for (int j = 0; j < 3; j = j+1){for (int k = 0;k<4;k=k+1){result = result + hoge[i][j][k];}}} result; })");
  assert( 96,({int hoge[2][3][4]; sizeof hoge;}), "({int hoge[2][3][4]; sizeof hoge;})");
  assert( 48,({int hoge[2][3][4]; sizeof hoge[0];}), "({int hoge[2][3][4]; sizeof hoge[0];})");
  assert( 16,({int hoge[2][3][4]; sizeof hoge[0][0];}), "({int hoge[2][3][4]; sizeof hoge[0][0];})");
  assert( 4,({int hoge[2][3][4]; sizeof hoge[0][0][0];}), "({int hoge[2][3][4]; sizeof hoge[0][0][0];})");

  // #30
  printf("\n\n#30\n");
  assert( 8,({struct square {int x; int y;} square; sizeof square;}), "({struct square {int x; int y;} square; sizeof square;})");
  assert( 3,({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.x;}), "({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.x;})");
  assert( 2,({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.y;}), "({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.y;})");
  assert( 6,({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.y *square.x;}), "({struct square {int x; int y;} square; square.x = 3; square.y = 2; square.y *square.x;})");
  assert( 80,({struct  subject {int math[10]; int English[10];} subject; sizeof(subject);}) ,"({struct  subject {int math[10]; int English[10];} subject; sizeof(subject);})");
  assert( 1,({struct  subject {int math[10]; int English[10];} subject; subject.math[0]=1; subject.math[0];}), "({struct  subject {int math[10]; int English[10];} subject; subject.math[0]=1; subject.math[0];})");
  assert( 90,({struct  subject {int math[10]; int English[10];} subject; for(int i = 0; i < 10; i = i+1){subject.math[i]= i; subject.English[9-i]=i;} int result = 0;for(int i = 0;i<10;i=i+1){result = result + subject.math[i] + subject.English[i];} result;}), "({struct  subject {int math[10]; int English[10];} subject; for(int i = 0; i < 10; i = i+1){subject.math[i]= i; subject.English[10-i]=i;} int result = 0;for(int i = 0;i<10;i=i+1){result = result + subject.math[i] + subject.English[i];} result;})");
  assert( 32,({ struct hoge {struct {int a; int b[10]; }hoge; int a;  } hoge; hoge.hoge.a = 19; hoge.hoge.b[0] = 1; hoge.hoge.b[2]= 2; hoge.hoge.b[9]=10;hoge.hoge.a + hoge.hoge.b[0]+hoge.hoge.b[2] +hoge.hoge.b[9];}), "({ struct hoge {struct {int a; int b[10]; }hoge; int a;  } hoge; hoge.hoge.a = 19; hoge.hoge.b[0] = 1; hoge.hoge.b[2]= 2; hoge.hoge.b[9]=10;hoge.hoge.a + hoge.hoge.b[0]+hoge.hoge.b[2] +hoge.hoge.b[9];})");
  assert( 12, ({struct hoge{int a; int b;}hoge[10]; hoge[1].a = 2; hoge[2].b =  10;  hoge[1].a + hoge[2].b;}),"({struct hoge{int a; int b;}hoge[10]; hoge[1].a = 2; hoge[2].b =  10;  hoge[1].a + hoge[2].b;})");
  assert( 8,({struct {char a; int b;}hoge; sizeof(hoge);}), "({struct {char a; int b;}hoge; sizeof(hoge);})");
  assert( 16,({struct {char a; int b; char c; }hoge; sizeof(hoge);}), "({struct {char a; int b; char c; }hoge; sizeof(hoge);})");
  assert( 30, ({struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;(*obj).x = 10;(*obj).y = 20; a.x+a.y;}),"({struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;(*obj).x = 10;(*obj).y = 20; a.x+a.y;})");
  assert( 30, ({struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;obj->x = 10;obj->y = 20; a.x+a.y;}),"({struct hoge {int x; int y;} *obj; struct hoge a; obj = &a;obj->x = 10;obj->y = 20; a.x+a.y;})");

  // #31
  printf("\n\n#31\n");
  assert( 1,({typedef int INT; INT x = 1; x;}), "({typedef int INT; INT x = 1; x;})");
  assert( 1,({ struct hoge {int a;}; typedef struct hoge HOGE; HOGE x; x.a = 1; x.a;}), "({ struct hoge {int a;}; typedef struct hoge HOGE; HOGE x; x.a = 1; x.a;})");
  assert( 1,({typedef struct hoge {int a;} HOGE; HOGE x; x.a = 1; x.a;}), "({typedef struct hoge {int a;} HOGE; HOGE x; x.a = 1; x.a;})");
  // assert( 1,({typedef int t; t t = 1; t;}), "({typedef int t; t t = 1; t;})"); // this cause err
  assert( 2,({typedef struct {int a;} t; { typedef int t; } t x; x.a=2; x.a; }), "({typedef struct {int a;} t; { typedef int t; } t x; x.a=2; x.a; })");

  // #32
  printf("\n\n#32\n");
  assert( 2, ({short a = 2;  a;}), "({short a = 2;  a;})");
  assert( 10, ({long a = 10; a;}), "({long a = 10; a;})");
  assert( 2, ({short a; sizeof(a);}), "({short a; sizeof(a);})");
  assert( 8, ({long a; sizeof(a);}), "({long a; sizeof(a);})");
  assert( 20, ({short a[10]; sizeof a;}), "({short a[10]; sizeof a;})");
  assert( 80, ({long a[10]; sizeof a;}), "({long a[10]; sizeof a;})");
  assert( 1, ({sub_short(4,3);}), "({sub_short(4,3);})");
  assert( 1, ({sub_long(4,3);}), "({sub_long(4,3);})");

  // #33
  printf("\n\n#33\n");
  assert(24, ({ int *x[3]; sizeof(x); }), "int *x[3]; sizeof(x);");
  assert(8, ({ int (*x)[3]; sizeof(x); }), "int (*x)[3]; sizeof(x);");
  assert(3, ({ int *x[3]; int y; x[0]=&y; y=3; x[0][0]; }), "int *x[3]; int y; x[0]=&y; y=3; x[0][0];");
  assert(4, ({ int x[3]; int (*y)[3]=x; y[0][0]=4; y[0][0]; }), "int x[3]; int (*y)[3]=x; y[0][0]=4; y[0][0];");

  {void * x;}

  // #34
  printf("\n\n#34\n");
  assert( 0, ({_Bool x = 0; x;}),"({_Bool x = 0; x;})");
  assert( 1, ({_Bool x = 1; x;}),"({_Bool x = 1; x;})");
  assert( 1, ({_Bool x = 2; x;}),"({_Bool x = 2; x;})");
  assert( 1, ({_Bool x = 2==2; x;}),"({_Bool x = 2==2; x;})");
  assert( 0, ({_Bool x = 2==3; x;}),"({_Bool x = 2==3; x;})");

  // #35
  printf("\n\n#35\n");
  assert( 1,({char x = 1; sizeof x;}), "({char x = 1; sizeof x;})");
  assert( 2,({short int x = 1; sizeof(x);}), "({short int x = 1; sizeof(x);})");
  assert( 2,({int short x = 1; sizeof(x);}), "({int short x = 1; sizeof(x);})");
  assert( 4,({int x = 1; sizeof(x);}), "({int x = 1; sizeof(x);})");
  assert( 8,({long x = 1; sizeof(x);}), "({long x = 1; sizeof(x);})");
  assert( 8,({long int x = 1; sizeof(x);}), "({long int x = 1; sizeof(x);})");
  assert( 8,({int long x = 1; sizeof(x);}), "({int long x = 1; sizeof(x);})");
  assert( 1, ({char typedef CHAR; CHAR x = 1;sizeof x;}),"({char typedef CHAR; CHAR x = 1;sizeof x;})");
  assert( 4, ({typedef A ; A x = 1;sizeof x;}),"({typedef A ; A x = 1;sizeof x;})");

  // #36
  printf("\n\n#36\n");
  assert (1, ({sizeof(char);}),"({sizeof(char);})");
  assert (2, ({sizeof(short);}),"({sizeof(short);})");
  assert (2, ({sizeof(short int);}),"({sizeof(short int);})");
  assert (2, ({sizeof(int short);}),"({sizeof(int short);})");
  assert (4, ({sizeof(int);}),"({sizeof(int);})");
  assert (8, ({sizeof(long);}),"({sizeof(long);})");
  assert (8, ({sizeof(long int);}),"({sizeof(long int);})");
  assert (8, ({sizeof(int long);}),"({sizeof(int long);})");

  assert(4, sizeof(0), "sizeof(0)");
  assert(4294967297, 4294967297, "4294967297");
  assert(8, sizeof(4294967297), "sizeof(4294967297)");


  // #37
  printf("\n\n#37\n");
  assert(131585, (int)8590066177, "(int)8590066177");
  assert(513, (short)8590066177, "(short)8590066177");
  assert(1, (char)8590066177, "(char)8590066177");
  assert(1, (_Bool)1, "(bool)1");
  assert(1, (_Bool)2, "(bool)2");
  assert(0, (_Bool)(char)256, "(bool)(char)256");
  assert(1, (long)1, "(long)1");
  assert(0, (long)&*(int *)0, "(long)&*(int *)0");
  assert(5, ({ int x=5; long y=(long)&x; *(int*)y; }), "int x=5; long y=(long)&x; *(int*)y");

  printf("\"\n");
  assert(1, char_fn(), "char_fn()");

  // #38
  printf("\n\n#38\n");
  assert( 0,({enum {zero,one,two}; zero;}), "({enum {zero,one,two}; zero;})");
  assert( 1,({enum {zero,one,two}; one;}), "({enum {zero,one,two}; one;})");
  assert( 2,({enum {zero,one,two}; two;}), "({enum {zero,one,two}; two;})");
  assert( 5,({enum {five=5,six,seven,}; five;}), "({enum {five=5,six,seven,}; five;})");
  assert( 6,({enum {five=5,six,seven,}; six;}), "({enum {five=5,six,seven,}; six;})");
  assert( 7,({enum {five=5,six,seven,}; seven;}), "({enum {five=5,six,seven,}; seven;})");
  assert( 0,({enum{zero, ten = 10 , five = 5}; zero;}), "({enum{zero, ten = 10 , five = 5}; zero;})");
  assert( 10,({enum{zero, ten = 10 , five = 5}; ten;}), "({enum{zero, ten = 10 , five = 5}; ten;})");
  assert( 5,({enum{zero, ten = 10 , five = 5}; five;}), "({enum{zero, ten = 10 , five = 5}; five;})");
  assert( 4,({enum hoge {zero} x; sizeof(x);}), "({enum hoge {zero} x; sizeof(x);})");
  assert( 4,({enum hoge {zero} ; enum hoge x; sizeof(x);}), "({enum hoge {zero} ; enum hoge x; sizeof(x);})");

  // #39
  printf("\n\n#39\n");
  assert(1, count(), "count()");
  assert(2, count(), "count()");
  assert(3, count(), "count()");

  assert(3, count2(), "count()");
  assert(4, count2(), "count()");
  assert(5, count2(), "count()");
  
  // #40
  printf("\n\n#40\n");
  assert(3,(1,2,3),"(1,2,3)");

  // #41
  printf("\n\n#41\n");
  assert(1,({int i =1;  i++;}), "({int i =1;  i++;})");
  assert(2,({int i =1;  ++i;}), "({int i =1;  ++i;})");
  assert(1,({int i =1;  i--;}), "({int i =1;  i--;})");
  assert(0,({int i =1;  --i;}), "({int i =1;  --i;})");
  assert( 2,({int i =1; i++; i;}), "({int i =1; i++; i;})");
  assert( 2,({int i =1; ++i; i;}), "({int i =1; ++i; i;})");
  assert( 0,({int i =1; i--; i;}), "({int i =1; i--; i;})");
  assert( 0,({int i =1; --i; i;}), "({int i =1; --i; i;})");
  assert( 3,({int a[] = {1,3,5};int *p = a+1; *p++;}), "({int a[] = {1,3,5};int *p = a+1; *p++;})");
  assert( 4,({int a[] = {1,3,5};int *p = a+1; ++*p;}), "({int a[] = {1,3,5};int *p = a+1; ++*p;})");
  assert( 3,({int a[] = {1,3,5};int *p = a+1; *p--;}), "({int a[] = {1,3,5};int *p = a+1; *p--;})");
  assert( 2,({int a[] = {1,3,5};int *p = a+1; --*p;}), "({int a[] = {1,3,5};int *p = a+1; --*p;})");
  assert( 5, ({int a[] = {1,3,5};int *p = a+1; *p++; *p;}),"({int a[] = {1,3,5};int *p = a+1; *p++; *p;})");
  assert( 1, ({int a[] = {1,3,5};int *p = a+1; *--p; *p;}),"({int a[] = {1,3,5};int *p = a+1; *--p; *p;})");

  // #42
  printf("\n\n#42\n");
  assert(6,({int i = 3; i+=3;  i;}), "({int i = 3; i+=3;  i;})");
  assert(0,({int i = 3; i-=3;  i;}), "({int i = 3; i-=3;  i;})");
  assert(9,({int i = 3; i*=3;  i;}), "({int i = 3; i*=3;  i;})");
  assert(1,({int i = 3; i/=3;  i;}), "({int i = 3; i/=3;  i;})");
  assert( 6,({int i = 3; i+=3; }), "({int i = 3; i+=3; })");
  assert( 0,({int i = 3; i-=3; }), "({int i = 3; i-=3; })");
  assert( 9,({int i = 3; i*=3; }), "({int i = 3; i*=3; })");
  assert( 1,({int i = 3; i/=3; }), "({int i = 3; i/=3; })");
  assert (45 ,({int result = 0;for (int i =0;i<10 ;i++){result +=i;}result;}),"({int result = 0;for (int i =0;i<10 ;i++){result +=i;}result;})");

  // #43
  printf("\n\n#43\n");
  assert( 1, ({int i = 0; !i;}),"({int i = 0; !i;})");
  assert( 0, ({int i = 0; !1;}),"({int i = 0; !1;})");
  assert( 0, ({int i = 0; !9;}),"({int i = 0; !9;})");
  assert( 1, ({int i = 0; !0;}),"({int i = 0; !0;})");

  // #44
  printf("\n\n#44\n");
  assert(11, ({int i =-12; ~i;}),"({int i =-12; ~i;})");
  assert(1, ({~~1;}),"({~~1;})");

  // #45
  printf("\n\n#45\n");
  assert( 1, ({ 1|0;}),"({ 1|0;})");
  assert( 3, ({ 2|1;}),"({ 2|1;})");
  assert( 3, ({ 3|1;}),"({ 3|1;})");
  assert( 0, ({ 1&0;}),"({ 1&0;})");
  assert( 0, ({ 2&1;}),"({ 2&1;})");
  assert( 1, ({ 3&1;}),"({ 3&1;})");
  assert( 1, ({ 1^0;}),"({ 1^0;})");
  assert( 3, ({ 2^1;}),"({ 2^1;})");
  assert( 0, ({ 0^0;}),"({ 0^0;})");
  assert( 0, ({ 5^5;}),"({ 5^5;})");
  assert( 1,({ 1|1^2&0;}), "({ 1|1^2&0;})");

  // #46
  printf("\n\n#46\n");
  assert( 1,({1||0;}), "({1||0;})");
  assert( 0,({0||0;}), "({0||0;})");
  assert( 1,({1||(1-1)||0;}), "({1||(1-1)||0;})");
  assert( 0,({0||(1-1)||0;}), "({0||(1-1)||0;})");
  assert( 1,({2&&2;}), "({2&&2;})");
  assert( 0,({0&&2;}), "({0&&2;})");
  assert( 0,({2&&0;}), "({2&&0;})");
  assert( 0,({1&&(2-2)&&2;}), "({1&&(2-2)&&2;})");

  // #47
  printf("\n\n#47\n");
  assert(1, ({int x[]={1,2,3}; arr_param(x);}), "({int x[]={1,2,3}; arr_param(x);})");

  // #48
  printf("\n\n#48\n");
  assert( 8, ({struct *foo; sizeof foo;}),"({struct *foo; sizeof foo;})");
  assert( 8, ({struct T *foo; struct T {int x;} ; sizeof (struct T); }),"({struct T *foo; struct T {int x;} ; sizeof (struct T); })");
  assert( 1, ({struct T { struct T *next; int x; } a; struct T b; b.x=1; a.next=&b; a.next->x;}),"({struct T { struct T *next; int x; } a; struct T b; b.x=1; a.next=&b; a.next->x;})");

  // #49
  printf("\n\n#49\n");
  assert( 3, ({int i = 0;for (; i<10; i++){if (i==3){break;}} i;}),"({int i = 0;for (; i<10; i++){if (i==3){break;}} i;})");
  assert( 1, ({int i = 0; i++ == 0;}),"({int i = 0; i++ == 0;})");
  assert( 0, ({int i = 0; ++i == 0;}),"({int i = 0; ++i == 0;})");
  assert( 0, ({int i =0; int result = 0; if (i++ == 0){result =  0;}else {result = 1;}result ;}),"({int i =0; int result = 0; if (i++ == 0){result =  0;}else {result = 1;}result ;})");
  assert( 4, ({int i = 0;int j = 0; while(j<10) {if (i++==3)break; j++;} i;}),"({int i = 0;int j = 0; while(j<10) {if (i++==3)break; j++;} i;})");
  assert( 0, ({int i = 0; for (;;)break; i;}),"({int i = 0; for (;;)break; i;})");
  assert( 3, ({int i = 0; for(;i<10;i++) { for (;;) break; if (i == 3) break; } i;}),"({int i = 0; for(;i<10;i++) { for (;;) break; if (i == 3) break; } i;})");
  assert( 1, ({int i =0; for (;;){for (;;) break; i++; break;}i;}),"({int i =0; for (;;){for (;;) break; i++; break;}i;})");
  assert( 4, ({int i = 0; while(1) { while(1) break; if (i++==3)break;} i;}),"({int i = 0; while(1) { while(1) break; if (i++==3)break;} i;})");

  // #50
  printf("\n\n#50\n");
  assert( 10, ({int i = 0; for (;i<10;i++){if (i==3)continue; if (i==3){return i;}}  i;}),"({int i = 0; for (;i<10;i++){if (i==3)continue; if (i==3){return i;}}  i;})");
  assert( 10, ({int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; } i;}),"({int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; } i;})");
  assert( 6, ({int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; } j;}),"({int i =0; int j =0;for(;i<10;i++){if(i>5)continue;j++; } j;})");
  assert( 10, ({int i=0; int j=0; for(;!i;) { for (;j!=10;j++) continue; break; }  j;}),"({int i=0; int j=0; for(;!i;) { for (;j!=10;j++) continue; break; }  j;})");
  assert( 10, ({int i = 0; while(i<10){if (i==3){i++;continue;} if (i==3){break;} i++;}  i;}),"({int i = 0; while(i<10){if (i==3){i++;continue;} if (i==3){break;} i++;}  i;})");
  assert( 11, ({int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; }  i;}),"({int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; }  i;})");
  assert( 5, ({int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; }  j;}),"({int i=0; int j=0; while (i++<10) { if (i>5) continue; j++; }  j;})");
  assert( 11, ({int i=0; int j=0; while(!i) { while (j++!=10) continue; break; }  j;}),"({int i=0; int j=0; while(!i) { while (j++!=10) continue; break; }  j;})");

  // #51
  printf("\n\n#51\n");
  assert( 3, ({int i =0; goto a; a: i++; b: i++; c: i++;  i;}),"({int i =0; goto a; a: i++; b: i++; c: i++;  i;})");
  assert( 2, ({int i =0; goto e; d: i++; e: i++; f: i++;  i;}),"({int i =0; goto e; d: i++; e: i++; f: i++;  i;})");
  assert( 1, ({int i =0; goto j; g: i++; h: i++; j: i++;  i;}),"({int i =0; goto j; g: i++; h: i++; j: i++;  i;})");
  assert( 1, ({label_test:1;1;}),"({label_test:return 1;})");

  // #52
  printf("\n\n#52\n");
  assert( 1, ({int i = 0; switch(0){case 0: i = 1;break; case 1: i = 2;break; case 3: i=3;break;} i;}),"({int i = 0; switch(0){case 0: i = 1;break; case 1: i = 2;break; case 3: i=3;break;} i;})");
  assert( 6, ({int i=0; switch(1) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;}),"({int i=0; switch(1) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;})");
  assert( 7, ({int i=0; switch(2) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;}),"({int i=0; switch(2) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;})");
  assert( 1, ({int i=1; switch(3) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;}),"({int i=1; switch(3) { case 0:i=5;break; case 1:i=6;break; case 2:i=7;break; } i;})");
  assert( 5, ({int i=0; switch(0) { case 0:i=5;break; default:i=7; } i;}),"({int i=0; switch(0) { case 0:i=5;break; default:i=7; } i;})");
  assert( 7, ({int i=0; switch(1) { case 0:i=5;break; default:i=7; } i;}),"({int i=0; switch(1) { case 0:i=5;break; default:i=7; } i;})");
  assert( 2, ({int i = 0;switch(0){case 0: i++; case 1: i++;} i;}),"({int i = 0;switch(0){case 0: i++; case 1: i++;} i;})");
  assert( 20,({int i=0; switch(1) { case 0:i=5;break; default:i=7; switch(i){case 0: i = 11; default: i = 20;} } i;}), "({int i=0; switch(1) { case 0:i=5;break; default:i=7; switch(i){case 0: i = 11; default: i = 20;} } i;})");
  assert( 11,({int i = 0; switch(1){default: i = 10; case 0: i++;} i;}), "({int i = 0; switch(1){default: i = 10; case 0: i++;} i;})");
  assert(9, ({int i = 0; int j = 0;for(;i<10;i++){switch(i){case 5: break; default: j++;break;} if (j==5){ break;}  }  i + j;}), "({int i = 0; int j = 0;for(;i<10;i++){switch(i){case 5: break; default: j++;break;} if (j==5){ break;}  }  i + j;})");

  // #53
  printf("\n\n#53\n");
  void_fn();

  // #54
  printf("\n\n#54\n");
  assert(1, 1<<0, "1<<0");
  assert(8, 1<<3, "1<<3");
  assert(10, 5<<1, "5<<1");
  assert(2, 5>>1, "5>>1");
  assert(-1, -1>>1, "-1>>1");
  assert(1, ({ int i=1; i<<=0; i; }), "int i=1; i<<0;");
  assert(8, ({ int i=1; i<<=3; i; }), "int i=1; i<<3;");
  assert(10, ({ int i=5; i<<=1; i; }), "int i=5; i<<1;");
  assert(2, ({ int i=5; i>>=1; i; }), "int i=5; i>>1;");
  assert(-1, -1, "-1");
  assert(-1, ({ int i=-1; i; }), "int i=-1; i;");
  assert(-1, ({ int i=-1; i>>=1; i; }), "int i=1; i>>1;");

  // #55
  printf("\n\n#55\n");
  assert( 2, ({0?1:2;}),"({0?1:2;})");
  assert( 1, ({1?1:2;}),"({1?1:2;})");
  
  // #56
  printf("\n\n#56\n");
  assert( 10,({enum { ten=1+2+3+4, }; ten;}), "({enum { ten=1+2+3+4, }; ten;})");
  assert( 1,({int i=0; switch(3) { case 5-2+0*3: i++; } i;}), "({int i=0; switch(3) { case 5-2+0*3: i++; } i;})");
  assert( 8,({int x[1+1]; sizeof(x);}), "({int x[1+1]; sizeof(x);})");
  assert( 2,({char x[1?2:3]; sizeof(x);}), "({char x[1?2:3]; sizeof(x);})");
  assert( 3,({char x[0?2:3]; sizeof(x);}), "({char x[0?2:3]; sizeof(x);})");


  // #57
  printf("\n\n#57\n");
  assert( 1, ({int x[3]={1,2,3};  x[0];}),"({int x[3]={1,2,3};  x[0];})");
  assert( 2, ({int x[3]={1,2,3};  x[1];}),"({int x[3]={1,2,3};  x[1];})");
  assert( 3, ({int x[3]={1,2,3};  x[2];}),"({int x[3]={1,2,3};  x[2];})");
  assert( 3, ({int x[3]={1,2,3,}; x[2];}),"({int x[3]={1,2,3,}; x[2];})");
  assert( 2, ({int x[2][3]={{1,2,3},{4,5,6}};  x[0][1];}),"({int x[2][3]={{1,2,3},{4,5,6}};  x[0][1];})");
  assert( 4, ({int x[2][3]={{1,2,3},{4,5,6}};  x[1][0];}),"({int x[2][3]={{1,2,3},{4,5,6}};  x[1][0];})");
  assert( 6, ({int x[2][3]={{1,2,3},{4,5,6}};  x[1][2];}),"({int x[2][3]={{1,2,3},{4,5,6}};  x[1][2];})");

  // #58
  printf("\n\n#58\n");
  assert( 2,({int x[2][3]={{1,2}}; x[0][1];}), "({int x[2][3]={{1,2}}; x[0][1];})");
  assert( 0,({int x[2][3]={{1,2}}; x[1][0];}), "({int x[2][3]={{1,2}}; x[1][0];})");
  assert( 0,({int x[2][3]={{1,2}}; x[1][2];}), "({int x[2][3]={{1,2}}; x[1][2];})");

  // #59
  printf("\n\n#59\n");
  assert( 104, ({char p[6] = "hello"; p[0];}), "({char p[6] = \"hello\"; p[0];})");
  assert( 108, ({char p[6] = "hello"; p[3];}), "({char p[6] = \"hello\"; p[3];})");
  assert( 0, ({char p[6] = "hello"; p[5];}), "({char p[6] = \"hello\"; p[5];})");
  assert( 97, ({char x[2][4]={"abc","def"};x[0][0];}), "({char x[2][4]={\"abc\",\"def\"};x[0][0];})");
  assert( 0, ({char x[2][4]={"abc","def"};x[0][3];}), "({char x[2][4]={\"abc\",\"def\"};x[0][3];})");
  assert( 100, ({char x[2][4]={"abc","def"};x[1][0];}), "({char x[2][4]={\"abc\",\"def\"};x[1][0];})");
  assert( 102, ({char x[2][4]={"abc","def"};x[1][2];}), "({char x[2][4]={\"abc\",\"def\"};x[1][2];})");

  // #60
  printf("\n\n#60\n");
  assert( 3, ({int x[]={1,2,3}; x[2];}),"({int x[]={1,2,3}; x[2];})");
  assert( 16, ({int x[]={1,2,3,4}; sizeof x;}),"({int x[]={1,2,3,4}; sizeof x;})");
  assert( 6, ({char p[] = "Hello"; sizeof p;}),"({char p[] = \"Hello\"; sizeof p;})");

  // #61
  printf("\n\n#61\n");
  assert(1, ({ struct {int a; int b; int c;} x={1,2,3}; x.a; }), "struct {int a; int b; int c;} x={1,2,3}; x.a;");
  assert(2, ({ struct {int a; int b; int c;} x={1,2,3}; x.b; }), "struct {int a; int b; int c;} x={1,2,3}; x.b;");
  assert(3, ({ struct {int a; int b; int c;} x={1,2,3}; x.c; }), "struct {int a; int b; int c;} x={1,2,3}; x.c;");
  assert(1, ({ struct {int a; int b; int c;} x={1}; x.a; }), "struct {int a; int b; int c;} x={1}; x.a;");
  assert(0, ({ struct {int a; int b; int c;} x={1}; x.b; }), "struct {int a; int b; int c;} x={1}; x.b;");
  assert(0, ({ struct {int a; int b; int c;} x={1}; x.c; }), "struct {int a; int b; int c;} x={1}; x.c;");

  assert(1, ({ struct {int a; int b;} x[2]={{1,2},{3,4}}; x[0].a; }), "struct {int a; int b;} x[2]={{1,2},{3,4}}; x[0].a;");
  assert(2, ({ struct {int a; int b;} x[2]={{1,2},{3,4}}; x[0].b; }), "struct {int a; int b;} x[2]={{1,2},{3,4}}; x[0].b;");
  assert(3, ({ struct {int a; int b;} x[2]={{1,2},{3,4}}; x[1].a; }), "struct {int a; int b;} x[2]={{1,2},{3,4}}; x[1].a;");
  assert(4, ({ struct {int a; int b;} x[2]={{1,2},{3,4}}; x[1].b; }), "struct {int a; int b;} x[2]={{1,2},{3,4}}; x[1].b;");

  assert(0, ({ struct {int a; int b;} x[2]={{1,2}}; x[1].b; }), "struct {int a; int b;} x[2]={{1,2}}; x[1].b;");

  // #62,63
  printf("\n\n#62,#63\n");
  assert(1,g4[0].a,"g4[0].a");
  assert(2,g4[0].b,"g4[0].b");
  assert(3,g4[1].a,"g4[1].a");
  assert(4,g4[1].b,"g4[1].b");
  assert(1,g5[0].a[0],"g5[0].a[0]");
  assert(2,g5[0].a[1],"g5[0].a[1]");
  assert(3,g5[1].a[0],"g5[1].a[0]");
  assert(4,g5[1].a[1],"g5[1].a[1]");
  assert(102,g6[0][0],"g6[0][0]");
  assert(114,g6[1][2],"g6[1][2]");

  // #64
  printf("\n\n#64\n");
  assert(1, g7[0].a[0], "g13[0].a[0]");
  assert(2, g7[0].a[1], "g13[0].a[1]");
  assert(3, g7[1].a[0], "g13[1].a[0]");
  assert(4, g7[1].a[1], "g13[1].a[1]");

  assert(1, g8[0].a[0], "g14[0].a[0]");
  assert(2, g8[0].a[1], "g14[0].a[1]");
  assert(3, g8[1].a[0], "g14[1].a[0]");
  assert(4, g8[1].a[1], "g14[1].a[1]");

  assert(0, ({ int x[2][3]={0,1,2,3,4,5,}; x[0][0]; }), "int x[2][3]={0,1,2,3,4,5,}; x[0][0];");
  assert(3, ({ int x[2][3]={0,1,2,3,4,5,}; x[1][0]; }), "int x[2][3]={0,1,2,3,4,5,}; x[1][0];");

  assert(0, ({ struct {int a; int b;} x[2]={0,1,2,3}; x[0].a; }), "struct {int a; int b;} x[2]={0,1,2,3}; x[0].a;");
  assert(2, ({ struct {int a; int b;} x[2]={0,1,2,3}; x[1].a; }), "struct {int a; int b;} x[2]={0,1,2,3}; x[1].a;");

  assert(0, strcmp(g9, "foo"), "strcmp(g15, \"foo\")");
  assert(0, strcmp(g10[0], "foo"), "strcmp(g16[0], \"foo\")");
  assert(0, strcmp(g10[1], "bar"), "strcmp(g16[1], \"bar\")");

  // #65
  printf("\n\n#65\n");
  assert(0, strcmp(g11, "hello2"), "strcmp(g11, \"hello\")");
  assert(0, strcmp(g12, "lo2"), "strcmp(g12, \"lo\")");
  assert(0, strcmp(g13+3, "hello2"), "strcmp(g13+3, \"hello\")");

  assert(3, g14, "g14");
  assert(3, *g15, "*g15");

  // #69
  printf("\n\n#69\n");
  assert( 7, ({int i = 0; int j =0; do{j++;}while(i++ < 6);  j;}),"({int i = 0; int j =0; do{j++;}while(i++ < 6);  j;})");
  assert( 4, ({int i = 0; int j =0; int k = 0; do{if (++j > 3)break; continue; k++;}while(1);  j;}),"({int i = 0; int j =0; int k = 0; do{if (++j > 3)break; continue; k++;}while(1);  j;})");

  // #70
  printf("\n\n#70\n");

  // #71
  printf("\n\n#71\n");
  assert( 6,({struct a{int a; int b; int c;} c={1,2,3}; struct a d; d = c;  d.a+d.b+d.c;}), "({struct a{int a; int b; int c;} c={1,2,3}; struct a d; d = c;  d.a+d.b+d.c;})");

  // #72
  printf("\n\n#72\n");
  assert( 3,({struct a{int a;}; struct b{int b;}; struct a a = {1}; struct b b ={2}; a.a + b.b;}), "({struct a{int a;}; struct b{int b;}; struct a a = {1}; struct b b ={2}; a.a + b.b;})");

  // #73
  printf("\n\n#73\n");
  assert(2, ({ int i=6; i&=3; i; }), "int i=6; i&=3; i;");
  assert(7, ({ int i=6; i|=3; i; }), "int i=6; i|=3; i;");
  assert(10, ({ int i=15; i^=5; i; }), "int 15; i^=5; i;");

  // #74
  printf("\n\n#74\n");
  assert(1, (int){1}, "(int){1}");
  assert(2, ((int[]){0,1,2})[2], "(int[]){0,1,2}[2]");
  assert('a', ((struct {char a; int b;}){'a', 3}).a, "((struct {char a; int b;}){'a', 3}).a");
  assert(3, ({ int x=3; (int){x}; }), "int x=3; (int){x};");

  assert(1, tree->val, "tree->val");
  assert(2, tree->lhs->val, "tree->lhs->val");
  assert(3, tree->lhs->lhs->val, "tree->lhs->lhs->val");
  assert(4, tree->lhs->rhs->val, "tree->lhs->rhs->val");

  printf("\n\n-----  ALL  TEST  PASSED  -----\n");
  return 0;
}