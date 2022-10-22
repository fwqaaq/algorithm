/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
pub struct Solution;

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            // 剪枝优化，判断到 nums[i] 为正数时，一定大于 target
            // [-4,-3,-2,-1], target = -10 时，不能进行剪枝
            if nums[i] > target && nums[i] >= 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (i + 1)..nums.len() {
                //[-4,-1,-1,0,1,2], -1
                //这时候要和 i 组合，如果单独比较，很容易出现 j > target && j >= 0
                if nums[j] + nums[i] > target && nums[j] + nums[i] >= 0 {
                    break;
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let (mut left, mut right) = (j + 1, nums.len() - 1);
                while left < right {
                    let total = nums[i] + nums[j] + nums[left] + nums[right];
                    match total.cmp(&target) {
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            right -= 1;
                            left += 1;
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
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let res = Solution::four_sum(nums, target);
        assert_eq!(
            res,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test_four_sum_two() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let target = -1;
        let res = Solution::four_sum(nums, target);
        assert_eq!(res, vec![vec![-4, 0, 1, 2], vec![-1, -1, 0, 1]]);
    }
}
