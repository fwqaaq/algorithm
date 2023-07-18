/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // let mut dp = vec![0; nums.len()];
        // dp[0] = nums[0];
        // let mut res = nums[0];
        // for i in 1..nums.len() {
        //     dp[i] = nums[i].max(dp[i - 1] + nums[i]);
        //     res = res.max(dp[i]);
        // }
        // res

        let (mut sum, mut res) = (0, i32::MIN);
        for v in nums {
            sum += v;
            res = res.max(sum);
            sum = sum.max(0);
        }
        res
    }
}
// @lc code=end
