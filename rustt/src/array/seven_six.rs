/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }

        let s_bytes = s.as_bytes();
        let mut t_length = t.len();
        let mut left = 0;
        let mut map = HashMap::new();
        let mut right_bunder = 0;
        let mut left_bunder = 0;
        for letter in t.chars() {
            *map.entry(letter).or_insert(0) += 1;
        }
        for right in 0..s.len() {
            dbg!(map.get_mut(&(s_bytes[right] as char)));
            if let Some(p) = map.get_mut(&(s_bytes[right] as char)) {
                *p -= 1;
                /* if the letter's count > 0, need to minus 1.
                 Beacuse greater than 0 means the letter has number of times to be used.
                */
                if *p >= 0 {
                    t_length -= 1;
                }
            }
            while t_length == 0 {
                left_bunder = left;
                right_bunder = right + 1;
                if let Some(p) = map.get_mut(&(s_bytes[left] as char)) {
                    *p += 1;
                    if *p > 0 {
                        t_length += 1;
                    }
                }
                left += 1;
            }
        }
        s[left_bunder..right_bunder].to_string()
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        let s = "A".to_string();
        let t = "B".to_string();
        let res = Solution::min_window(s, t);
        assert_eq!(res, "".to_string());
    }
}
