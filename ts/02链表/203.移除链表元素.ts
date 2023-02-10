/*
 * @lc app=leetcode.cn id=203 lang=typescript
 *
 * [203] 移除链表元素
 */

// @lc code=start

//Definition for singly-linked list.
class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

function removeElements(head: ListNode | null, val: number): ListNode | null {
  //加入一个虚拟头节点
  head = new ListNode(0, head)
  let pre: ListNode = head,
    cur: ListNode = head.next
  //删除有val的节点
  while (cur) {
    if (cur.val === val) {
      pre.next = cur.next
    } else {
      pre = pre.next
    }
    //更新cur
    cur = cur.next
  }
  return pre.next
}
// @lc code=end

function remove(head: ListNode | null, val: number): ListNode | null {
  while (head !== null && head.val === val) {
    head = head.next
  }
  if (head === null) return head
  let pre = head,
    cur = head.next
  while (cur) {
    if (cur.val === val) {
      pre.next = cur.next
    } else {
      pre = pre.next
    }
    cur = cur.next
  }
  return head
}

export {}
