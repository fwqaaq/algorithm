/**
 * /*
 *
 * [1049] 最后一块石头的重量 II
 *
 * @format
 * @lc app=leetcode.cn id=1049 lang=typescript
 */

// @lc code=start
function lastStoneWeightII(stones: number[]): number {
  const sum = stones.reduce((a, b) => a + b)

  const bagSize = stones.length
  const target = Math.floor(sum / 2)
  const dp = new Array(target + 1).fill(0)

  for (let i = 0; i < bagSize; i++) {
    for (let j = target; j >= stones[i]; j--) {
      dp[j] = Math.max(dp[j], dp[j - stones[i]] + stones[i])
    }
  }

  return sum - dp[target] - dp[target]
}
// @lc code=end
export {}
