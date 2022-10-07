/*
 * @lc app=leetcode.cn id=383 lang=rust
 *
 * [383] 赎金信
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut record = vec![0; 26];
        for byte in ransom_note.bytes() {
            record[(byte - b'a') as usize] += 1;
        }

        for byte in magazine.bytes() {
            record[(byte - b'a') as usize] -= 1;
        }

        record.into_iter().filter(|x| *x > 0).count() == 0
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        let res = Solution::can_construct("aa".to_string(), "aab".to_string());
        assert!(res);
    }
}
