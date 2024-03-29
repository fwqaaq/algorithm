/*
 * @lc app=leetcode.cn id=106 lang=rust
 *
 * [106] 从中序与后序遍历序列构造二叉树
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let mut postorder = postorder;
        let root = postorder.pop().unwrap();
        let index = inorder.iter().position(|&x| x == root).unwrap();
        let mut root = TreeNode::new(root);
        root.left = Self::build_tree(inorder[..index].to_vec(), postorder[..index].to_vec());
        root.right = Self::build_tree(inorder[index + 1..].to_vec(), postorder[index..].to_vec());
        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_tree() {
        let res = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        println!("{:?}", res);
    }
}
