/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

use std::cmp::Ordering;

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return ((left + right) / 2) as i32,
                Ordering::Greater => right = mid,
            }
        }
        ((left + right) / 2) as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_test() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        // print!("{:?}", result);
        assert_eq!(result, 1);
    }
}
