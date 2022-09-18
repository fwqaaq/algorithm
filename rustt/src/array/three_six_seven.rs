/*
 * @lc app=leetcode.cn id=367 lang=rust
 *
 * [367] 有效的完全平方数
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left = 0;
        let mut right = num;
        let mut ans = 0;
        while left <= right {
            let mid = (left + right) / 2;
            if mid as i64 * mid as i64 <= num as i64 {
                left = mid + 1;
                ans = mid;
            } else {
                right = mid - 1;
            }
        }
        ans * ans == num
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
        assert!(Solution::is_perfect_square(0));
    }
}
