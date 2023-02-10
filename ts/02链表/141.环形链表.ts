/*
 * @lc app=leetcode.cn id=141 lang=typescript
 *
 * [141] 环形链表
 */

import { ListNode } from "./707.设计链表"

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

function hasCycle(head: ListNode | null): boolean {
  let low = head
  let high = head
  while (high !== null && high.next !== null) {
    low = low!.next
    high = high!.next!.next
    if (low === high) {
      return true
    }
  }
  return false
}
// @lc code=end
