/*
 * @lc app=leetcode.cn id=977 lang=c
 *
 * [977] 有序数组的平方
 */

// @lc code=start

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *sortedSquares(int *nums, int numsSize, int *returnSize) {
  *returnSize = numsSize;
  int right = numsSize - 1;
  int left = 0;

  int *ans = (int *)malloc(sizeof(int) * numsSize);
  for (int index = numsSize - 1; index >= 0; index--) {
    int l = nums[left] * nums[left];
    int r = nums[right] * nums[right];

    if (l > r) {
      ans[index] = l;
      left++;
    } else {
      ans[index] = r;
      right--;
    }
  }
  return ans;
}
// @lc code=end
