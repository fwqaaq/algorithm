/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    //* 递归 */
    // pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    //     if root.is_none() {
    //         return false;
    //     }
    //     let node = root.unwrap();
    //     if node.borrow().left.is_none() && node.borrow().right.is_none() {
    //         return node.borrow().val == target_sum;
    //     }
    //     // Self::has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
    //     //     || Self::has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val)
    //     return Self::has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
    //         || Self::has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val);
    // }

    //* 迭代 */
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push((node.borrow().val, node.to_owned()));
        }
        while !stack.is_empty() {
            let (value, node) = stack.pop().unwrap();
            if node.borrow().left.is_none() && node.borrow().right.is_none() && value == target_sum
            {
                return true;
            }
            if node.borrow().left.is_some() {
                if let Some(r) = node.borrow().left.as_ref() {
                    stack.push((r.borrow().val + value, r.to_owned()));
                }
            }
            if node.borrow().right.is_some() {
                if let Some(r) = node.borrow().right.as_ref() {
                    stack.push((r.borrow().val + value, r.to_owned()));
                }
            }
        }
        false
    }
}
// @lc code=end
