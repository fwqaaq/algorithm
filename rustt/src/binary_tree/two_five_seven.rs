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
        Self::recur(&root, String::from(""), &mut res);
        res
    }
    pub fn recur(node: &Option<Rc<RefCell<TreeNode>>>, mut path: String, res: &mut Vec<String>) {
        let r = node.as_ref().unwrap().borrow();
        path += format!("{}", r.val).as_str();
        if r.left.is_none() && r.right.is_none() {
            res.push(path.to_string());
            return;
        }
        if r.left.is_some() {
            Self::recur(&r.left, path.clone() + "->", res);
        }
        if r.right.is_some() {
            Self::recur(&r.right, path + "->", res);
        }
    }
}
// @lc code=end
