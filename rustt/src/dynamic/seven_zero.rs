/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn climb_stairs(n: i32) -> i32 {
    //     let n = n as usize;
    //     let mut dp = vec![0; n + 1];
    //     dp[0] = 1;
    //     dp[1] = 1;
    //     for i in 2..=n {
    //         dp[i] = dp[i - 1] + dp[i - 2];
    //     }
    //     dp[n]
    // }

    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let (mut a, mut b, mut f) = (1, 1, 0);
        for _ in 2..=n {
            f = a + b;
            a = b;
            b = f;
        }
        f
    }
}
// @lc code=end
