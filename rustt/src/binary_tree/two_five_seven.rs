/*
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = vec![];
        let mut path = vec![];
        Self::recur(&root, &mut path, &mut res);
        res
    }

    pub fn recur(
        root: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<String>,
        res: &mut Vec<String>,
    ) {
        let node = root.as_ref().unwrap().borrow();
        path.push(node.val.to_string());
        if node.left.is_none() && node.right.is_none() {
            res.push(path.join("->"));
            return;
        }
        if node.left.is_some() {
            Self::recur(&node.left, path, res);
            path.pop();
        }
        if node.right.is_some() {
            Self::recur(&node.right, path, res);
            path.pop();
        }
    }
}
// @lc code=end
