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
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let node = root.unwrap();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return node.borrow().val == target_sum;
        }
        // Self::has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
        //     || Self::has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val)
        return Self::has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
            || Self::has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val);
    }
}
// @lc code=end
