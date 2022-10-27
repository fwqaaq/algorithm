/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = vec![];
        nums.sort();
        for i in 0..nums.len() {
            if nums[i] > target && nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] > target && nums[j] + nums[i] > 0 {
                    break;
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let total = nums[i] + nums[j] + nums[left] + nums[right];
                    match total.cmp(&target) {
                        std::cmp::Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            left += 1;
                            right -= 1;
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        }
                        std::cmp::Ordering::Less => left += 1,
                        std::cmp::Ordering::Greater => right -= 1,
                    }
                }
            }
        }
        res
    }
}
// @lc code=end
