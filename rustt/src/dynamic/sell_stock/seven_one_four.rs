/*
 * @lc app=leetcode.cn id=714 lang=rust
 *
 * [714] 买卖股票的最佳时机含手续费
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut low, mut res) = (-prices[0], 0);
        for p in prices {
            low = low.max(res - p);
            res = res.max(p + low - fee);
        }
        res
        // let mut dp = vec![vec![0; 2]; prices.len()];
        // dp[0][0] = -prices[0];
        // for (i, &p) in prices.iter().enumerate().skip(1) {
        //     dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] - p);
        //     dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + p - fee);
        // }
        // dp[prices.len() - 1][1]
    }
}
// @lc code=end
