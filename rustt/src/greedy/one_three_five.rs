/*
 * @lc app=leetcode.cn id=135 lang=rust
 *
 * [135] 分发糖果
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1i32; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_candy() {
        Solution::candy(vec![1, 3, 4, 5, 2]);
    }
}
