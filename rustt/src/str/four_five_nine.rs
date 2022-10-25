/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 * [459] 重复的子字符串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn get_next(next_len: usize, s: &Vec<char>) -> Vec<i32> {
        let mut next = vec![-1; next_len];
        let mut j = -1;
        for i in 1..s.len() {
            while j >= 0 && s[i] != s[(j + 1) as usize] {
                j = next[j as usize];
            }
            if s[i] == s[(j + 1) as usize] {
                j += 1;
            }
            next[i] = j;
        }
        next
    }
    pub fn repeated_substring_pattern(s: String) -> bool {
        // let douable = s.clone() + &s;
        // douable[1..douable.len() - 1].contains(&s)
        let s_chars = s.chars().collect::<Vec<char>>();
        let next = Self::get_next(s_chars.len(), &s_chars);
        println!("{:?}", next);
        if next[s_chars.len() - 1] >= 0
            && s_chars.len() % (s_chars.len() - (next[s_chars.len() - 1] + 1) as usize) == 0
        {
            return true;
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_substring_pattern() {
        let res = Solution::repeated_substring_pattern("abcbc".to_string());
        println!("{:?}", res);
    }
}
