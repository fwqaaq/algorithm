/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

use std::backtrace;

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrace(&mut res, &mut path, n, k, 1);
        res
    }

    pub fn backtrace(
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target_num: i32,
        count: i32,
        start_index: i32,
    ) {
        if count as usize == path.len() {
            res.push(path.to_vec());
            return;
        }

        for i in start_index..=target_num - (count - path.len() as i32) + 1 {
            path.push(i);
            Self::backtrace(res, path, target_num, count, i + 1);
            path.pop();
        }
    }
}
// @lc code=end
