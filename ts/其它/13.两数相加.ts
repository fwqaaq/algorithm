class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

function addTwoNumbers(
  l1: ListNode | null,
  l2: ListNode | null
): ListNode | null {
  let head: ListNode = new ListNode(1)
  let tail: ListNode = head
  let carry = 0
  while (l1 !== null || l2 !== null) {
    const n1 = l1 !== null ? l1.val : 0
    const n2 = l2 !== null ? l2.val : 0
    let sum = n1 + n2 + carry
    carry = Math.floor(sum / 10)
    sum %= 10
    tail.next = new ListNode(sum)
    tail = tail.next
    if (l1 !== null) l1 = l1.next
    if (l2 !== null) l2 = l2.next
  }
  if (carry === 1) tail.next = new ListNode(1)
  return head.next
}

let l1 = new ListNode(1, new ListNode(7, new ListNode(7)))
let l2 = new ListNode(4, new ListNode(3))
console.log(addTwoNumbers(l1, l2))
