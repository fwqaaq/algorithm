/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn is_subsequence(s: String, t: String) -> bool {
    //     let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    //     for (i, char_s) in s.chars().enumerate() {
    //         for (j, char_t) in t.chars().enumerate() {
    //             if char_s == char_t {
    //                 dp[i + 1][j + 1] = dp[i][j] + 1;
    //                 continue;
    //             }
    //             dp[i + 1][j + 1] = dp[i + 1][j]
    //         }
    //     }
    //     dp[s.len()][t.len()] == s.len()
    // }

    // 一维 dp
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut dp = vec![0; t.len() + 1];
        let (s, t) = (s.as_bytes(), t.as_bytes());
        for &byte_s in s {
            let mut pre = 0;
            for j in 0..t.len() {
                let temp = dp[j + 1];
                if byte_s == t[j] {
                    dp[j + 1] = pre + 1;
                } else {
                    dp[j + 1] = dp[j];
                }
                pre = temp;
            }
        }
        dp[t.len()] == s.len()
    }
}
// @lc code=end

#[test]
fn test_is_subsequence() {
    assert!(Solution::is_subsequence(
        "abc".to_string(),
        "ahbgdc".to_string()
    ));
}
