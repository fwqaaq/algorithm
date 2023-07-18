use std::vec;

/*
 * @lc app=leetcode.cn id=583 lang=rust
 *
 * [583] 两个字符串的删除操作
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn min_distance(word1: String, word2: String) -> i32 {
    //     let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    //     for i in 0..word1.len() {
    //         dp[i + 1][0] = i + 1;
    //     }
    //     for j in 0..word2.len() {
    //         dp[0][j + 1] = j + 1;
    //     }
    //     for (i, char1) in word1.chars().enumerate() {
    //         for (j, char2) in word2.chars().enumerate() {
    //             if char1 == char2 {
    //                 dp[i + 1][j + 1] = dp[i][j];
    //                 continue;
    //             }
    //             dp[i + 1][j + 1] = dp[i][j + 1].min(dp[i + 1][j]) + 1;
    //         }
    //     }
    //     dp[word1.len()][word2.len()] as i32
    // }

    // 最长公共子序列
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for (i, char1) in word1.chars().enumerate() {
            for (j, char2) in word2.chars().enumerate() {
                if char1 == char2 {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    continue;
                }
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
        (word1.len() + word2.len() - 2 * dp[word1.len()][word2.len()]) as i32
    }
}
// @lc code=end
