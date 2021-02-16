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
void va_start();


char *fmt(char *buf, char *fmt, ...) {
  va_list ap;
  // *ap = (__va_elem)__va_area__;
  va_start(ap);

  vsprintf(buf, fmt, ap);
}

int main(){
   char buf[100]; 
   fmt(buf, "aaa %d %d, %s \n",12,110,"hello world"); 
   printf(buf);
   return 0;
}