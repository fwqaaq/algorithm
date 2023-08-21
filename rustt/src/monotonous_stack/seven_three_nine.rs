/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res) = (vec![], vec![0; temperatures.len()]);

        for (i, &value) in temperatures.iter().enumerate() {
            while let Some(&top) = stack.last() {
                // 当栈值大于当前值时，跳出循环
                if temperatures[top] >= value {
                    break;
                }
                // 当栈值小于当前值时，弹出栈顶元素，并计算索引差值
                let saved_index = stack.pop().unwrap();
                res[saved_index] = (i - saved_index) as i32;
            }
            stack.push(i);
        }
        res
    }
}
// @lc code=end

#[test]
fn test_daily_temperatures() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let res = Solution::daily_temperatures(temperatures);
    assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
}
