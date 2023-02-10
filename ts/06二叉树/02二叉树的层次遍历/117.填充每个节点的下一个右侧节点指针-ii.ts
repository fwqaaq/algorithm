/**
 * /*
 *
 * [117] 填充每个节点的下一个右侧节点指针 II
 *
 * @format
 * @lc app=leetcode.cn id=117 lang=typescript
 */

// @lc code=start
/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     left: Node | null
 *     right: Node | null
 *     next: Node | null
 *     constructor(val?: number, left?: Node, right?: Node, next?: Node) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function connect(root: Node | null): Node | null {
  if (root === null) return null
  let queue: Node[] = [root]
  let preNode: Node, curNode: Node
  while (queue.length !== 0) {
    let len = queue.length
    for (let i = 0; i < len; i++) {
      if (i === 0) {
        preNode = queue.shift()!
      } else {
        curNode = queue.shift()!
        preNode!.next = curNode
        preNode = curNode
      }
      if (preNode!.left) queue.push(preNode!.left)
      if (preNode!.right) queue.push(preNode!.right)
    }
    preNode!.next = null
  }
  return root
}
// @lc code=end
export {}
