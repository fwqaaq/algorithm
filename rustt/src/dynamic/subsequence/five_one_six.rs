use std::vec;

/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 * [516] 最长回文子序列
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for i in (0..s.len()).rev() {
            dp[i][i] = 1;
            for j in i + 1..s.len() {
                if s[i..=i] == s[j..=j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                    continue;
                }
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
        dp[0][s.len() - 1]
    }
}
// @lc code=end
