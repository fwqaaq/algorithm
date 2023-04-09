/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] 斐波那契数
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn fib(n: i32) -> i32 {
    //     if n <= 1 {
    //         n
    //     } else {
    //         Self::fib(n - 1) + Self::fib(n - 2)
    //     }
    // }

    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = dp[i - 2] + dp[i - 1];
        }
        dp[n]
    }
}
// @lc code=end
