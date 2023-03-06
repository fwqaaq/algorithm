/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut cur = 0;
        for v in nums {
            cur += v;
            max_sum = cur.max(max_sum);
            // 如果 cur 是负数，连续和会变小，就需要重置
            cur = cur.max(0);
        }
        max_sum
    }
}
// @lc code=end
