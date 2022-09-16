/*
* @lc app=leetcode.cn id=704 lang=rust
*
* [704] 二分查找
*/
// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (right + left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid,
            }
        }
        -1
    }
}
// @lc code=end
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }
}
