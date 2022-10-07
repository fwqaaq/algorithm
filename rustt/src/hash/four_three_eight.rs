/*
* @lc app=leetcode.cn id=438 lang=rust
*
* [438] 找到字符串中所有字母异位词
*/
pub struct Solution;
// @lc code=start

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let p = p.into_bytes();
        let s = s.into_bytes();
        let mut record_p = vec![0; 26];
        let mut record_window = vec![0; 26];
        let mut res = vec![];
        for i in 0..p.len() {
            record_p[(p[i] - b'a') as usize] += 1;
        }

        for i in 0..s.len() {
            record_window[(s[i] - b'a') as usize] += 1;
            if i >= p.len() {
                // s[i - p.len()]: decrement from the first position of the window
                record_window[(s[i - p.len()] - b'a') as usize] -= 1;
            }
            if record_window == record_p {
                res.push(i as i32 - p.len() as i32 + 1)
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_anagrams() {
        let res = Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string());
        assert_eq!(res, vec![0, 6]);
    }
}
