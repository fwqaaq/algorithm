/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut element = prices[0];
        // prices
        //     .iter()
        //     .skip(1)
        //     .fold(vec![], |mut v, &i| {
        //         v.push(i - element);
        //         element = i;
        //         v
        //     })
        //     .iter()
        //     .filter(|x| **x > 0)
        //     .sum()
        prices
            .iter()
            .skip(1)
            .fold((0, prices[0]), |(mut res, pre), &v| {
                if v > pre {
                    res += v - pre;
                }
                (res, v)
            })
            .0
    }
}
// @lc code=end
