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
        while slow < nums.len() {
            nums[slow] = 0;
            slow += 1;
        }
    }
}
// @lc code=end
