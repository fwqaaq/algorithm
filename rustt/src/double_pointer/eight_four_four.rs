/*
 * @lc app=leetcode.cn id=844 lang=rust
 *
 * [844] 比较含退格的字符串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn backspace_compare(s: String, t: String) -> bool {
    //     let (s, t) = (
    //         s.chars().collect::<Vec<char>>(),
    //         t.chars().collect::<Vec<char>>(),
    //     );
    //     Self::get_string(s) == Self::get_string(t)
    // }
    // pub fn get_string(mut chars: Vec<char>) -> Vec<char> {
    //     let mut slow = 0;
    //     for i in 0..chars.len() {
    //         if chars[i] == '#' {
    //             slow = (slow as u32).saturating_sub(1) as usize;
    //         } else {
    //             chars[slow] = chars[i];
    //             slow += 1;
    //         }
    //     }
    //     chars.truncate(slow);
    //     chars
    // }

    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::get_string(s) == Self::get_string(t)
    }

    pub fn get_string(string: String) -> String {
        let mut s = String::new();
        for c in string.chars() {
            if c != '#' {
                s.push(c);
            } else if !s.is_empty() {
                s.pop();
            }
        }
        s
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
