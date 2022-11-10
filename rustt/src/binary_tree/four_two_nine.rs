pub struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Option<Rc<RefCell<Node>>> {
        Some(Rc::new(RefCell::new(Node {
            val,
            children: vec![],
        })))
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let mut temp = vec![];
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();
                temp.push(node.borrow().val);
                if !node.borrow().children.is_empty() {
                    for n in node.borrow().children.clone() {
                        queue.push_back(n);
                    }
                }
            }
            res.push(temp)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_level_order() {
        let root = Node {
            val: 1,
            children: vec![Node::new(2), Node::new(3), Node::new(4)],
        };
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(root)))),
            vec![vec![1], vec![2, 3, 4]]
        );
    }
}
