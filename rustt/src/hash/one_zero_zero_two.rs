/*
 * @lc app=leetcode.cn id=1002 lang=rust
 *
 * [1002] 查找共用字符
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        if words.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut hash = vec![0; 26];
        for i in words[0].bytes() {
            hash[(i - b'a') as usize] += 1;
        }
        for i in words.iter().skip(1) {
            let mut other_hash_str = vec![0; 26];
            for j in i.bytes() {
                other_hash_str[(j - b'a') as usize] += 1;
            }
            for k in 0..26 {
                hash[k] = hash[k].min(other_hash_str[k]);
            }
        }

        for (i, v) in hash.iter_mut().enumerate() {
            while *v > 0 {
                res.push(((i as u8 + b'a') as char).to_string());
                *v -= 1;
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
    fn test_common_chars() {
        let res = Solution::common_chars(vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string(),
        ]);
        assert_eq!(res, vec!["e".to_string(), "l".to_string(), "l".to_string()]);
    }
}
