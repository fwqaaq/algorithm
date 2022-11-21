/*
 * @lc app=leetcode.cn id=113 lang=rust
 *
 * [113] 路径总和 II
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut route = vec![];
        if root.is_none() {
            return res;
        } else {
            route.push(root.as_ref().unwrap().borrow().val);
        }
        Self::recur(
            &root,
            target_sum - root.as_ref().unwrap().borrow().val,
            &mut res,
            &mut route,
        );
        res
    }

    pub fn recur(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sum: i32,
        res: &mut Vec<Vec<i32>>,
        route: &mut Vec<i32>,
    ) {
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() && sum == 0 {
            res.push(route.to_vec());
            return;
        }
        if node.left.is_some() {
            let left = node.left.as_ref().unwrap();
            route.push(left.borrow().val);
            Self::recur(&node.left, sum - left.borrow().val, res, route);
            route.pop();
        }
        if node.right.is_some() {
            let right = node.right.as_ref().unwrap();
            route.push(right.borrow().val);
            Self::recur(&node.right, sum - right.borrow().val, res, route);
            route.pop();
        }
    }
}
// @lc code=end
