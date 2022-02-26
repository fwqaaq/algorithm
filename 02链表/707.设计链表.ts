/*
 * @lc app=leetcode.cn id=707 lang=typescript
 *
 * [707] 设计链表
 */

export class ListNode {
  public val: number
  public next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

// @lc code=start
class MyLinkedList {
  private size: number
  private head: ListNode | null
  private tail: ListNode | null

  constructor() {
    this.size = 0
    this.head = null
    this.tail = null
  }
  //获取链表中第index个节点的值
  get(index: number): number {
    if (index < 0 || index >= this.size) return -1
    let cur = this.getNode(index)
    return cur!.val
  }

  //在链表头部增加一个值为val的节点
  addAtHead(val: number): void {
    this.head = new ListNode(val, this.head)
    if (!this.tail) this.tail = this.head
    this.size++
  }

  // 将值为val的节点追加到链表中最后一个元素
  addAtTail(val: number): void {
    let node = new ListNode(val, null)
    if (this.tail) {
      this.tail.next = node
    } else {
      this.head = node
    }
    //确保this.tail是最后一个节点
    this.tail = node
    console.log(this.tail === this.head)
    this.size++
  }

  //在链表第index个之前插入值为val的节点
  addAtIndex(index: number, val: number): void {
    if (index === this.size) {
      this.addAtTail(val)
      return
    }
    if (index <= 0) {
      this.addAtHead(val)
      return
    }
    if (index > this.size) return
    //正常情况
    let cur = this.getNode(index - 1)
    cur!.next = new ListNode(val, cur!.next)
    this.size++
  }

  deleteAtIndex(index: number): void {
    if (index >= this.size || index < 0) return
    if (index === 0) {
      this.head = this.head!.next
      this.size--
      return
    }
    //正常情况
    let cur = this.getNode(index - 1)
    cur!.next = cur!.next!.next
    if (index === this.size - 1) this.tail = cur
    this.size--
  }
  public getNode(index: number): ListNode | null {
    //设置虚拟头节点
    let cur = new ListNode(0, this.head)
    for (let i = 0; i <= index; i++) {
      cur = cur.next!
    }
    return cur
  }
}

var obj = new MyLinkedList()
var param_1 = obj.get(1)
//obj.addAtHead(1)
obj.addAtTail(3)
obj.addAtIndex(1, 2)
obj.deleteAtIndex(0)

// @lc code=end
