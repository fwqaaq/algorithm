/**
 * /*
 *
 * [122] 买卖股票的最佳时机 II
 *
 * @format
 * @lc app=leetcode.cn id=122 lang=typescript
 */

import { describe, it } from 'node:test'
import assert from 'node:assert'

// @lc code=start

/**
 * use a[n] - a[0] = a[n] - a[n-1] + a[n-1] - a[n-2] + ... +a[0]
 * We reduce the array of prices by comparing the current price to the previous price, and if the
 * current price is greater than the previous price, we add the current price to the previous price and
 * subtract the previous price from the sum.
 * @param {number[]} prices - number[]
 * @returns The max profit that can be made from buying and selling a stock.
 */
// function maxProfit(prices: number[]): number {
//   return prices.reduce((prev, curr, curIndex, arr) => {
//     if (curr > arr[curIndex - 1]) {
//       return curr + prev - arr[curIndex - 1]
//     }
//     return prev
//   }, 0)
// }

/**
 *
 * @param {number} prices - save all money
 **/

function maxProfit(prices: number[]): number {
  const dp = Array(prices.length)
    .fill(0)
    .map(() => Array(2).fill(0))
  dp[0][0] = -prices[0]
  for (let i = 1; i < prices.length; i++) {
    // 买股票后所剩最多的现金
    dp[i][0] = Math.max(dp[i - 1][0], dp[i - 1][1] - prices[i])
    // 不买或者卖完所持有最多的现金
    dp[i][1] = Math.max(dp[i - 1][1], dp[i - 1][0] + prices[i])
  }
  return dp[prices.length - 1][1]
}

// @lc code=end

// describe('macProfile', () => {})

describe('Math', () => {
  assert.strictEqual(maxProfit([1, 3, 5, 2, 8]), 10)
})

export {}
