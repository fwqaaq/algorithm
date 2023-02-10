/**
 * /*
 *
 * [515] 在每个树行中找最大值
 *
 * @format
 * @lc app=leetcode.cn id=515 lang=typescript
 */

// @lc code=start

// Definition for a binary tree node.
class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val
    this.left = left === undefined ? null : left
    this.right = right === undefined ? null : right
  }
}

function largestValues(root: TreeNode | null): number[] {
  let res: number[] = []
  if (root === null) return res
  let queue: TreeNode[] = [root]
  while (queue.length !== 0) {
    let len = queue.length
    //* 定义每层中初始比较的最小值
    let max = -Number.MAX_VALUE
    for (let i = 0; i < len; i++) {
      let curNode = queue.shift()
      max = Math.max(max, curNode!.val)
      if (curNode!.left) queue.push(curNode!.left)
      if (curNode!.right) queue.push(curNode!.right)
    }
    res.push(max)
  }
  return res
}
// @lc code=end
export {}
