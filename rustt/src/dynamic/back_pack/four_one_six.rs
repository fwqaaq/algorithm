/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![0; target + 1];
        for n in nums {
            for j in (n as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j - n as usize] + n)
            }
        }
        if dp[target] == target as i32 {
            return true;
        }
        false
    }
}
// @lc code=end

#[test]
fn test_cap_partition() {
    Solution::can_partition(vec![1, 5, 11, 5]);
}
