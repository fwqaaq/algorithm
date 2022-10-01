/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let mut cur = head;
        // let mut pre = None;
        // while let Some(mut node) = cur.take() {
        //     cur = node.next;
        //     node.next = pre;
        //     pre = Some(node);
        // }
        // pre

        fn rev(
            mut head: Option<Box<ListNode>>,
            mut pre: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head.take() {
                let cur = node.next;
                node.next = pre;
                pre = Some(node);
                return rev(cur, pre);
            }
            pre
        }
        rev(head, None)
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let head = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        };
        let res = Solution::reverse_list(Some(Box::new(head)));
        println!("{:?}", res);
    }
}
