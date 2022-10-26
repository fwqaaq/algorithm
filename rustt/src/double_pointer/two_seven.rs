/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[slow] = nums[i];
                slow += 1;
            }
        }
        slow as i32
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_elements() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let len = Solution::remove_element(&mut nums, val);
        assert_eq!(len, 2);
        assert_eq!(nums[..len as usize], [2, 2]);
    }
}
