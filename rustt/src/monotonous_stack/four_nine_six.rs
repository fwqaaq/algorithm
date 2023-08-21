/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */
pub struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut res, mut map) = (vec![-1; nums1.len()], HashMap::new());
        if nums1.is_empty() {
            return res;
        }

        nums1.into_iter().enumerate().for_each(|(v, k)| {
            map.insert(k, v);
        });

        let mut stack = vec![];
        for (i, &value) in nums2.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if value <= nums2[top] {
                    break;
                }
                let stacked_index = stack.pop().unwrap();
                // stacked_index 是 nums2 的栈内索引，此时一定 > i
                // 如果 stacked_index 对应 nums1 中有值，那么 value 一定大于这个值
                if let Some(&mapped_index) = map.get(&nums2[stacked_index]) {
                    res[mapped_index] = value;
                }
            }
            stack.push(i);
        }

        res
    }
}
// @lc code=end
