/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if target.abs() > sum {
            return 0;
        }
        if (target + sum) % 2 == 1 {
            return 0;
        }

        // 假设 ➕：x，那么 ➖：sum - x
        // target = x - (sum - x)，即总和为 x 的组合
        let size = (sum + target) as usize / 2;
        let mut dp = vec![0; size + 1];
        dp[0] = 1;
        for n in nums {
            for s in (n as usize..=size).rev() {
                // s -> 总和，dp[s] -> 组成总和的方式
                dp[s] += dp[s - n as usize];
            }
        }
        dp[size]
    }
}
// @lc code=end

#[test]
fn test_find_sum_ways() {
    let res = Solution::find_target_sum_ways(vec![3, 4, 6, 7, 10], 4);
    println!("{}", res);
}
