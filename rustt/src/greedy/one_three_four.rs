/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] 加油站
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        let mut cur_sum = 0;
        let mut start = 0;
        for i in 0..gas.len() {
            cur_sum += gas[i] - cost[i];
            total_sum += gas[i] - cost[i];
            if cur_sum < 0 {
                cur_sum = 0;
                start = i + 1;
            }
        }

        if total_sum < 0 {
            return -1;
        }
        start as i32
    }
}
// @lc code=end
