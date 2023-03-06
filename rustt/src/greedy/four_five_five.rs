/*
 * @lc app=leetcode.cn id=455 lang=rust
 *
 * [455] 分发饼干
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let (mut child, mut cookie) = (0, 0);
        while child < g.len() && cookie < s.len() {
            if g[child] <= s[cookie] {
                child += 1;
            }
            cookie += 1;
        }
        child as i32
    }
}
// @lc code=end
