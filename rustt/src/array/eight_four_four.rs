/*
 * @lc app=leetcode.cn id=844 lang=rust
 *
 * [844] 比较含退格的字符串
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Solution::backspace(s) == Solution::backspace(t)
    }

    pub fn backspace(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == '#' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect::<String>()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        let s = "a#b#c#".to_string();
        let t = "a#d#c#".to_string();
        assert!(Solution::backspace_compare(s, t));
    }
}
