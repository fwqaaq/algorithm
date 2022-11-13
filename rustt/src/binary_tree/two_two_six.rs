/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
            node.borrow_mut().left = Self::invert_tree(right);
            node.borrow_mut().right = Self::invert_tree(left);
        }
        root
    }
    //* 迭代 */
    // pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let mut stack = vec![root.clone()];
    //     while !stack.is_empty() {
    //         if let Some(node) = stack.pop().unwrap() {
    //             let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
    //             stack.push(right.clone());
    //             stack.push(left.clone());
    //             node.borrow_mut().left = right;
    //             node.borrow_mut().right = left;
    //         }
    //     }
    //     root
    // }
}
// @lc code=end
