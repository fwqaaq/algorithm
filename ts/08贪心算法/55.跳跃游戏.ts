/**
 * /*
 *
 * [55] 跳跃游戏
 *
 * @format
 * @lc app=leetcode.cn id=55 lang=typescript
 */

// @lc code=start
function canJump(nums: number[]): boolean {
  let startIndex = 0
  if (nums.length === 1) return true
  for (let j = 0; j <= startIndex; j++) {
    //固定范围内的最大跳跃距离
    //j指针移动,直到找到符合条件的值
    startIndex = Math.max(startIndex, j + nums[j])
    if (startIndex >= nums.length - 1) return true
  }
  return false
}
// @lc code=end
console.log(canJump([1, 1, 0, 1]))
export {}
