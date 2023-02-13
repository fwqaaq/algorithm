/*
 * @lc app=leetcode.cn id=904 lang=c
 *
 * [904] 水果成篮
 */

// @lc code=start

#include <limits.h>

int totalFruit(int *fruits, int fruitsSize) {
  int l = 0, r = 0;
  int res = INT_MIN;

  while (r < fruitsSize) {
    int hash[100000] = {};
    int sum = 0;
    while (sum <= 2) {
      if (r == fruitsSize) {
        break;
      }
      if (hash[fruits[r]]++ == 0) {
        sum++;
      }
      if (sum == 3) {
        break;
      }
      r++;
    }
    res = res > r - l ? res : r - l;
    if (r == fruitsSize) {
      break;
    }
    l = r;
    while (fruits[l - 1] == fruits[r - 1]) {
      r -= 1;
    }
    l = r;
  }
  return res;
}
// @lc code=end
