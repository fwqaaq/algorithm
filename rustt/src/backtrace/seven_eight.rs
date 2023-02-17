/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtracking(&mut res, &mut path, &nums, 0);
        res
    }

    pub fn backtracking(
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_index: usize,
    ) {
        res.push(path.clone());
        // 回溯结束条件：根据 start_index = nums.len() 判断
        for i in start_index..nums.len() {
            path.push(nums[i]);
            Self::backtracking(res, path, nums, i + 1);
            path.pop();
        }
    }
}
// @lc code=end
