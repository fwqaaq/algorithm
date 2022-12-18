/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        sum: i32,
        start_index: usize,
    ) {
        if sum == target {
            result.push(path.to_vec());
            return;
        }
        if sum > target {
            return;
        }
        for i in start_index..candidates.len() {
            if sum + candidates[i] <= target {
                if i > start_index && candidates[i] == candidates[i - 1] {
                    continue;
                }
                path.push(candidates[i]);
                Self::backtracking(result, path, candidates, target, sum + candidates[i], i + 1);
                path.pop();
            }
        }
    }
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        candidates.sort();
        Self::backtracking(&mut res, &mut path, &candidates, target, 0, 0);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2() {
        let res = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        println!("{:?}", res);
    }
}
