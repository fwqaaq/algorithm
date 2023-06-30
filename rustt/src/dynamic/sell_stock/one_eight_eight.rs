/*
 * @lc app=leetcode.cn id=188 lang=rust
 *
 * [188] 买卖股票的最佳时机 IV
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    //     let mut dp = vec![vec![0; 2 * k as usize + 1]; prices.len()];

    //     for v in dp[0].iter_mut().skip(1).step_by(2) {
    //         *v = -prices[0];
    //     }

    //     for (i, &p) in prices.iter().enumerate().skip(1) {
    //         for j in (0..2 * k as usize - 1).step_by(2) {
    //             dp[i][j + 1] = dp[i - 1][j + 1].max(dp[i - 1][j] - p);
    //             dp[i][j + 2] = dp[i - 1][j + 2].max(dp[i - 1][j + 1] + p);
    //         }
    //     }

    //     dp[prices.len() - 1][2 * k as usize]
    // }

    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![0; 2 * k as usize + 1];
        for v in dp.iter_mut().skip(1).step_by(2) {
            *v = -prices[0];
        }

        for p in prices {
            for i in 1..=2 * k as usize {
                if i % 2 == 1 {
                    // 买入
                    dp[i] = dp[i].max(dp[i - 1] - p);
                    continue;
                }
                // 卖出
                dp[i] = dp[i].max(dp[i - 1] + p);
            }
        }

        dp[2 * k as usize]
    }
}
// @lc code=end
