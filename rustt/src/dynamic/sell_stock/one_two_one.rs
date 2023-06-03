/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let (mut low, mut res) = (i32::MAX, 0);
    //     for p in prices {
    //         low = p.min(low);
    //         res = res.max(p - low);
    //     }
    //     res
    // }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![-prices[0], 0];
        for p in prices {
            dp[0] = dp[0].max(-p);
            dp[1] = dp[1].max(dp[0] + p);
        }
        dp[1]
    }
}
// @lc code=end
