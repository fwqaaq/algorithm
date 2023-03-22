/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 * [860] 柠檬水找零
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);
        for v in bills {
            if v == 5 {
                five += 1;
            }
            if v == 10 {
                if five <= 0 {
                    return false;
                }
                five -= 1;
                ten += 1;
            }
            if v == 20 {
                if five >= 1 && ten >= 1 {
                    five -= 1;
                    ten -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
