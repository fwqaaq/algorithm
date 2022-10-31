/*
 * @lc app=leetcode.cn id=1047 lang=rust
 *
 * [1047] 删除字符串中的所有相邻重复项
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        let mut chars: Vec<char> = s.chars().collect();
        while let Some(c) = chars.pop() {
            if stack.is_empty() || stack[stack.len() - 1] != c {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        stack.into_iter().rev().collect()
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        let res = Solution::remove_duplicates("abbaca".to_string());
        assert_eq!(res, "ca".to_string());
    }
}
