// pub struct Solution;
// #[derive(PartialEq, Eq, Debug)]
// pub struct TreeNode {
//     val: i32,
//     left: Option<Rc<RefCell<TreeNode>>>,
//     right: Option<Rc<RefCell<TreeNode>>>,
//     next: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> TreeNode {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//             next: None,
//         }
//     }
// }

// use std::cell::RefCell;
// use std::collections::VecDeque;
// use std::rc::Rc;

// impl Solution {
//     pub fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//         let mut queue = VecDeque::new();
//         if root.is_some() {
//             queue.push_back(root);
//         }

//         root
//     }
// }
