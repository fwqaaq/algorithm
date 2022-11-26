/*
 * @lc app=leetcode.cn id=617 lang=rust
 *
 * [617] 合并二叉树
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
    //* 递归 */
    // pub fn merge_trees(
    //     root1: Option<Rc<RefCell<TreeNode>>>,
    //     root2: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if root1.is_none() {
    //         return root2;
    //     }
    //     if root2.is_none() {
    //         return root1;
    //     }
    //     let binding = root1.clone();
    //     let mut node1 = binding.as_ref().unwrap().borrow_mut();
    //     let node2 = root2.as_ref().unwrap().borrow_mut();
    //     node1.left = Self::merge_trees(node1.left.clone(), node2.left.clone());
    //     node1.right = Self::merge_trees(node1.right.clone(), node2.right.clone());
    //     node1.val += node2.val;
    //     root1
    // }

    //* 迭代 */
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }
        let mut stack = vec![];
        stack.push(root2);
        stack.push(root1.clone());
        while !stack.is_empty() {
            let node1 = stack.pop().unwrap().unwrap();
            let node2 = stack.pop().unwrap().unwrap();
            let mut node1 = node1.borrow_mut();
            let node2 = node2.borrow();
            node1.val += node2.val;
            if node1.left.is_some() && node2.left.is_some() {
                stack.push(node2.left.clone());
                stack.push(node1.left.clone());
            }
            if node1.right.is_some() && node2.right.is_some() {
                stack.push(node2.right.clone());
                stack.push(node1.right.clone());
            }
            if node1.left.is_none() && node2.left.is_some() {
                node1.left = node2.left.clone();
            }
            if node1.right.is_none() && node2.right.is_some() {
                node1.right = node2.right.clone();
            }
        }
        root1
    }
}
// @lc code=end
