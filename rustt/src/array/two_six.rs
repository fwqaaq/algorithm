/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[count] {
                count += 1;
                nums[count] = nums[i];
            }
        }
        (count + 1) as i32
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 3, 4, 4, 5, 5]);
    }
}
