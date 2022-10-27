/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 * [151] 反转字符串中的单词
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn move_space(s: String) -> String {
        let mut slow = 0;
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < s.len() {
            if chars[i] != ' ' {
                // 保证至少有一个空格
                if slow != 0 {
                    chars[slow] = ' ';
                    slow += 1;
                }
                while i < s.len() && chars[i] != ' ' {
                    chars[slow] = chars[i];
                    slow += 1;
                    i += 1;
                }
            }
            i += 1;
        }
        chars.splice(0..slow, []).into_iter().collect()
    }
    pub fn reverse_words(s: String) -> String {
        Self::move_space(s)
            .split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
// @lc code=end
