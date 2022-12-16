/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

use std::path;

pub struct Solution;

// @lc code=start
impl Solution {
    fn backtrace(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        n: i32,
        k: i32,
        start_index: i32,
    ) {
        let len = path.len() as i32;
        if len == k {
            result.push(path.to_vec());
            return;
        }
        for i in start_index..=n - (k - len) + 1 {
            path.push(i);
            Self::backtrace(result, path, n, k, i + 1);
            path.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];
        Self::backtrace(&mut result, &mut path, n, k, 1);
        result
    }
}
// @lc code=end
