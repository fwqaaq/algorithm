/*
 * @lc app=leetcode.cn id=513 lang=rust
 *
 * [513] 找树左下角的值
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    //* 层序遍历 */
    // pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut queue = VecDeque::new();
    //     let mut res = 0;
    //     if root.is_some() {
    //         queue.push_back(root);
    //     }
    //     while !queue.is_empty() {
    //         for i in 0..queue.len() {
    //             let node = queue.pop_front().unwrap().unwrap();
    //             if i == 0 {
    //                 res = node.borrow().val;
    //             }
    //             if node.borrow().left.is_some() {
    //                 queue.push_back(node.borrow().left.clone());
    //             }
    //             if node.borrow().right.is_some() {
    //                 queue.push_back(node.borrow().right.clone());
    //             }
    //         }
    //     }
    //     res
    // }

    //*递归*/
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut max_depth = i32::MIN;
        Self::traversal(root, 0, &mut max_depth, &mut res);
        res
    }
    fn traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: &mut i32,
        res: &mut i32,
    ) {
        let node = root.unwrap();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            if depth > *max_depth {
                *max_depth = depth;
                *res = node.borrow().val;
            }
            return;
        }
        if node.borrow().left.is_some() {
            Self::traversal(node.borrow().left.clone(), depth + 1, max_depth, res);
        }
        if node.borrow().right.is_some() {
            Self::traversal(node.borrow().right.clone(), depth + 1, max_depth, res);
        }
    }
}
// @lc code=end
