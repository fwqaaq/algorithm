/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root = preorder[0];
        let index = inorder.iter().position(|&x| x == root).unwrap();
        let mut root = TreeNode::new(root);
        root.left = Self::build_tree(preorder[1..index + 1].to_vec(), inorder[0..index].to_vec());
        root.right = Self::build_tree(
            preorder[index + 1..].to_vec(),
            inorder[index + 1..].to_vec(),
        );
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
