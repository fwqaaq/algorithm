/*
 * @lc app=leetcode.cn id=1049 lang=rust
 *
 * [1049] 最后一块石头的重量 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>();
        let target = sum as usize / 2;
        let mut dp = vec![0; target + 1];
        for s in stones {
            for j in (s as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j - s as usize] + s);
            }
        }
        sum - dp[target] * 2
    }
}
// @lc code=end

#[test]
fn test_last_stone_weight_ii() {
    let v = Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]);
    println!("{}", v);
}
