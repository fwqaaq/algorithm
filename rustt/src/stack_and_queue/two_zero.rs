/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut stack = vec![];
        let mut chars: Vec<char> = s.chars().collect();
        while let Some(s) = chars.pop() {
            match s {
                ')' => stack.push('('),
                ']' => stack.push('['),
                '}' => stack.push('{'),
                _ => {
                    if stack.is_empty() || stack.pop().unwrap() != s {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        let res = Solution::is_valid("(({{".to_string());
        assert!(!res);
    }
}
