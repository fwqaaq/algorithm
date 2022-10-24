/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 * [541] 反转字符串 II
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let k = k as usize;
        for i in (0..chars.len()).step_by(2 * k) {
            if (i + k) < chars.len() {
                chars[i..i + k].reverse();
            } else {
                chars[i..].reverse();
            }
        }
        chars.into_iter().collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_str() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        );
        assert_eq!(
            Solution::reverse_str("abcd".to_string(), 2),
            "bacd".to_string()
        );
    }
}
