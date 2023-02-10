/**
 * /*
 *
 * [116] 填充每个节点的下一个右侧节点指针
 *
 * @format
 * @lc app=leetcode.cn id=116 lang=typescript
 */

// @lc code=start

// Definition for Node.
class Node {
  val: number
  left: Node | null
  right: Node | null
  next: Node | null
  constructor(val?: number, left?: Node, right?: Node, next?: Node) {
    this.val = val === undefined ? 0 : val
    this.left = left === undefined ? null : left
    this.right = right === undefined ? null : right
    this.next = next === undefined ? null : next
  }
}

function connect(root: Node | null): Node | null {
  if (root === null) return null
  let queue: Node[] = [root]
  let preNode: Node, curNode: Node
  while (queue.length !== 0) {
    let len = queue.length
    for (let i = 0; i < len; i++) {
      if (i === 0) {
        //* 如果是第一个节点,则将他赋值给preNode
        preNode = queue.shift()!
      } else {
        //* 如果不是第一个节点,将当前节点赋值给curNode
        curNode = queue.shift()!
        preNode!.next = curNode
        //* 需要将当前节点左右子节点入队
        preNode = curNode
      }
      if (preNode!.left) queue.push(preNode!.left)
      if (preNode!.right) queue.push(preNode!.right)
    }
    //* 一层遍历之后,将preNode置空(尾节点)
    preNode!.next = null
  }
  return root
}
// @lc code=end
export {}
