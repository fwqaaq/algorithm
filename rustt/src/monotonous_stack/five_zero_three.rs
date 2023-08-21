/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 * [503] 下一个更大元素 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res) = (vec![], vec![-1; nums.len()]);

        for i in 0..nums.len() * 2 {
            while let Some(&top) = stack.last() {
                if nums[i % nums.len()] <= nums[top] {
                    break;
                }
                let saved_index = stack.pop().unwrap();
                res[saved_index] = nums[i % nums.len()];
            }
            stack.push(i % nums.len());
        }

        res
    }
}
// @lc code=end
