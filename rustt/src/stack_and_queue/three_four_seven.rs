use std::vec;

/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */
pub struct Solution;
// @lc code=start
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k as usize);
        nums.into_iter().for_each(|k| {
            *hash.entry(k).or_insert(0) += 1;
        });

        for (k, v) in hash {
            if heap.len() == heap.capacity() {
                if *heap.peek().unwrap() < (Reverse(v), k) {
                    continue;
                } else {
                    heap.pop();
                }
            }
            heap.push((Reverse(v), k));
        }
        heap.into_iter().map(|(_, k)| k).collect()
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let res = Solution::top_k_frequent(nums, k);
        assert_eq!(res, vec![2, 1]);
    }
}
