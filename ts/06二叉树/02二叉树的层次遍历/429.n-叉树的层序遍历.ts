/**
 * /*
 *
 * [429] N 叉树的层序遍历
 *
 * @format
 * @lc app=leetcode.cn id=429 lang=typescript
 */

// @lc code=start

//  Definition for node.
class Node {
  val: number
  children: Node[]
  constructor(val?: number) {
    this.val = val === undefined ? 0 : val
    this.children = []
  }
}

function levelOrder(root: Node | null): number[][] {
  let res: number[][] = []
  if (root === null) return res
  let queue: Node[] = [root]
  while (queue.length !== 0) {
    let len = queue.length
    let curArr: number[] = []
    for (let i = 0; i < len; i++) {
      let curNode = queue.shift()
      curArr.push(curNode!.val)
      if (curNode!.children.length !== 0) {
        for (let i = 0; i < curNode!.children.length; i++) {
          queue.push(curNode!.children[i])
        }
      }
    }
    res.push(curArr)
  }
  return res
}
// @lc code=end

export {}
