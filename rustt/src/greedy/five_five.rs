/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let (mut i, mut cover) = (0, 0);
        while i <= cover {
            cover = (i + nums[i] as usize).max(cover);
            if cover >= nums.len() - 1 {
                return true;
            }
            i += 1;
        }
        false
    }
}
// @lc code=end
