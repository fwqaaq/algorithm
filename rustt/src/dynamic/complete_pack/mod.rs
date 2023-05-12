mod five_one_eight;
mod seven_zero;
mod three_seven_seven;
mod three_two_two;

struct Solution;

impl Solution {
    // 先遍历物品
    fn complete_pack() {
        let (goods, bag_size) = (vec![(1, 15), (3, 20), (4, 30)], 4);
        let mut dp = vec![0; bag_size + 1];
        for (weight, value) in goods {
            for j in weight..=bag_size {
                dp[j] = dp[j].max(dp[j - weight] + value);
            }
        }
        println!("先遍历物品：{}", dp[bag_size]);
    }

    // 先遍历背包
    fn complete_pack_after() {
        let (goods, bag_size) = (vec![(1, 15), (3, 20), (4, 30)], 4);
        let mut dp = vec![0; bag_size + 1];
        for i in 0..=bag_size {
            for (weight, value) in &goods {
                if i >= *weight {
                    dp[i] = dp[i].max(dp[i - weight] + value);
                }
            }
        }
        println!("先遍历背包：{}", dp[bag_size]);
    }
}

#[test]
fn test_complete_pack() {
    Solution::complete_pack();
    Solution::complete_pack_after();
}
