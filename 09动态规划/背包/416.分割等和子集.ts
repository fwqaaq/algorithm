/**
 * /*
 *
 * [416] 分割等和子集
 *
 * @format
 * @lc app=leetcode.cn id=416 lang=typescript
 */

// @lc code=start
function canPartition(nums: number[]): boolean {
  const target = nums.reduce((a, b) => a + b, 0) / 2
  if (target % 1 !== 0) return false
  const bagSize = nums.length
  const dp = new Array(target + 1).fill(0)
  for (let i = 0; i <= bagSize; i++) {
    for (let j = target; j >= nums[i]; j--) {
      dp[j] = Math.max(dp[j], dp[j - nums[i]] + nums[i])
    }
  }
  return dp[target] === target
}
// @lc code=end

console.log(canPartition([1, 2, 3, 5]))

export {}
