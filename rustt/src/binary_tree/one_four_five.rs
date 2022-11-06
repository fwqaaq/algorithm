/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] 二叉树的后序遍历
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::traverse(&root, &mut res);
        res
    }

    pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::traverse(&node.borrow().left, res);
            Self::traverse(&node.borrow().right, res);
            res.push(node.borrow().val);
        }
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
        let res = Solution::postorder_traversal(Some(Rc::new(RefCell::new(root))));
        assert_eq!(res, vec![3, 2, 1]);
    }
}
