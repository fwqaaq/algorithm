/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hlep_map = HashMap::new();
        for (index, value) in nums.into_iter().enumerate() {
            if let Some(i) = hlep_map.get(&(target - value)) {
                return vec![*i as i32, index as i32];
            }
            hlep_map.insert(value, index);
        }
        panic!("No two sum solution")
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, vec![0, 1]);
    }
}
