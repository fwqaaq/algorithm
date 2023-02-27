use std::vec;

/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];
        nums.sort();
        Self::backtracking(&mut res, &mut path, &mut used, &nums);
        res
    }
    pub fn backtracking(
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        nums: &Vec<i32>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
        }

        for i in 0..nums.len() {
            // used[i - 1] == true: 对树层进行去重
            // used[i - 1] == false: 对树枝进行去重
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            if !used[i] {
                used[i] = true;
                path.push(nums[i]);
                Self::backtracking(res, path, used, nums);
                path.pop();
                used[i] = false;
            }
        }
    }
}
// @lc code=end
