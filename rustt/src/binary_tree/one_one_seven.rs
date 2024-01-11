use std::{cell::RefCell, rc::Rc};

type Wrapper = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    val: i32,
    left: Wrapper,
    right: Wrapper,
}

impl TreeNode {
    pub fn new(val: i32, left: Wrapper, right: Wrapper) -> Wrapper {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

pub struct Solution;

impl Solution {
    pub fn sum(root: Wrapper) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                node.val + Self::sum(node.left.clone()) + Self::sum(node.right.clone())
            }
            None => 0,
        }
    }
}
