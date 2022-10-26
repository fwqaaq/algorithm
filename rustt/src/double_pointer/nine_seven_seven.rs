/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.into_iter().map(|x| x * x).collect::<Vec<i32>>();
        nums.sort();
        nums
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_squares() {
        let nums = vec![-4, -1, 0, 3, 10];
        let res = Solution::sorted_squares(nums);
        assert_eq!(res, vec![0, 1, 9, 16, 100]);
    }
}
