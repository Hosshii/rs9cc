int printf(char *p,...);
int exit(int status);
#
    #

int assert(int expected, int actual, char *msg) {
  if (expected == actual) {
    printf("ok ");
    return 0;
  } else {
    printf("\n\n");
    printf("err occurred\n\n");
    printf("-----  INPUT  START  -----\n\n");
    printf(msg);
    printf("\n\n");
    printf("-----  INPUT  END  -----\n");
    printf("\nexpected: %d, actual %d\n",expected,actual);
    exit(actual);
  }
}

int ext1 = 5;
int *ext2 = &ext1;
int ext3 = 10;
int *ext4 = &ext3;
static int inner1 = 20;