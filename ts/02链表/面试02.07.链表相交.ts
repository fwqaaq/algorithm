import { ListNode } from "./707.设计链表"
function getIntersectionNode(
  headA: ListNode | null,
  headB: ListNode | null
): ListNode | null {
  let sizeA = 0,
    sizeB = 0
  let curA = headA,
    curB = headB
  while (curA) {
    sizeA++
    curA = curA.next
  }
  while (curB) {
    sizeB++
    curB = curB.next
  }
  if (sizeA < sizeB) {
    ;[headA, headB] = [headB, headA]
    ;[sizeA, sizeB] = [sizeB, sizeA]
  }
  let diff = sizeA - sizeB
  while (diff-- && headA) {
    headA = headA.next
  }
  while (headA && headB) {
    if (headA === headB) {
      return headA
    }
    headA = headA.next
    headB = headB.next
  }
  return null
}
