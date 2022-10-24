pub struct Solution;
impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut count = 0;
        let mut chars: Vec<char> = s.chars().collect();
        for c in &chars {
            if *c == ' ' {
                count += 1;
            }
        }
        let mut len = s.len();
        let mut new_len = len + count * 2;
        chars.resize(new_len, ' ');
        while len < new_len {
            len -= 1;
            new_len -= 1;
            if chars[len] == ' ' {
                chars[new_len] = '0';
                chars[new_len - 1] = '2';
                chars[new_len - 2] = '%';
                new_len -= 2;
            } else {
                chars[new_len] = chars[len];
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_space() {
        assert_eq!(
            Solution::replace_space("We are happy.".to_string()),
            "We%20are%20happy.".to_string()
        );
    }
}
