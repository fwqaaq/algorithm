/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    //     let mut dp = vec![0; cost.len() + 1];
    //     for i in 2..=cost.len() {
    //         dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    //     }
    //     dp[cost.len()]
    // }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut dp_before, mut dp_after) = (0, 0);
        for i in 2..=cost.len() {
            let dpi = (dp_before + cost[i - 2]).min(dp_after + cost[i - 1]);
            dp_before = dp_after;
            dp_after = dpi;
        }
        dp_after
    }
}
// @lc code=end
