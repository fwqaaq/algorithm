/*
 * @lc app=leetcode.cn id=844 lang=rust
 *
 * [844] 比较含退格的字符串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s, mut t) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let mut slow = 0;
        for i in 0..s.len() {
            if s[i] == '#' {
                if slow > 0 {
                    slow -= 1;
                }
            } else {
                s[slow] = s[i];
                slow += 1;
            }
        }
        s.truncate(slow);
        slow = 0;
        for i in 0..t.len() {
            if t[i] == '#' {
                if slow > 0 {
                    slow -= 1;
                }
            } else {
                t[slow] = t[i];
                slow += 1;
            }
        }
        t.truncate(slow);
        s == t
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_elements() {
        let s = "isfcow#".to_string();
        let t = "isfco#w#".to_string();
        let res = Solution::backspace_compare(s, t);
        assert!(!res);
    }
}
