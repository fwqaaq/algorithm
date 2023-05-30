/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len() {
            for j in 0..i {
                if word_dict.iter().any(|word| *word == s[j..i]) && dp[j] {
                    dp[i] = true;
                }
            }
        }
        dp[s.len()]
    }
}
// @lc code=end

#[test]
fn test_word_break() {
    Solution::word_break(
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );
}
