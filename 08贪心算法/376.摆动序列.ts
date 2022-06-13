/**
 * /*
 *
 * [376] 摆动序列
 *
 * @format
 * @lc app=leetcode.cn id=376 lang=typescript
 */

// @lc code=start
function wiggleMaxLength(nums: number[]): number {
  if (nums.length <= 0) return nums.length
  //第一个一定是摆动序列的开头
  let result = 1
  let pre: number = 0,
    cur: number
  for (let i = 1; i < nums.length; i++) {
    cur = nums[i] - nums[i - 1]
    if ((cur < 0 && pre >= 0) || (pre <= 0 && cur > 0)) {
      pre = cur
      result++
    }
  }
  return result
}
// @lc code=end
export {}
