/*
 * @lc app=leetcode.cn id=111 lang=rust
 *
 * [111] 二叉树的最小深度
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            res += 1;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return res;
                }
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
            }
        }
        res
    }
}
// @lc code=end
