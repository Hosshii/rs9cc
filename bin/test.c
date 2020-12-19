int printf(char *p);
int exit(int status);

int add(int x, int y) { return x + y; }
int sub(int x, int y) { return x - y; }
int mul(int x, int y) { return x * y; }
int add3(int a, int b, int c) { return a + b + c; }

int add6(int a, int b, int c, int d, int e, int f) {
  return a + b + c + d + e + f;
}

int assert(int expected, int actual, char *msg) {
  if (expected == actual) {
    printf("ok ");
    return 0;
  } else {
    printf(msg);
    exit(1);
  }
}

int main() {
  /*
  this is test script.
  */

  // #1
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
  assert(1, 0 == 0, "0==0");
  assert(1, -39 == -39, "-39==-39");
  assert(0, -210 == 932, "-210 == 932");

  // #3
  assert(1, 321 != 4442, "321!=4442");
  assert(0, 33 != 33, "33!=33");

  // #4
  assert(1, 2 > 1, " 2 >   1  ");
  assert(0, 40 > 200, " 40 > 200");
  assert(0, 40 > 40, "40>40");

  // #5
  assert(1, 4 < 200, "4<200");
  assert(0, 4000 < 500, " 4000 < 500");
  assert(0, -40 < -40, "-40<-40");

  // #6
  assert(1, 0 <= 1, "0<=1");
  assert(1, 0 <= 0, "0 <= 0");
  assert(0, 4 <= 0, "4<= 0");

  // #7
  assert(1, 0 >= 0, "0>=0");
  assert(1, -11 >= -11, "-11>=-11");
  assert(1, 100 >= 3, "100 >= 3");
  assert(0, 3 >= 100, "3 >= 100");
  assert(0, -100 >= 30, "-100 >= 30");

  // #8
}