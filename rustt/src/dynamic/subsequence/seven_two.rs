/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn min_distance(word1: String, word2: String) -> i32 {
    //     let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    //     for i in 1..=word2.len() {
    //         dp[0][i] = i;
    //     }
    //     for (j, v) in dp.iter_mut().enumerate().skip(1) {
    //         v[0] = j;
    //     }
    //     for (i, char1) in word1.chars().enumerate() {
    //         for (j, char2) in word2.chars().enumerate() {
    //             if char1 == char2 {
    //                 dp[i + 1][j + 1] = dp[i][j];
    //                 continue;
    //             }
    //             dp[i + 1][j + 1] = dp[i][j + 1].min(dp[i + 1][j]).min(dp[i][j]) + 1;
    //         }
    //     }
    //     dp[word1.len()][word2.len()] as i32
    // }

    // 一维 dp
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![0; word1.len() + 1];
        for (i, v) in dp.iter_mut().enumerate().skip(1) {
            *v = i;
        }

        for char2 in word2.chars() {
            // 相当于 dp[i][0] 的初始化
            let mut pre = dp[0];
            dp[0] += 1; // j = 0, 将前 i 个字符变成空串
            for (j, char1) in word1.chars().enumerate() {
                let temp = dp[j + 1];
                if char1 == char2 {
                    dp[j + 1] = pre;
                } else {
                    dp[j + 1] = dp[j + 1].min(dp[j]).min(pre) + 1;
                }
                pre = temp;
            }
        }

        dp[word1.len()] as i32
    }
}

// @lc code=end
#[test]
fn test_min_distance() {
    assert_eq!(
        Solution::min_distance("horse".to_string(), "ros".to_string()),
        3
    );
}
