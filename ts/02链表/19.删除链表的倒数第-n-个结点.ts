/*
 * @lc app=leetcode.cn id=19 lang=typescript
 *
 * [19] 删除链表的倒数第 N 个结点
 */

import { ListNode } from './707.设计链表.js'

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
  let fast: ListNode | null = cur
  let low: ListNode | null = cur
  for (let i = 0; i < n; i++) {
    if (fast) fast = fast.next
  }
  while (fast?.next) {
    fast = fast.next
    if (low) low = low.next
  }
  if (low && low.next) low.next = low.next.next
  return cur.next
}
// @lc code=end
