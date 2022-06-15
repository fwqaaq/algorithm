/**
 * /*
 *
 * [53] 最大子数组和
 *
 * @format
 * @lc app=leetcode.cn id=53 lang=typescript
 */

// @lc code=start
function maxSubArray(nums: number[]): number {
  let result = -Infinity
  let count = 0
  for (let i = 0; i < nums.length; i++) {
    count += nums[i]
    if (count > result) result = count
    // 从第一个开始,如果count小于0,则重新计算
    if (count < 0) count = 0
  }
  return result
}
// @lc code=end

export {}
