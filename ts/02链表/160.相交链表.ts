/**
 * /*
 *
 * [160] 相交链表
 *
 * @format
 * @lc app=leetcode.cn id=160 lang=typescript
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 *
 * */
class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}
/* 
Todo: we divide the pointer is gonna take into three parts:
   beacuse a + c + b =  b + c + a
   When headA === null, we set headA = headB; instead, headB = headA
   the pointer will go same path before pointer is in the same position
*/
function getIntersectionNode(
  headA: ListNode | null,
  headB: ListNode | null
): ListNode | null {
  if (headA === undefined || headB === undefined) return null
  let curA = headA,
    curB = headB
  while (curA !== curB) {
    curA = curA === null ? headB : curA.next
    curB = curB === null ? headA : curB.next
  }
  return curA
}
// @lc code=end

export {}
