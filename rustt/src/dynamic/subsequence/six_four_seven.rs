/*
 * @lc app=leetcode.cn id=647 lang=rust
 *
 * [647] 回文子串
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // 动态规划
    pub fn count_substrings(s: String) -> i32 {
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut res = 0;

        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if s[i..=i] == s[j..=j] && (j - i <= 1 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    res += 1;
                }
            }
        }
        res
    }
    // 双指针
    // pub fn count_substrings(s: String) -> i32 {
    //     let mut res = 0;
    //     for i in 0..s.len() {
    //         res += Self::extend(&s, i, i, s.len());
    //         res += Self::extend(&s, i, i + 1, s.len());
    //     }
    //     res
    // }

    // fn extend(s: &str, mut i: usize, mut j: usize, len: usize) -> i32 {
    //     let mut res = 0;
    //     while i < len && j < len && s[i..=i] == s[j..=j] {
    //         res += 1;
    //         i = i.wrapping_sub(1);
    //         j += 1;
    //     }
    //     res
    // }
}
// @lc code=end
#[test]
fn test_count_substrings() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
