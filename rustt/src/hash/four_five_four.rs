/*
 * @lc app=leetcode.cn id=454 lang=rust
 *
 * [454] 四数相加 II
 */
pub struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut helper_map = HashMap::new();
        let mut res = 0;
        for num1 in nums1 {
            for num2 in &nums2 {
                *helper_map.entry(num1 + num2).or_insert(0) += 1;
            }
        }

        for num3 in nums3 {
            for num4 in &nums4 {
                res += helper_map.get(&-(num3 + num4)).unwrap_or(&0);
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_four_sum_count() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        assert_eq!(Solution::four_sum_count(nums1, nums2, nums3, nums4), 2);
    }
}
