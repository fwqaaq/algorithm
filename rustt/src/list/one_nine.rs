/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
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
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut fast = &dummy_head.clone();
        let mut slow = &mut dummy_head;
        while n > 0 {
            fast = fast.next.as_ref().unwrap();
            n -= 1;
        }
        while fast.next.is_some() {
            fast = fast.next.as_ref().unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        slow.next = slow.next.as_mut().unwrap().next.take();
        dummy_head.next
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let head = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        };
        let res = Solution::remove_nth_from_end(Some(Box::new(head)), 2);
        println!("{:?}", res);
    }
}
