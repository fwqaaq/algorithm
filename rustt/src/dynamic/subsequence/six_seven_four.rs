/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    //     let mut dp = vec![1; nums.len()];
    //     let mut res = 1;
    //     for i in 1..nums.len() {
    //         if nums[i] > nums[i - 1] {
    //             dp[i] = dp[i - 1] + 1;
    //             res = res.max(dp[i]);
    //         }
    //     }
    //     res
    // }

    // 贪心
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let (mut res, mut count) = (1, 1);
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
                res = res.max(count);
                continue;
            }
            count = 1;
        }
        res
    }
}
// @lc code=end
