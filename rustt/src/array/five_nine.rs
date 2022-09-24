/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];
        let (mut start_x, mut start_y, mut offset) = (0, 0, 1);
        let mut loop_index = n / 2;
        let mut count = 1;
        let mut i;
        let mut j;
        while loop_index > 0 {
            i = start_x;
            j = start_y;
            while j < n as usize - offset {
                res[i][j] = count;
                count += 1;
                j += 1;
            }
            while i < (n as usize) - offset {
                res[i][j] = count;
                count += 1;
                i += 1;
            }

            while j > start_y {
                res[i][j] = count;
                count += 1;
                j -= 1;
            }

            while i > start_x {
                res[i][j] = count;
                count += 1;
                i -= 1;
            }

            start_x += 1;
            start_y += 1;
            offset += 1;
            loop_index -= 1;
        }
        if n % 2 == 1 {
            res[n as usize / 2][n as usize / 2] = count;
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_squares() {
        let n = 4;
        let res = Solution::generate_matrix(n);
        println!("{:?}", res);
    }
}
