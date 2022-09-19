/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[count] = nums[i];
                count += 1;
            }
        }
        count as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3, 0, 9, 8];
        assert_eq!(Solution::remove_element(&mut nums, 3), 5);
        assert_eq!(nums, vec![2, 2, 0, 9, 8, 9, 8]);
    }
}
