/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[slow] = nums[i];
                slow += 1;
            }
        }
        for i in nums.iter_mut().skip(slow) {
            *i = 0;
        }
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeros() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }
}
