/*
 * @lc app=leetcode.cn id=530 lang=rust
 *
 * [530] 二叉搜索树的最小绝对差
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
    //* 辅助数组 */
    //     pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //         let mut vec = vec![];
    //         Self::traversal(root, &mut vec);
    //         let mut min = i32::MAX;
    //         for i in 1..vec.len() {
    //             min = min.min(vec[i] - vec[i - 1])
    //         }
    //         min
    //     }
    //     pub fn traversal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    //         if root.is_none() {
    //             return;
    //         }
    //         let node = root.as_ref().unwrap().borrow();
    //         Self::traversal(node.left.clone(), v);
    //         v.push(node.val);
    //         Self::traversal(node.right.clone(), v);
    //     }

    //* 递归中 */
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut pre = None;
        let mut min = i32::MAX;
        Self::inorder(root, &mut pre, &mut min);
        min
    }
    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>, min: &mut i32) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        Self::inorder(node.left.clone(), pre, min);
        if let Some(pre) = pre {
            *min = (node.val - *pre).min(*min);
        }
        *pre = Some(node.val);

        Self::inorder(node.right.clone(), pre, min);
    }

    //*迭代 */
    // pub fn get_minimum_difference(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_none() {
    //         return 0;
    //     }
    //     let mut stack = vec![];
    //     let mut pre = -1;
    //     let mut res = i32::MAX;
    //     while root.is_some() || !stack.is_empty() {
    //         while let Some(node) = root {
    //             root = node.borrow().left.clone();
    //             stack.push(node);
    //         }

    //         let node = stack.pop().unwrap();

    //         if pre >= 0 {
    //             res = res.min(node.borrow().val - pre);
    //         }

    //         pre = node.borrow().val;

    //         root = node.borrow().right.clone();
    //     }
    //     res
    // }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_minimum_difference() {
        let root = TreeNode {
            val: 543,
            //[543,384,652,null,445,null,699]
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 384,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 445,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 652,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(699)))),
            }))),
        };

        println!(
            "{}",
            Solution::get_minimum_difference(Some(Rc::new(RefCell::new(root))))
        );
    }
}
