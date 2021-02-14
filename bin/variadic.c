#include "test.h"
// #include <stdio.h>

typedef struct {
  int gp_offset;
  int fp_offset;
  void *overflow_arg_area;
  void *reg_save_area;
} __va_elem;

typedef __va_elem va_list[1];

int add_all(int n, ...);
int sprintf(char *buf, char *fmt, ...);
int vsprintf(char *buf, char *fmt, va_list ap);
void *memcpy(void *buf1,void *buf2,int n);


char *fmt(char *buf, char *fmt, ...) {
  va_list ap;

  memcpy(ap, __va_area__,24);

  vsprintf(buf, fmt, ap);
}

int main(){
   char buf[100]; 
   fmt(buf, "aaa %d %d, %s \n",12,110,"hello world"); 
   printf(buf);
   return 0;
}