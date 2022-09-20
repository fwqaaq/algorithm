/*
 * @lc app=leetcode.cn id=904 lang=rust
 *
 * [904] 水果成篮
 */
pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, 0);
        let mut res = 0;
        while right < fruits.len() {
            let mut def_fruit = HashSet::new();
            while def_fruit.len() <= 2 {
                if right == fruits.len() {
                    break;
                }
                def_fruit.insert(fruits[right]);
                if def_fruit.len() == 3 {
                    break;
                }
                right += 1;
            }
            res = res.max(right - left);
            if right == fruits.len() {
                break;
            }
            left = right;
            while fruits[left - 1] == fruits[right - 1] {
                right -= 1;
            }
            left = right;
        }

        dbg!(res);
        res as i32
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_fruit() {
        let fruits = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
        let res = Solution::total_fruit(fruits);
        assert_eq!(res, 5);
        let fruits2 = vec![1, 2, 3, 2, 2];
        let res2 = Solution::total_fruit(fruits2);
        assert_eq!(res2, 4);
    }
}
