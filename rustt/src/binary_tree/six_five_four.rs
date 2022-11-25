/*
 * @lc app=leetcode.cn id=654 lang=rust
 *
 * [654] 最大二叉树
 */

pub struct Solution;
// @lc code=start
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    //* 索引 */
    // pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     Self::traversal(&nums, 0, nums.len())
    // }
    // pub fn traversal(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
    //     if left >= right {
    //         return None;
    //     }
    //     let mut max_value_index = left;
    //     for i in left + 1..right {
    //         if nums[max_value_index] < nums[i] {
    //             max_value_index = i;
    //         }
    //     }
    //     let mut root = TreeNode::new(nums[max_value_index]);
    //     root.left = Self::traversal(nums, left, max_value_index);
    //     root.right = Self::traversal(nums, max_value_index + 1, right);
    //     Some(Rc::new(RefCell::new(root)))
    // }

    //* 数组 */
    pub fn construct_maximum_binary_tree(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mut max_value_index = 0;
        for i in 0..nums.len() {
            if nums[max_value_index] < nums[i] {
                max_value_index = i;
            }
        }
        let right = Self::construct_maximum_binary_tree(nums.split_off(max_value_index + 1));
        let root = nums.pop().unwrap();
        let left = Self::construct_maximum_binary_tree(nums);
        Some(Rc::new(RefCell::new(TreeNode {
            val: root,
            left,
            right,
        })))
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_maximum_binary_tree() {
        let res = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        println!("{:?}", res);
    }
}
