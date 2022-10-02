/**
 * /*
 *
 * [62] 不同路径
 *
 * @format
 * @lc app=leetcode.cn id=62 lang=typescript
 */

// @lc code=start
function uniquePaths(m: number, n: number): number {
  const dp: number[][] = []
  for (let i = 0; i < m; i++) {
    dp[i] = [1]
    for (let j = 0; j < n; j++) {
      dp[i][j] = 1
      if (i > 0) break
    }
  }
  for (let i = 1; i < m; i++) {
    for (let j = 1; j < n; j++) {
      dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
    }
  }
  return dp[m - 1][n - 1]
}
// @lc code=end

console.log(uniquePaths(3, 7))
export {}
