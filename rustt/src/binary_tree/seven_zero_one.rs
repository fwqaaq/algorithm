/*
 * @lc app=leetcode.cn id=701 lang=rust
 *
 * [701] 二叉搜索树中的插入操作
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
    //* 迭代 */
    // pub fn insert_into_bst(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     val: i32,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if root.is_none() {
    //         return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    //     }
    //     let mut cur = root.clone();
    //     let mut pre = None;
    //     while let Some(node) = cur.clone() {
    //         pre = cur;
    //         if node.borrow().val > val {
    //             cur = node.borrow().left.clone();
    //         } else {
    //             cur = node.borrow().right.clone();
    //         };
    //     }
    //     let r = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    //     let mut p = pre.as_ref().unwrap().borrow_mut();
    //     if val < p.val {
    //         p.left = r;
    //     } else {
    //         p.right = r;
    //     }
    //     root
    // }

    //* 递归 */
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            if node.borrow().val > val {
                let left = Self::insert_into_bst(node.borrow_mut().left.take(), val);
                node.borrow_mut().left = left;
            } else {
                let right = Self::insert_into_bst(node.borrow_mut().right.take(), val);
                node.borrow_mut().right = right;
            }
            root
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
// @lc code=end
