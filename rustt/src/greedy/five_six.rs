/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if intervals.is_empty() {
            return res;
        }
        intervals.sort_by_key(|a| a[0]);
        res.push(intervals[0].clone());
        for interval in intervals.into_iter().skip(1) {
            let res_last_ele = res.last_mut().unwrap();
            if res_last_ele[1] >= interval[0] {
                res_last_ele[1] = interval[1].max(res_last_ele[1]);
            } else {
                res.push(interval);
            }
        }
        res
    }
}
// @lc code=end
