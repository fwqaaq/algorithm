/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn num_squares(n: i32) -> i32 {
    //     let n = n as usize;
    //     let mut dp = vec![i32::MAX; n + 1];
    //     dp[0] = 0;
    //     for i in 0..=n {
    //         let mut j = 1;
    //         loop {
    //             match j * j > i {
    //                 true => break,
    //                 false => dp[i] = dp[i].min(dp[i - j * j] + 1),
    //             }
    //             j += 1;
    //         }
    //     }
    //     dp[n]
    // }

    pub fn num_squares(n: i32) -> i32 {
        let (n, mut goods) = (n as usize, 1);
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        loop {
            if goods * goods > n {
                break;
            }
            for j in goods * goods..=n {
                dp[j] = dp[j].min(dp[j - goods * goods] + 1);
            }
            goods += 1;
        }
        dp[n]
    }
}
// @lc code=end

#[test]
fn test_num_squares() {
    let res = Solution::num_squares(13);
}
