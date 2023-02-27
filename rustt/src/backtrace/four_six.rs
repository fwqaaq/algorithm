/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];
        Self::backtraking(&mut res, &mut path, &mut used, &nums);
        res
    }

    pub fn backtraking(
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        nums: &Vec<i32>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::backtraking(res, path, used, nums);
            path.pop();
            used[i] = false;
        }
    }
}
// @lc code=end
