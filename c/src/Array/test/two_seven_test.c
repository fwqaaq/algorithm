#include "../include/two_seven.h"

#include <stdio.h>
int main() {
    int nums[] = {3, 2, 2, 3};
    int numsSize = 4;
    int val = 3;
    int result = removeElement(nums, numsSize, val);
    printf("%d\n", result);
}