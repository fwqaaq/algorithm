/*
 * @lc app=leetcode.cn id=538 lang=rust
 *
 * [538] 把二叉搜索树转换为累加树
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
    //* 递归*/
    // pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let mut pre = 0;
    //     Self::traversal(&root, &mut pre);
    //     root
    // }
    // pub fn traversal(cur: &Option<Rc<RefCell<TreeNode>>>, pre: &mut i32) {
    //     if cur.is_none() {
    //         return;
    //     }
    //     let mut node = cur.as_ref().unwrap().borrow_mut();
    //     Self::traversal(&node.right, pre);
    //     *pre += node.val;
    //     node.val = *pre;
    //     Self::traversal(&node.left, pre);
    // }

    //* 迭代 */
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root.clone();
        let mut stack = vec![];
        let mut pre = 0;
        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow().right.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                pre += node.borrow().val;
                node.borrow_mut().val = pre;
                cur = node.borrow().left.clone();
            }
        }
        root
    }
}
// @lc code=end
