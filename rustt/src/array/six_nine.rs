/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        let mut ans = 0;
        //* consider bordering
        while left <= right {
            let mid = left + (right - left) / 2;

            if mid as i64 * mid as i64 <= x as i64 {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
        //(x as f64).sqrt() as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        println!("{}", Solution::my_sqrt(2147395599));
        // assert_eq!(Solution::my_sqrt(), 46339);
    }
}
