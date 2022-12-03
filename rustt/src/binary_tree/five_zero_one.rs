/*
 * @lc app=leetcode.cn id=501 lang=rust
 *
 * [501] 二叉搜索树中的众数
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
    //* 递归 */
    // pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut count = 0;
    //     let mut max_count = 0;
    //     let mut res = vec![];
    //     let mut pre = i32::MIN;
    //     Self::search_bst(&root, &mut pre, &mut res, &mut count, &mut max_count);
    //     res
    // }
    // pub fn search_bst(
    //     cur: &Option<Rc<RefCell<TreeNode>>>,
    //     mut pre: &mut i32,
    //     res: &mut Vec<i32>,
    //     count: &mut i32,
    //     max_count: &mut i32,
    // ) {
    //     if cur.is_none() {
    //         return;
    //     }
    //     let cur_node = cur.as_ref().unwrap().borrow();
    //     Self::search_bst(&cur_node.left, pre, res, count, max_count);
    //     if *pre == i32::MIN {
    //         *count = 1;
    //     } else if *pre == cur_node.val {
    //         *count += 1;
    //     } else {
    //         *count = 1;
    //     };
    //     match count.cmp(&max_count) {
    //         std::cmp::Ordering::Equal => res.push(cur_node.val),
    //         std::cmp::Ordering::Greater => {
    //             *max_count = *count;
    //             res.clear();
    //             res.push(cur_node.val);
    //         }
    //         _ => {}
    //     };
    //     *pre = cur_node.val;
    //     Self::search_bst(&cur_node.right, pre, res, count, max_count);
    // }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let (mut cur, mut pre) = (root, i32::MIN);
        let mut res = vec![];
        let mut stack = vec![];
        let (mut count, mut max_count) = (0, 0);
        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                if pre == i32::MIN {
                    count = 1;
                } else if pre == node.borrow().val {
                    count += 1;
                } else {
                    count = 1;
                }
                match count.cmp(&max_count) {
                    std::cmp::Ordering::Equal => res.push(node.borrow().val),
                    std::cmp::Ordering::Greater => {
                        max_count = count;
                        res.clear();
                        res.push(node.borrow().val);
                    }
                    _ => {}
                }
                pre = node.borrow().val;
                cur = node.borrow().right.clone();
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
    fn test_find_mode() {
        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        let res = Solution::find_mode(Some(Rc::new(RefCell::new(root))));

        assert_eq!(res, vec![2]);
    }
}
