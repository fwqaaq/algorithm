use std::result;

/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] 长度最小的子数组
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut res, mut sub_length) = (i32::MAX, 0);
        let (mut i, mut sum) = (0, 0);
        for (index, value) in nums.iter().enumerate() {
            sum += value;
            while sum >= target {
                //* get the sub length */
                sub_length = (index - i + 1) as i32;
                //* compare res length and sub length */
                res = res.min(sub_length as i32);
                sum -= nums[i];
                i += 1;
            }
        }
        if res == i32::MAX {
            return 0;
        }
        res as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 100;
        let res = Solution::min_sub_array_len(target, nums);
        assert_eq!(res, 0);
    }
}
