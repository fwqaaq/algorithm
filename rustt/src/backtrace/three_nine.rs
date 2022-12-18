/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtracking(&mut res, &mut path, &candidates, target, 0, 0);
        res
    }
    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        condidates: &Vec<i32>,
        target: i32,
        sum: i32,
        start_index: usize,
    ) {
        if sum > target {
            return;
        }
        if sum == target {
            result.push(path.to_vec());
            return;
        }

        for i in start_index..condidates.len() {
            if sum + condidates[i] <= target {
                path.push(condidates[i]);
                Self::backtracking(result, path, condidates, target, sum + condidates[i], i);
                path.pop();
            }
        }
    }
}
// @lc code=end
