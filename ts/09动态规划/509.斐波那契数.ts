/**
 * /*
 *
 * [509] 斐波那契数
 *
 * @format
 * @lc app=leetcode.cn id=509 lang=typescript
 */

// @lc code=start
function fib(n: number): number {
  // if (n === 0) return 0
  // if (n === 1) return 1
  // return fib(n - 1) + fib(n - 2)
  if (n <= 1) return n
  const dp = [0, 1]
  for (let i = 2; i <= n; i++) {
    dp[i] = dp[i - 2] + dp[i - 1]
  }
  return dp[n]
}
// @lc code=end

export {}
