/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */
pub struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = VecDeque::with_capacity(k as usize);
        for (i, &v) in nums.iter().enumerate() {
            // 如果队列长度超过 k，那么需要移除队首过期元素
            if i - queue.front().unwrap_or(&0) == k as usize {
                queue.pop_front();
            }
            while let Some(&index) = queue.back() {
                if nums[index] >= v {
                    break;
                }
                // 如果队列第一个元素比当前元素小，那么就把队列第一个元素弹出
                queue.pop_back();
            }
            queue.push_back(i);
            if i >= k as usize - 1 {
                res.push(nums[queue[0]]);
            }
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sliding_window() {
        let res = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);
    }
}
