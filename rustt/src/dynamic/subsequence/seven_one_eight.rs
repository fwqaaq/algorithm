/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut res, mut dp) = (0, vec![0; nums2.len()]);
        for n1 in nums1 {
            for j in (0..nums2.len()).rev() {
                if n1 == nums2[j] {
                    dp[j] = if j == 0 { 1 } else { dp[j - 1] + 1 };
                    res = res.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn test_find_length() {
    let nums1 = vec![1, 2, 3, 2, 1];
    let nums2 = vec![3, 2, 1, 4, 7];
    assert_eq!(Solution::find_length(nums1, nums2), 3);
}
