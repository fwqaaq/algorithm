/*
 * @lc app=leetcode.cn id=27 lang=c
 *
 * [27] 移除元素
 */

// @lc code=start

int removeElement(int *nums, int numsSize, int val) {
  int slow = 0;
  for (int fast = 0; fast < numsSize; fast++) {
    if (nums[fast] != val) {
      nums[slow++] = nums[fast];
    }
  }
  return slow;
}
// @lc code=end
