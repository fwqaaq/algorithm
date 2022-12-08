/*
 * @lc app=leetcode.cn id=669 lang=rust
 *
 * [669] 修剪二叉搜索树
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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;
        let mut node = root.as_ref().unwrap().borrow_mut();
        if node.val < low {
            return Self::trim_bst(node.right.clone(), low, high);
        }
        if node.val > high {
            return Self::trim_bst(node.left.clone(), low, high);
        }

        node.left = Self::trim_bst(node.left.clone(), low, high);
        node.right = Self::trim_bst(node.right.clone(), low, high);
        drop(node);
        root
    }
}
// @lc code=end
