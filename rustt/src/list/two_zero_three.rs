/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
 */
pub struct Solution;
// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T, node: Option<Box<ListNode<T>>>) -> Self {
        ListNode { next: node, val }
    }
}
impl Solution {
    pub fn remove_elements(
        head: Option<Box<ListNode<i32>>>,
        val: i32,
    ) -> Option<Box<ListNode<i32>>> {
        let mut dummy_head = Box::new(ListNode::new(0, None));
        dummy_head.next = head;
        let mut cur = dummy_head.as_mut();
        while let Some(node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next;
            } else {
                // node.val != val, keep it
                cur.next = Some(node);
                cur = cur.next.as_mut()?;
            }
        }
        dummy_head.next
    }
}
// @lc code=end

#[cfg(test)]
mod tsets {
    use super::*;
    #[test]
    fn test_remove_elements() {
        let head = Box::new(ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    6,
                    Some(Box::new(ListNode::new(
                        3,
                        Some(Box::new(ListNode::new(
                            4,
                            Some(Box::new(ListNode::new(
                                5,
                                Some(Box::new(ListNode::new(6, None))),
                            ))),
                        ))),
                    ))),
                ))),
            ))),
        ));
        let res = Solution::remove_elements(Some(head), 6);
        println!("{:?}", res);
    }
}
