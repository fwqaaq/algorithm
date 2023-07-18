/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    //     let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
    //     for (i, c1) in text1.chars().enumerate() {
    //         for (j, c2) in text2.chars().enumerate() {
    //             if c1 == c2 {
    //                 dp[i + 1][j + 1] = dp[i][j] + 1;
    //             } else {
    //                 dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
    //             }
    //         }
    //     }
    //     dp[text1.len()][text2.len()]
    // }

    // 一维 dp
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![0; text2.len() + 1];
        for c1 in text1.chars() {
            // 初始化 prev
            let mut prev = 0;

            for (j, c2) in text2.chars().enumerate() {
                let temp = dp[j + 1];
                if c1 == c2 {
                    // 使用上一次的状态，防止重复计算
                    dp[j + 1] = prev + 1;
                } else {
                    dp[j + 1] = dp[j + 1].max(dp[j]);
                }
                // 使用上一次的状态更新 prev
                prev = temp;
            }
        }
        dp[text2.len()]
    }
}
// @lc code=end
#[test]
fn test_longest_common_subsequence() {
    let text1 = String::from("abcba");
    let text2 = String::from("abcbcba");
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 5);
}
