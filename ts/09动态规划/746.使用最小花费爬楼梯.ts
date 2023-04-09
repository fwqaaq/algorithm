/**
 * /*
 *
 * [746] 使用最小花费爬楼梯
 *
 * @format
 * @lc app=leetcode.cn id=746 lang=typescript
 */

// @lc code=start
// function minCostClimbingStairs(cost: number[]): number {
//   const dp = [0, 0]
//   for (let i = 2; i <= cost.length; i++) {
//     dp[i] = Math.min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
//   }
//   return dp[cost.length]
// }

function minCostClimbingStairs(cost: number[]): number {
  let dp_before = 0,
    dp_after = 0
  for (let i = 2; i <= cost.length; i++) {
    let dpi = Math.min(dp_before + cost[i - 2], dp_after + cost[i - 1])
    dp_before = dp_after
    dp_after = dpi
  }
  return dp_after
}

// @lc code=end

export {}
