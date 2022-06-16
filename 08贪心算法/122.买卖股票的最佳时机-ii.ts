/**
 * /*
 *
 * [122] 买卖股票的最佳时机 II
 *
 * @format
 * @lc app=leetcode.cn id=122 lang=typescript
 */

// @lc code=start

/**
 * use a[n] - a[0] = a[n] - a[n-1] + a[n-1] - a[n-2] + ... +a[0]
 * We reduce the array of prices by comparing the current price to the previous price, and if the
 * current price is greater than the previous price, we add the current price to the previous price and
 * subtract the previous price from the sum.
 * @param {number[]} prices - number[]
 * @returns The max profit that can be made from buying and selling a stock.
 */
function maxProfit(prices: number[]): number {
  let maxPrice = prices.reduce((prev, curr, curIndex, arr) => {
    if (curr > arr[curIndex - 1]) {
      return curr + prev - arr[curIndex - 1]
    }
    return prev
  }, 0)
  return maxPrice
}
// @lc code=end
console.log(maxProfit([1, 2, 3, 4, 5]))
export {}
