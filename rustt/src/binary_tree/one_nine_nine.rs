/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * [199] 二叉树的右视图
 */
pub struct Solution;
// @lc code=start
//Definition for a binary tree node.
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let len = queue.len();
            for i in 0..len {
                let node = queue.pop_front().unwrap().unwrap();
                if i == len - 1 {
                    res.push(node.borrow().val);
                }
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
            }
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_preorder_traversal() {
        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let res = Solution::right_side_view(Some(Rc::new(RefCell::new(root))));
        assert_eq!(res, vec![1, 2, 3]);
    }
}
