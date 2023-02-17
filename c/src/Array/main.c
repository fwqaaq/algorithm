// #include "include/704.二分查找.h"
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

int *arr() {
  int *a = (int *)malloc(sizeof(int) * 3);
  a[0] = 1;
  a[1] = 2;
  a[2] = 3;
  return a;
}

int main(int argc, char **argv) {
  int *b = arr();
  printf("%d", b[0]);
  free(b);
  return 0;
}