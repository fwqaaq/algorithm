/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 * [151] 反转字符串中的单词
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn move_space(s: String) -> String {
        let mut slow = 0;
        let mut i = 0;
        let mut chars = s.chars().collect::<Vec<char>>();
        while i < s.len() {
            if chars[i] != ' ' {
                if slow != 0 {
                    chars[slow] = ' ';
                    slow += 1;
                }
                while i < s.len() && chars[i] != ' ' {
                    chars[slow] = chars[i];
                    slow += 1;
                    i += 1;
                }
            }
            i += 1;
        }
        chars.splice(0..slow, []).into_iter().collect()
    }

    pub fn reverse_words(s: String) -> String {
        // s.trim()
        //     .to_string()
        //     .split_ascii_whitespace()
        //     .rev()
        //     .map(|x| x.to_string() + " ")
        //     .collect::<String>()
        //     .trim_end()
        //     .to_string()
        Self::move_space(s)
            .split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
        // Self::move_space(s).split_ascii_whitespace().rev()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_words() {
        // println!("{}", Solution::reverse_words("the sky is blue".to_string()));
        // println!("{}", vec![1, 2, 3] == vec![1, 2, 3]);
        println!("{}", Solution::move_space("    sky is    blue".to_string()));
    }
}
