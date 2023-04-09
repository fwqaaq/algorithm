/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; obstacle_grid[0].len()];
        for (i, &v) in obstacle_grid[0].iter().enumerate() {
            if v == 0 {
                dp[i] = 1;
            } else {
                break;
            }
        }
        for rows in obstacle_grid.iter().skip(1) {
            for j in 0..rows.len() {
                if rows[j] == 1 {
                    dp[j] = 0;
                } else if j != 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }
        dp.pop().unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(
            4,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0],
            ])
        );
    }
}
