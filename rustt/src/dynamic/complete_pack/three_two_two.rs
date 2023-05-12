/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */
pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    //     let amount = amount as usize;
    //     let mut dp = vec![i32::MAX; amount + 1];
    //     dp[0] = 0;
    //     for coin in coins {
    //         for i in coin as usize..=amount {
    //             if dp[i - coin as usize] != i32::MAX {
    //                 dp[i] = dp[i].min(dp[i - coin as usize] + 1);
    //             }
    //         }
    //     }
    //     if dp[amount] == i32::MAX {
    //         return -1;
    //     }
    //     dp[amount]
    // }

    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;
        for i in 0..=amount {
            for &coin in &coins {
                if i >= coin as usize && dp[i - coin as usize] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1)
                }
            }
        }
        if dp[amount] == i32::MAX {
            return -1;
        }
        dp[amount]
    }
}
// @lc code=end
#[test]
fn test_coin_change() {
    let res = Solution::coin_change(vec![2], 3);
    println!("{res}");
}
