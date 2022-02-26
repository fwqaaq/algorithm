/*
 * @lc app=leetcode.cn id=19 lang=typescript
 *
 * [19] 删除链表的倒数第 N 个结点
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

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
  //定义虚拟头节点
  let cur = new ListNode(0, head)
  let fast = cur
  let low = cur
  for (let i = 0; i < n; i++) {
    fast = fast.next
  }
  while (fast.next) {
    fast = fast.next
    low = low.next
  }
  low.next = low.next.next
  return cur.next
}
// @lc code=end
