/*
 * @lc app=leetcode.cn id=115 lang=rust
 *
 * [115] 不同的子序列
 */
pub struct Solution;
// @lc code=start
impl Solution {
    // pub fn num_distinct(s: String, t: String) -> i32 {
    //     if s.len() < t.len() {
    //         return 0;
    //     }
    //     let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
    //     // i = 0, t 为空字符串，s 作为子序列的个数为 1（删除 s 所有元素）
    //     dp[0] = vec![1; s.len() + 1];
    //     for (i, char_t) in t.chars().enumerate() {
    //         for (j, char_s) in s.chars().enumerate() {
    //             if char_t == char_s {
    //                 // t 的前 i 个字符在 s 的前 j 个字符中作为子序列的个数
    //                 dp[i + 1][j + 1] = dp[i][j] + dp[i + 1][j];
    //                 continue;
    //             }
    //             dp[i + 1][j + 1] = dp[i + 1][j];
    //         }
    //     }
    //     dp[t.len()][s.len()]
    // }

    // 一维 dp
    pub fn num_distinct(s: String, t: String) -> i32 {
        if s.len() < t.len() {
            return 0;
        }
        let (s, t) = (s.into_bytes(), t.into_bytes());
        // 对于 t 为空字符串，s 作为子序列的个数为 1（删除 s 所有元素）
        let mut dp = vec![1; s.len() + 1];
        for char_t in t {
            // dp[i - 1][j - 1]，dp[j + 1] 更新之前的值
            // 第一次为 1，之后为 0
            let mut pre = dp[0];
            // 当开始遍历 t，s 的前 0 个字符无法包含任意子序列
            dp[0] = 0;
            for (j, &char_s) in s.iter().enumerate() {
                let temp = dp[j + 1];
                if char_t == char_s {
                    dp[j + 1] = pre + dp[j];
                } else {
                    dp[j + 1] = dp[j];
                }
                pre = temp;
            }
        }
        dp[s.len()]
    }
}
// @lc code=end

#[test]
fn test_num_distinct() {
    assert_eq!(
        Solution::num_distinct("ddd".to_string(), "dd".to_string()),
        3
    );
}
