/*
 * @lc app=leetcode.cn id=452 lang=rust
 *
 * [452] 用最少数量的箭引爆气球
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_by_key(|point| point[0]);
        let mut result = 1;
        for i in 1..points.len() {
            if points[i][0] > points[i - 1][1] {
                result += 1;
            } else {
                points[i][1] = points[i][1].min(points[i - 1][1])
            }
        }
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min_arrow_shots() {
        let mut x = vec![1, 3, 2];
        x.sort_by_key(|&a| a);
        println!("{:?}", x);
    }
}
