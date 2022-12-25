/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtracking(&mut res, &mut path, &s.chars().collect(), 0);
        res
    }

    pub fn backtracking(
        res: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
        sub_str: &Vec<char>,
        start: usize,
    ) {
        if start >= sub_str.len() {
            res.push(path.to_vec());
            return;
        }

        for i in start..sub_str.len() {
            if !Self::is_partition(sub_str, start, i) {
                continue;
            }
            let s = sub_str[start..i + 1].iter().collect();
            path.push(s);

            Self::backtracking(res, path, sub_str, i + 1);
            path.pop();
        }
    }

    pub fn is_partition(s: &[char], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}
// @lc code=end
