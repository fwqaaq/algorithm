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
            // nums[i] < left < right, if nums[i] is greater 0, then invalid
            if nums[i] > 0 {
                return res;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let total = nums[i] + nums[left] + nums[right];
                match total.cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while nums[left] == nums[left - 1] && left < right {
                            left += 1;
                        }
                        while nums[right] == nums[right + 1] && left < right {
                            right -= 1;
                        }
                    }
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
            }
        }
        println!("{:?}", res);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three_num() {
        let nums = vec![0, 0, 0];
        let res = Solution::three_sum(nums);
        assert_eq!(res, vec![vec![0, 0, 0]]);
    }
}
