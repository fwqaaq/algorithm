/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 * [93] 复原 IP 地址
 */

use std::usize;

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid(s: &[char], start: usize, end: usize) -> bool {
        if start > end {
            return false;
        }
        if s[start] == '0' && start != end {
            return false;
        }
        let mut num = 0;
        for &c in s.iter().take(end + 1).skip(start) {
            if !('0'..='9').contains(&c) {
                return false;
            }
            if let Some(digit) = c.to_digit(10) {
                num = num * 10 + digit;
            }
            if num > 255 {
                return false;
            }
        }
        true
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = Vec::new();
        if s.len() < 4 || s.len() > 12 {
            return res;
        }
        let mut s = s.chars().collect::<Vec<char>>();
        Self::backtracking(&mut res, &mut s, 0, 0);
        res
    }
    pub fn backtracking(
        res: &mut Vec<String>,
        s: &mut Vec<char>,
        start_index: usize,
        point_num: usize,
    ) {
        let len = s.len();
        if point_num == 3 {
            if Self::is_valid(s, start_index, len - 1) {
                res.push(s.iter().collect::<String>());
            }
            return;
        }
        for i in start_index..len {
            if Self::is_valid(s, start_index, i) {
                s.insert(i + 1, '.');
                Self::backtracking(res, s, i + 2, point_num + 1);
                s.remove(i + 1);
            } else {
                break;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let res = Solution::restore_ip_addresses("25525511135".to_string());
        println!("{res:?}")
    }
}
