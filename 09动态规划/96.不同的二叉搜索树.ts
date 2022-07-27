/**
 * /*
 *
 * [96] 不同的二叉搜索树
 *
 * @format
 * @lc app=leetcode.cn id=96 lang=typescript
 */

// @lc code=start
//dp[n] = dp[n - 1]*dp[0] + dp[n - 2]*dp[1] + ... + dp[1]*dp[n - 2] + dp[0]*dp[n - 1]
function numTrees(n: number): number {
  const dp = new Array(n + 1).fill(0)
  dp[0] = 1
  dp[1] = 1
  for (let i = 2; i <= n; i++) {
    for (let j = 1; j <= i; j++) {
      dp[i] += dp[j - 1] * dp[i - j]
    }
  }
  return dp[n]
}
// @lc code=end
console.log(numTrees(4))
export {}
