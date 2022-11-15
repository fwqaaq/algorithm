/*
 * @lc app=leetcode.cn id=110 lang=rust
 *
 * [110] 平衡二叉树
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_depth(root) != -1
    }
    pub fn get_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let right = Self::get_depth(root.as_ref().unwrap().borrow().left.clone());
        let left = Self::get_depth(root.unwrap().borrow().right.clone());
        if right == -1 {
            return -1;
        }
        if left == -1 {
            return -1;
        }
        if (right - left).abs() > 1 {
            return -1;
        }

        1 + right.max(left)
    }
}
// @lc code=end
