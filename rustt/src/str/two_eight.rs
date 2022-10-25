/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 找出字符串中第一个匹配项的下标
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn get_next(next_len: usize, s: &Vec<char>) -> Vec<i32> {
        let mut next = vec![-1; next_len];
        let mut j = -1;
        for i in 1..s.len() {
            // 前后缀不相同，继续向前回退
            while j >= 0 && s[(j + 1) as usize] != s[i] {
                j = next[j as usize];
            }
            // 前后缀相同，j++
            if s[i] == s[(j + 1) as usize] {
                j += 1;
            }
            next[i] = j;
        }
        next
    }
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        let (haystack_chars, needle_chars) = (
            haystack.chars().collect::<Vec<char>>(),
            needle.chars().collect::<Vec<char>>(),
        );
        let mut j = -1;
        let next = Self::get_next(needle.len(), &needle_chars);
        for (i, v) in haystack_chars.into_iter().enumerate() {
            while j >= 0 && v != needle_chars[(j + 1) as usize] {
                j = next[j as usize];
            }
            if v == needle_chars[(j + 1) as usize] {
                j += 1;
            }
            if j == needle_chars.len() as i32 - 1 {
                return (i - needle_chars.len() + 1) as i32;
            }
        }
        -1
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {}
}
