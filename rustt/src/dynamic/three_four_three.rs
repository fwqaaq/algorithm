/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 * [343] 整数拆分
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn integer_break(n: i32) -> i32 {
    //     let n = n as usize;
    //     let mut dp = vec![1; n + 1];
    //     for i in 3..=n {
    //         for j in 1..=i / 2 {
    //             dp[i] = dp[i].max(((i - j) * j).max(dp[i - j] * j));
    //         }
    //     }
    //     dp[n] as i32
    // }

    pub fn integer_break(mut n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            4 => 4,
            5.. => {
                let mut res = 1;
                while n > 4 {
                    res *= 3;
                    n -= 3;
                }
                res * n
            }
            _ => panic!("Error"),
        }
    }
}
// @lc code=end
