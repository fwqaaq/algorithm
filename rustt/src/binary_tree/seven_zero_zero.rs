/*
 * @lc app=leetcode.cn id=700 lang=rust
 *
 * [700] 二叉搜索树中的搜索
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
use std::cmp;
use std::rc::Rc;
impl Solution {
    //* 递归 */
    // pub fn search_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if root.is_none() || root.as_ref().unwrap().borrow().val == val {
    //         return root;
    //     }
    //     let node_val = root.as_ref().unwrap().borrow().val;
    //     if node_val > val {
    //         return Self::search_bst(root.as_ref().unwrap().borrow().left.clone(), val);
    //     }
    //     if node_val < val {
    //         return Self::search_bst(root.unwrap().borrow().right.clone(), val);
    //     }
    //     None
    // }

    //* 迭代 */
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(ref node) = root.clone() {
            match val.cmp(&node.borrow().val) {
                cmp::Ordering::Less => root = node.borrow().left.clone(),
                cmp::Ordering::Equal => return root,
                cmp::Ordering::Greater => root = node.borrow().right.clone(),
            };
        }
        None
    }
}
// @lc code=end
