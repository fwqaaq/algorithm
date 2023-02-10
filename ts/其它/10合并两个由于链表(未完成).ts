class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

function mergeTwoLists(l1: ListNode | null, l2: ListNode | null): void {
  console.log(l1, l2)
}

mergeTwoLists(
  new ListNode(1, new ListNode(2)),
  new ListNode(2, new ListNode(3))
)
export {}
