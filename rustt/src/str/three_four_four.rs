/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] 反转字符串
 */
pub struct Solution;
// @lc code=start

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
// @lc code=end
