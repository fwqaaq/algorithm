/*
* @lc app=leetcode.cn id=491 lang=rust
*
* [491] 递增子序列
*/
pub struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut path = vec![];
        Self::backtracking(&mut res, &mut path, &nums, 0);
        res.into_iter().collect()
    }

    pub fn backtracking(
        res: &mut HashSet<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_index: usize,
    ) {
        if path.len() > 1 {
            res.insert(path.clone());
        }

        // only when the numbers on the layer are the same
        let mut same_digit = HashSet::new();
        for i in start_index..nums.len() {
            if (!path.is_empty() && nums[i] < *path.last().unwrap())
                || same_digit.contains(&nums[i])
            {
                continue;
            }
            same_digit.insert(nums[i]);
            path.push(nums[i]);
            Self::backtracking(res, path, nums, i + 1);
            path.pop();
        }
    }
}
// @lc code=end
