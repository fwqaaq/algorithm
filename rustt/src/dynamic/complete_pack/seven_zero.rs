/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (n, m) = (n as usize, 2);

        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            for j in 1..=m {
                if i >= j {
                    dp[i] += dp[i - j];
                }
            }
        }
        dp[n]
    }
}
// @lc code=end
