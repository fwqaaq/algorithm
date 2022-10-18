/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] 快乐数
 */
pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn get_sum(mut value: i32) -> i32 {
        let mut sum = 0;
        while value > 0 {
            let temp = value % 10;
            sum += temp.pow(2);
            value /= 10;
        }
        sum
    }
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        loop {
            let sum = Solution::get_sum(n);
            if sum == 1 {
                return true;
            }
            if set.contains(&sum) {
                return false;
            }
            set.insert(sum);
            n = sum
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_happy() {
        assert!(Solution::is_happy(19));
    }
}
