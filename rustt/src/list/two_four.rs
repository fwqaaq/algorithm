/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let mut dummy_head = Box::new(ListNode::new(0));
        // dummy_head.next = head;
        // let mut cur = dummy_head.as_mut();
        // while let Some(mut node) = cur.next.take() {
        //     if let Some(mut next) = node.next.take() {
        //         node.next = next.next.take();
        //         next.next = Some(node);
        //         cur.next = Some(next);
        //         cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        //     } else {
        //         cur.next = Some(node);
        //         cur = cur.next.as_mut().unwrap();
        //     }
        // }
        // dummy_head.next
        if head == None || head.as_ref().unwrap().next == None {
            return head;
        }

        let mut node = head.unwrap();

        if let Some(mut next) = node.next.take() {
            node.next = Solution::swap_pairs(next.next);
            next.next = Some(node);
            Some(next)
        } else {
            Some(node)
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
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
        let res = Solution::swap_pairs(Some(Box::new(head)));
        println!("{:?}", res);
    }
}
