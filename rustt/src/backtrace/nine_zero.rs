/*
* @lc app=leetcode.cn id=90 lang=rust
*
* [90] 子集 II
*/
pub struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut path = vec![];
        nums.sort();
        Self::backtracking(&nums, &mut path, &mut res, 0);
        res.into_iter().collect()
    }

    pub fn backtracking(
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        res: &mut HashSet<Vec<i32>>,
        start_index: usize,
    ) {
        res.insert(path.clone());
        for i in start_index..nums.len() {
            path.push(nums[i]);
            Self::backtracking(nums, path, res, i + 1);
            path.pop();
        }
    }
}
// @lc code=end
