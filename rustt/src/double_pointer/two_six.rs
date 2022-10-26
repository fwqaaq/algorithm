/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[slow] {
                slow += 1;
                nums[slow] = nums[i];
            }
        }
        (slow + 1) as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_elements() {
        let mut nums = vec![1, 1, 2];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(nums[..len as usize], [1, 2]);
    }
}
