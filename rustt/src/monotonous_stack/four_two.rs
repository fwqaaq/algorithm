/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut max_left, mut max_right, mut res) =
            (0, height.len() - 1, 0, 0, 0);

        // 将墙往中间推动，每次推动都会有一个“墙”被推动，这个“墙”是当前较矮的那个
        while left < right {
            max_left = max_left.max(height[left]);
            max_right = max_right.max(height[right]);
            // 这里左指针右移是因为左“墙”较矮，左边这一片实际情况下的盛水量是受制于这个矮的左“墙”的
            // 而较高的右边在实际情况下的限制条件可能不是当前的左“墙”，比如限制条件可能是右“墙”，就能装更高的水，
            if max_left < max_right {
                res += max_left - height[left];
                left += 1;
            } else {
                res += max_right - height[right];
                right -= 1;
            }
        }
        res
    }
}
// @lc code=end
