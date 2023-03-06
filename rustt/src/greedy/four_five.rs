/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut cur_distance = 0;
        let mut ans = 0;
        let mut next_distance = 0;
        for (i, &n) in nums.iter().enumerate().take(nums.len() - 1) {
            next_distance = (n as usize + i).max(next_distance);
            if i == cur_distance {
                if cur_distance < nums.len() - 1 {
                    ans += 1;
                    cur_distance = next_distance;
                    if next_distance >= nums.len() - 1 {
                        break;
                    };
                } else {
                    break;
                }
                // cur_distance = next_distance;
                // ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
