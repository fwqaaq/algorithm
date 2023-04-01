/*
 * @lc app=leetcode.cn id=738 lang=rust
 *
 * [738] 单调递增的数字
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n_bytes = n.to_string().into_bytes();
        let mut flag = n_bytes.len();
        for i in (1..n_bytes.len()).rev() {
            if n_bytes[i - 1] > n_bytes[i] {
                flag = i;
                n_bytes[i - 1] -= 1;
            }
        }
        for v in n_bytes.iter_mut().skip(flag) {
            *v = 57;
        }
        n_bytes
            .into_iter()
            .fold(0, |acc, x| acc * 10 + x as i32 - 48)
    }
}
// @lc code=end
