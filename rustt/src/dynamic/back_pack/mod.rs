pub struct Solution;

impl Solution {
    pub fn wei_bag_problem1(weight: Vec<usize>, value: Vec<usize>, bag_size: usize) -> usize {
        let mut dp = vec![vec![0; bag_size + 1]; weight.len()];
        for j in weight[0]..=weight.len() {
            dp[0][j] = value[0];
        }

        for i in 1..weight.len() {
            for j in 0..=bag_size {
                match j < weight[i] {
                    true => dp[i][j] = dp[i - 1][j],
                    false => dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - weight[i]] + value[i]),
                }
            }
        }
        dp[weight.len() - 1][bag_size]
    }

    pub fn wei_bag_problem2(weight: Vec<usize>, value: Vec<usize>, bag_size: usize) -> usize {
        let mut dp = vec![0; bag_size + 1];
        for i in 0..weight.len() {
            // 倒序
            for j in (weight[i]..=bag_size).rev() {
                if j >= weight[i] {
                    // 从后往前，因为状态由 dp[j - weight[i]] 保留，都会用 dp 开头的状态
                    // 如果 j 是顺序，会使用之前保留的状态，可能重复使用
                    dp[j] = dp[j].max(dp[j - weight[i]] + value[i]);
                }
            }
        }
        dp[dp.len() - 1]
    }
}

#[test]
fn test_wei_bag_problem2() {
    println!(
        "{}",
        Solution::wei_bag_problem2(vec![1, 3, 4], vec![15, 20, 30], 4)
    );
}
