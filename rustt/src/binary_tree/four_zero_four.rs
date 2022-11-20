/*
 * @lc app=leetcode.cn id=404 lang=rust
 *
 * [404] 左叶子之和
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
    // 递归
    // pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut res = 0;
    //     if let Some(node) = root {
    //         if let Some(left) = &node.borrow().left {
    //             if left.borrow().right.is_none() && left.borrow().right.is_none() {
    //                 res += left.borrow().val;
    //             }
    //         }
    //         res + Self::sum_of_left_leaves(node.borrow().left.clone())
    //             + Self::sum_of_left_leaves(node.borrow().right.clone())
    //     } else {
    //         0
    //     }
    // }
    // 迭代
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                if let Some(left) = &node.borrow().left {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        res += left.borrow().val;
                    }
                    stack.push(Some(left.to_owned()));
                }
                if let Some(right) = &node.borrow().right {
                    stack.push(Some(right.to_owned()));
                }
            }
        }
        res
    }
}
