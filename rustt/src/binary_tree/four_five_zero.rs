/*
 * @lc app=leetcode.cn id=450 lang=rust
 *
 * [450] 删除二叉搜索树中的节点
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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;

        let mut node = root.as_ref().unwrap().borrow_mut();
        match node.val.cmp(&key) {
            std::cmp::Ordering::Less => node.right = Self::delete_node(node.right.clone(), key),
            std::cmp::Ordering::Equal => match (node.left.clone(), node.right.clone()) {
                (None, None) => return None,
                (None, Some(r)) => return Some(r),
                (Some(l), None) => return Some(l),
                (Some(l), Some(r)) => {
                    let mut cur = Some(r.clone());
                    while let Some(n) = cur.clone().unwrap().borrow().left.clone() {
                        cur = Some(n);
                    }
                    cur.unwrap().borrow_mut().left = Some(l);
                    return Some(r);
                }
            },
            std::cmp::Ordering::Greater => node.left = Self::delete_node(node.left.clone(), key),
        }
        root.clone()
    }
}
// @lc code=end
