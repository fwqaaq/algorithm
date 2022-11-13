/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
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
    // pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     Self::recur(
    //         &root.as_ref().unwrap().borrow().left,
    //         &root.as_ref().unwrap().borrow().right,
    //     )
    // }
    // pub fn recur(
    //     left: &Option<Rc<RefCell<TreeNode>>>,
    //     right: &Option<Rc<RefCell<TreeNode>>>,
    // ) -> bool {
    //     match (left, right) {
    //         (None, None) => true,
    //         (Some(n1), Some(n2)) => {
    //             return n1.borrow().val == n2.borrow().val
    //                 && Self::recur(&n1.borrow().left, &n2.borrow().right)
    //                 && Self::recur(&n1.borrow().right, &n2.borrow().left)
    //         }
    //         _ => false,
    //     }
    // }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node.borrow().left.clone());
            queue.push_back(node.borrow().right.clone());
        }
        while !queue.is_empty() {
            let (n1, n2) = (queue.pop_front().unwrap(), queue.pop_front().unwrap());
            match (n1.clone(), n2.clone()) {
                (None, None) => continue,
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    }
                }
                _ => return false,
            };
            queue.push_back(n1.as_ref().unwrap().borrow().left.clone());
            queue.push_back(n2.as_ref().unwrap().borrow().right.clone());
            queue.push_back(n1.unwrap().borrow().right.clone());
            queue.push_back(n2.unwrap().borrow().left.clone());
        }
        true
    }
}
// @lc code=end
