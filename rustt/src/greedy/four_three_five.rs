/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 * [435] 无重叠区间
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_by_key(|interval| interval[1]);
        let mut count = 1;
        let mut end = intervals[0][1];
        for v in intervals.iter().skip(1) {
            if end <= v[0] {
                end = v[1];
                count += 1;
            }
        }

        (intervals.len() - count) as i32
    }
}
// @lc code=end
