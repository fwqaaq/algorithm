/*
 * @lc app=leetcode.cn id=24 lang=typescript
 *
 * [24] 两两交换链表中的节点
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

function swapPairs(head: ListNode | null): ListNode | null {
  let VNode = new ListNode(0, head)
  let cur = VNode
  while (cur && cur.next && cur.next.next) {
    let node1 = cur.next
    let node2 = cur.next.next
    //保存node2的下一个节点
    let tmp = node2.next
    //如果有虚拟头节点指向,并且cur指向的节点顺序不会变
    cur.next = node2
    //node2指向node1,交换顺序完成
    node2.next = node1
    //node1指向node2的下一个节点
    node1.next = tmp
    cur = cur.next.next
  }
  return VNode.next
}
// @lc code=end
