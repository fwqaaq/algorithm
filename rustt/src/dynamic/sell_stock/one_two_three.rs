/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 */

pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     /*
    //      * 定义 5 种状态:
    //      * 0: 没有操作, 1: 第一次买入, 2: 第一次卖出, 3: 第二次买入, 4: 第二次卖出
    //      */
    //     let mut dp = vec![vec![0; 5]; prices.len()];
    //     dp[0][1] = -prices[0];
    //     dp[0][3] = -prices[0];

    //     for (i, &p) in prices.iter().enumerate().skip(1) {
    //         // 不操作
    //         // dp[i][0] = dp[i - 1][0];
    //         dp[i][1] = dp[i - 1][1].max(-p);
    //         dp[i][2] = dp[i - 1][2].max(dp[i - 1][1] + p);
    //         dp[i][3] = dp[i - 1][3].max(dp[i - 1][2] - p);
    //         dp[i][4] = dp[i - 1][4].max(dp[i - 1][3] + p);
    //     }

    //     dp[prices.len() - 1][4]
    // }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut one_buy, mut one_sale, mut two_buy, mut two_sale) = (-prices[0], 0, -prices[0], 0);

        for p in prices {
            one_buy = one_buy.max(-p);
            one_sale = one_sale.max(p + one_buy);
            two_buy = two_buy.max(one_sale - p);
            two_sale = two_sale.max(two_buy + p);
        }
        two_sale
    }
}
// @lc code=end

#[test]
fn test_max_profit() {
    let res = Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]);
    println!("{}", res);
}
