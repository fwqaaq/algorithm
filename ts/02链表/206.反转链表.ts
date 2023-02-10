/*
 * @lc app=leetcode.cn id=206 lang=typescript
 *
 * [206] 反转链表
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

function reverseList(head: ListNode | null): ListNode | null {
  let pre: ListNode = null
  let cur: ListNode = head
  while (cur) {
    //*保存下一个节点
    let next = cur.next
    //*将当前节点的下一个节点指向前一个节点
    cur.next = pre
    //*将前一个节点指向当前节点
    pre = cur
    cur = next
  }
  return pre
}
// @lc code=end
