/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */
pub struct Solution;
// @lc code=start
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         prices
//             .iter()
//             .skip(1)
//             .fold((0, prices[0]), |(mut res, pre), &v| {
//                 if v > pre {
//                     res += v - pre;
//                 }
//                 (res, v)
//             })
//             .0
//     }
// }
impl Solution {
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let mut result = 0;
    //     for i in 1..prices.len() {
    //         result += (prices[i] - prices[i - 1]).max(0);
    //     }
    //     result
    // }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 2]; prices.len()];
        dp[0][0] = -prices[0];
        for i in 1..prices.len() {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] - prices[i]);
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + prices[i]);
        }
        dp[prices.len() - 1][1]
    }
}
// @lc code=end
