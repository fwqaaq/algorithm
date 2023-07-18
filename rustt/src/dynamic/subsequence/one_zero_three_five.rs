/*
 * @lc app=leetcode.cn id=1035 lang=rust
 *
 * [1035] 不相交的线
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    //     let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
    //     for (i, num1) in nums1.iter().enumerate() {
    //         for (j, num2) in nums2.iter().enumerate() {
    //             if num1 == num2 {
    //                 dp[i + 1][j + 1] = dp[i][j] + 1;
    //             } else {
    //                 dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
    //             }
    //         }
    //     }
    //     dp[nums1.len()][nums2.len()]
    // }

    // 一维 dp
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums2.len() + 1];
        for num1 in nums1 {
            let mut prev = 0;
            for (j, &num2) in nums2.iter().enumerate() {
                let temp = dp[j + 1];
                if num1 == num2 {
                    // 使用上一次的状态，防止重复计算
                    dp[j + 1] = prev + 1;
                } else {
                    dp[j + 1] = dp[j + 1].max(dp[j]);
                }
                prev = temp;
            }
        }
        dp[nums2.len()]
    }
}
// @lc code=end
