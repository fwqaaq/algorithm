/**
 * /*
 *
 * [343] 整数拆分
 *
 * @format
 * @lc app=leetcode.cn id=343 lang=typescript
 */

// @lc code=start
function integerBreak(n: number): number {
  // if (n <= 3) return n - 1
  // if (n === 4) return 4
  // let result = 1
  // while (n > 4) {
  //   result *= 3
  //   n -= 3
  // }
  // return result * n
  const dp = new Array(n + 1).fill(0)
  dp[2] = 1
  for (let i = 3; i < n + 1; i++) {
    for (let j = 1; j < i - 1; j++) {
      dp[i] = Math.max(dp[i], Math.max(dp[i - j] * j, (i - j) * j))
    }
  }
  return dp[n]
}
// @lc code=end
console.log(integerBreak(6))
export {}
