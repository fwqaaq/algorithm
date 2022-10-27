/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

pub struct Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                return res;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let total = nums[i] + nums[left] + nums[right];
                match total.cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }

                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
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
    fn test_three_num() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        assert_eq!(res, vec![[-1, -1, 2], [-1, 0, 1]]);
    }
}
