/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<HashSet<_>>()
            .intersection(&nums2.into_iter().collect::<HashSet<_>>())
            .copied()
            .collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection() {
        let res = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(res, vec![2]);
    }
}
