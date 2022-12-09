/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let index = nums.len() / 2;
        let mut root = TreeNode::new(nums[index]);

        root.left = Self::sorted_array_to_bst(nums[..index].to_vec());
        root.right = Self::sorted_array_to_bst(nums[index + 1..].to_vec());
        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
