/*
 * @lc app=leetcode.cn id=283 lang=typescript
 *
 * [283] 移动零
 */

// @lc code=start
/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes(nums: number[]): number[] {
  let low = 0,
    high = 0
  while (high < nums.length) {
    if (nums[high] !== 0) {
      // 如果不是0,就依次覆盖数组
      nums[low++] = nums[high]
    }
    high++
  }
  // 将剩余的所有0放到末尾
  while (low < nums.length) {
    nums[low++] = 0
  }
  return nums
}
// @lc code=end

console.log(moveZeroes([0, 1, 0, 3, 12]))
