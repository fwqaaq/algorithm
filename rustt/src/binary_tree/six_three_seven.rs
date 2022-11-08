/*
 * @lc app=leetcode.cn id=637 lang=rust
 *
 * [637] 二叉树的层平均值
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let len = queue.len();
            let mut sum = 0_f64;
            for _ in 0..len {
                let node = queue.pop_front().unwrap().unwrap();
                sum += node.borrow().val as f64;
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
            }
            res.push(sum / len as f64);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    fn o(t: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(t)))
    }
    #[test]
    fn test_average_of_levels() {
        let root = o(TreeNode {
            val: 1,
            left: o(TreeNode::new(2)),
            right: o(TreeNode::new(3)),
        });
        assert_eq!(Solution::average_of_levels(root), vec![1.0, 2.5]);
    }
}
