#include "test.h"

extern int ext1;
extern int *ext2;
static int inner1 = 10;

int test(){
  int ext3 = 1;
  int *ext4  = &ext3;
  return ext3 + *ext4;
}

int main(){
  printf("\n-----  TEST  START  -----\n\n");
  
  assert(5,ext1,"ext1");
  assert(5,*ext2,"*ext2");
  extern int ext3;
  extern int *ext4;
  assert(10,ext3,"ext3");
  assert(10,*ext4,"*ext4");
  assert(2, test(), "test");
  assert(10, inner1 , "inner1");

  printf("\n\n-----  ALL  TEST  PASSED  -----\n");
  return 0;
}