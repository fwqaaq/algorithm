pub struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        chars[..n as usize].reverse();
        chars[n as usize..].reverse();
        chars.reverse();
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_left_words() {
        assert_eq!(
            Solution::reverse_left_words("abcdefg".to_string(), 2),
            "cdefgab".to_string()
        );
    }
}
