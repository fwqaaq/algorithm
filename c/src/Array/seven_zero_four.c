/*
 * @lc app=leetcode.cn id=704 lang=c
 *
 * [704] 二分查找
 */

// @lc code=start

#include <stdlib.h>
int search(int *nums, int numsSize, int target) {
    int left = 0, right = numsSize, middle = 0;

    while (left < right) {
        middle = (left + right) / 2;
        if (nums[middle] > target) {
            right--;
        } else if (nums[middle] < target) {
            left++;
        } else {
            return middle;
        }
    }
    free(nums);
    return -1;
}
// @lc code=end
