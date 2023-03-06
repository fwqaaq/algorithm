/*
 * @lc app=leetcode.cn id=1005 lang=rust
 *
 * [1005] K 次取反后最大化的数组和
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_by_key(|b| std::cmp::Reverse(b.abs()));
        for v in nums.iter_mut() {
            if *v < 0 && k > 0 {
                *v *= -1;
                k -= 1;
            }
        }
        if k % 2 == 1 {
            *nums.last_mut().unwrap() *= -1;
        }
        nums.iter().sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_sum_after_k_negations() {
        Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3);
    }
}
