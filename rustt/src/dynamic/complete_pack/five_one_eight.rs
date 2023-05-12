/*
 * @lc app=leetcode.cn id=518 lang=rust
 *
 * [518] 零钱兑换 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for coin in coins {
            for j in coin as usize..=amount {
                dp[j] += dp[j - coin as usize];
            }
        }
        dp[amount]
    }
}
// @lc code=end
