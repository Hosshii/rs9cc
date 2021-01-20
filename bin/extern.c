#include "test.h"

extern int ext1;
extern int *ext2;

int main(){
  printf("\n-----  TEST  START  -----\n\n");
  assert(5,ext1,"ext1");
  assert(5,*ext2,"*ext2");
  printf("\n\n-----  ALL  TEST  PASSED  -----\n");
  return 0;
}