/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
    //*  递归
    // pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     Self::valid_bst(i64::MIN, i64::MAX, root)
    // }
    // pub fn valid_bst(low: i64, upper: i64, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     if root.is_none() {
    //         return true;
    //     }
    //     let root = root.as_ref().unwrap().borrow();
    //     if root.val as i64 <= low || root.val as i64 >= upper {
    //         return false;
    //     }
    //     Self::valid_bst(low, root.val as i64, root.left.clone())
    //         && Self::valid_bst(root.val as i64, upper, root.right.clone())
    // }

    //* 辅助数组 */
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut vec = vec![];
        Self::valid_bst(root, &mut vec);
        for i in 1..vec.len() {
            if vec[i] <= vec[i - 1] {
                return false;
            }
        }
        true
    }
    pub fn valid_bst(root: Option<Rc<RefCell<TreeNode>>>, mut v: &mut Vec<i64>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        Self::valid_bst(node.left.clone(), v);
        v.push(node.val as i64);
        Self::valid_bst(node.right.clone(), v);
    }
}
// @lc code=end
