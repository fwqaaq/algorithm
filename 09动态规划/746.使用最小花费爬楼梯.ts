/**
 * /*
 *
 * [746] 使用最小花费爬楼梯
 *
 * @format
 * @lc app=leetcode.cn id=746 lang=typescript
 */

// @lc code=start
function minCostClimbingStairs(cost: number[]): number {
  const dp = [cost[0], cost[1]]
  for (let i = 2; i < cost.length; i++) {
    //* find the min cost of the two previous steps
    dp[i] = Math.min(dp[i - 1], dp[i - 2]) + cost[i]
  }
  //* return the min cost of the last two steps
  return Math.min(dp[cost.length - 1], dp[cost.length - 2])
}
// @lc code=end

export {}
