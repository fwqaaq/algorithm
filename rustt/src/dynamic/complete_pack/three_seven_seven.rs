/*
 * @lc app=leetcode.cn id=377 lang=rust
 *
 * [377] 组合总和 Ⅳ
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &n in &nums {
                if i >= n as usize {
                    dp[i] += dp[i - n as usize];
                }
            }
        }
        dp[target]
    }
}
// @lc code=end
