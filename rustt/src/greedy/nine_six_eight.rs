/*
 * @lc app=leetcode.cn id=968 lang=rust
 *
 * [968] 监控二叉树
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
    pub fn traversal(cur: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(node) = cur {
            let left = Self::traversal(&node.borrow().left, res);
            let right = Self::traversal(&node.borrow().right, res);
            if left == 2 && right == 2 {
                return 0;
            } else if left == 0 || right == 0 {
                *res += 1;
                return 1;
            }
        }
        2
    }
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if Self::traversal(&root, &mut res) == 0 {
            res += 1;
        }
        res
    }
}
// @lc code=end
