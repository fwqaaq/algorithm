/**
 * /*
 *
 * [637] 二叉树的层平均值
 *
 * @format
 * @lc app=leetcode.cn id=637 lang=typescript
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */
import type { binaryTrees as TreeNode } from "../nodeTree"
function averageOfLevels(root: TreeNode<number> | null): number[] {
  let res: number[] = []
  if (root === null) return res
  let queue: TreeNode<number>[] = [root]
  while (queue.length !== 0) {
    let sum = 0
    let queueLen = queue.length
    for (let i = 0; i < queueLen; i++) {
      let queueNode = queue.shift()
      sum += queueNode!.key
      if (queueNode!.left) queue.push(queueNode!.left)
      if (queueNode!.right) queue.push(queueNode!.right)
    }
    res.push(sum / queueLen)
  }
  return res
}
// @lc code=end
