/*
 * @lc app=leetcode.cn id=142 lang=typescript
 *
 * [142] 环形链表 II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */
import { ListNode } from './707.设计链表.js'
function detectCycle(head: ListNode | null): ListNode | null {
  let slow = head
  let high = head
  while (high && high.next !== null) {
    if (slow) slow = slow.next
    high = high.next.next
    if (high === slow) {
      let cur = head
      while (cur !== slow) {
        if (cur) cur = cur.next
        if (slow) slow = slow.next
      }
      return cur
    }
  }
  return null
}
// @lc code=end
