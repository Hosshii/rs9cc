int printf(char *p);
int exit(int status);

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

int assert(int expected, int actual, char *msg) {
  if (expected == actual) {
    printf("ok ");
    return 0;
  } else {
    printf(msg);
    exit(actual);
  }
}

int main() {
  /*
  this is test script.
  */

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

  // #16
  printf("\n\n#16\n");
  assert( 1,({int foo; int *bar; foo=1; bar = &foo;  *bar;}), "({int foo; int *bar; foo=1; bar = &foo;  *bar;})");
  assert( 2,({int foo; int *bar; foo=1; bar = &foo;  *bar+1;}), "({int foo; int *bar; foo=1; bar = &foo;  *bar+1;})");
  assert( 3,( {int x; x=3; *&x; }), "( {int x; x=3; *&x; })");
  assert( 3,( {int x; x=3; int *y;y=&x;  int **z;z=&y;  **z; }), "( {int x; x=3; int *y;y=&x;  int **z;z=&y;  **z; })");
  assert( 5,( { int x; int y; x=3; y=5;  *(&x-2); }), "( { int x; int y; x=3; y=5;  *(&x-8); })"); // コンパイラ 依存
  assert( 3,( { int x; int y; x=3; y=5;  *(&y+2); }), "( { int x; int y; x=3; y=5;  *(&y+8); })"); // コンパイラ 依存
  assert( 5,( { int x; int *y; x=3; y=&x; *y=5;  x; }), "( { int x; int *y; x=3; y=&x; *y=5;  x; })");
  assert( 7,( { int x; int y; x=3; y=5; *(&x-2)=7; y; }), "( { int x; int y; x=3; y=5; *(&x-8)=7; y; })"); // コンパイラ 依存
  assert( 7,( { int x; int y; x=3; y=5; *(&y+2)=7; x; }), "( { int x; int y; x=3; y=5; *(&y+8)=7; x; })"); // コンパイラ 依存


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


  return 0;
}