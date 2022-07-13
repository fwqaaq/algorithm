/**
 * /*
 *
 * [714] 买卖股票的最佳时机含手续费
 *
 * @format
 * @lc app=leetcode.cn id=714 lang=typescript
 */

// @lc code=start a - b + b - c = a - c
function maxProfit(prices: number[], fee: number): number {
  let minPrice = prices[0]
  let res = 0
  for (let i = 1; i < prices.length; i++) {
    if (prices[i] < minPrice) minPrice = prices[i]
    if (prices[i] > minPrice + fee) {
      res += prices[i] - minPrice - fee
      // minus fee, do not subtract expenses repeatedly
      minPrice = prices[i] - fee
    }
  }
  return res
}
// @lc code=end
console.log(maxProfit([1, 3, 7, 5, 10, 3], 3))
export {}
