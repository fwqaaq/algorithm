/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![-prices[0], 0];
        for p in prices {
            // 可以看作 low、res
            dp[0] = dp[0].max(dp[1] - p);
            dp[1] = dp[1].max(dp[0] + p);
        }
        dp[1]
    }
}
// @lc code=end

#[test]
fn test_max_profit() {
    let res = Solution::max_profit(vec![7, 2, 3, 4, 5, 1, 2, 5]);
}
