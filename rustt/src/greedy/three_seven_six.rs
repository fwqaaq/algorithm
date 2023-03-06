/*
 * @lc app=leetcode.cn id=376 lang=rust
 *
 * [376] 摆动序列
 */
pub struct Solution;
// @lc code=start
// impl Solution {
//     pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
//         if nums.len() == 1 {
//             return 1;
//         }
//         let mut res = 1;
//         let mut pre_diff = 0;
//         for i in 0..nums.len() - 1 {
//             let cur_diff = nums[i + 1] - nums[i];
//             if (pre_diff <= 0 && cur_diff > 0) || (pre_diff >= 0 && cur_diff < 0) {
//                 res += 1;
//                 pre_diff = cur_diff;
//             }
//         }
//         res
//     }
// }
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let (mut down, mut up) = (1, 1);
        for i in 1..nums.len() {
            // i - 1 为峰顶
            if nums[i] < nums[i - 1] {
                down = down.max(up + 1);
            }
            // i - 1 为峰谷
            if nums[i] > nums[i - 1] {
                up = up.max(down + 1);
            }
        }
        down.max(up)
    }
}
// @lc code=end
