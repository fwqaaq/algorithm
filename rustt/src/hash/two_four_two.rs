/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */
pub struct Solution {}
// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut record = vec![0; 26];

        for byte in s.bytes() {
            record[(byte - b'a') as usize] += 1;
        }

        for byte in t.bytes() {
            record[(byte - b'a') as usize] -= 1;
        }

        record.into_iter().filter(|x| *x != 0).count() == 0
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let res = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
        assert!(res);
    }
}
