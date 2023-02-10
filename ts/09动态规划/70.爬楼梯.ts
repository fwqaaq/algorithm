/**
 * /*
 *
 * [70] 爬楼梯
 *
 * @format
 * @lc app=leetcode.cn id=70 lang=typescript
 */

// @lc code=start
function climbStairs(n: number): number {
  if (n <= 2) return n
  let dp = [0, 1, 2]
  for (let i = 3; i <= n; i++) {
    dp[i] = dp[i - 1] + dp[i - 2]
  }
  return dp[n]
}
// @lc code=end
export {}
