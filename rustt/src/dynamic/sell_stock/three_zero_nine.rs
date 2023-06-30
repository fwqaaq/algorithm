/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 最佳买卖股票时机含冷冻期
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /*
         * dp[i][0]: 持股状态；
         * dp[i][1]: 无股状态，当天为非冷冻期；
         * dp[i][2]: 无股状态，当天卖出；
         * dp[i][3]: 无股状态，当天为冷冻期；
         */
        let mut dp = vec![vec![0; 4]; prices.len()];
        dp[0][0] = -prices[0];
        for (i, &p) in prices.iter().enumerate().skip(1) {
            dp[i][0] = dp[i - 1][0].max((dp[i - 1][3] - p).max(dp[i - 1][1] - p));
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][3]);
            dp[i][2] = dp[i - 1][0] + p;
            dp[i][3] = dp[i - 1][2];
        }
        *dp[prices.len() - 1].iter().skip(1).max().unwrap()
    }
}
// @lc code=end
