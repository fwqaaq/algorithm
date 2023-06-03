/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            _ => Self::rob_range(&nums, 0, nums.len() - 2).max(Self::rob_range(
                &nums,
                1,
                nums.len() - 1,
            )),
        }
    }

    pub fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if start == end {
            return nums[start];
        }
        let mut dp = vec![0; nums.len()];
        dp[start] = nums[start];
        dp[start + 1] = nums[start].max(nums[start + 1]);
        for i in start + 2..=end {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        dp[end]
    }
}
// @lc code=end
