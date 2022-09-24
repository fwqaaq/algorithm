/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 */
pub struct Solution;
// @lc code=start
impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let (mut start_left, mut start_right) = (0, matrix[0].len());
        let (mut top, mut down) = (0, matrix.len());
        while res.len() < matrix[0].len() * matrix.len() {
            for j in start_left..start_right {
                res.push(matrix[top][j]);
            }
            top += 1;

            for j in top..down {
                res.push(matrix[j][start_right - 1]);
            }
            start_right -= 1;

            if down <= top {
                break;
            }

            for j in (start_left..start_right).rev() {
                res.push(matrix[down - 1][j]);
            }
            down -= 1;

            if start_right <= start_left {
                break;
            }

            for j in (top..down).rev() {
                res.push(matrix[j][start_left]);
            }
            start_left += 1;
        }

        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![2, 5, 8], vec![4, 0, -1]];
        let res = Solution::spiral_order(matrix);
        assert_eq!(res, vec![2, 5, 8, -1, 0, 4]);
    }
}
