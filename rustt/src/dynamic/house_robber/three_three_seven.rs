/*
 * @lc app=leetcode.cn id=337 lang=rust
 *
 * [337] 打家劫舍 III
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (v1, v2) = Self::rob_tree(&root);
        v1.max(v2)
    }
    pub fn rob_tree(cur: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match cur {
            None => (0, 0),
            Some(node) => {
                let left = Self::rob_tree(&node.borrow_mut().left);
                let right = Self::rob_tree(&node.borrow_mut().right);
                (
                    left.0.max(left.1) + right.0.max(right.1), // 考虑左右节点
                    node.borrow().val + left.0 + right.0,      // 偷父节点
                )
            }
        }
    }
}
// @lc code=end
