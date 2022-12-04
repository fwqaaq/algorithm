/*
 * @lc app=leetcode.cn id=235 lang=rust
 *
 * [235] 二叉搜索树的最近公共祖先
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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    //*  递归
    // pub fn lowest_common_ancestor(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     let q_val = q.as_ref().unwrap().borrow().val;
    //     let p_val = p.as_ref().unwrap().borrow().val;
    //     let root_val = root.as_ref().unwrap().borrow().val;
    //     if root_val > q_val && root_val > p_val {
    //         return Self::lowest_common_ancestor(
    //             root.as_ref().unwrap().borrow().left.clone(),
    //             p,
    //             q,
    //         );
    //     };
    //     if root_val < q_val && root_val < p_val {
    //         return Self::lowest_common_ancestor(
    //             root.as_ref().unwrap().borrow().right.clone(),
    //             p,
    //             q,
    //         );
    //     }
    //     root
    // }

    //* 迭代 */
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        while let Some(node) = root.clone() {
            let root_val = node.borrow().val;
            if root_val > q_val && root_val > p_val {
                root = node.borrow().left.clone();
            } else if root_val < q_val && root_val < p_val {
                root = node.borrow().right.clone();
            } else {
                return root;
            }
        }
        None
    }
}
// @lc code=end
