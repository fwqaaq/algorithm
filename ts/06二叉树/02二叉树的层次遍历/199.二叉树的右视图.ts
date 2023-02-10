/**
 * /*
 *
 * [199] 二叉树的右视图
 *
 * @format
 * @lc app=leetcode.cn id=199 lang=typescript
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
function rightSideView(root: TreeNode<number> | null): number[] {
  let res: number[] = []
  if (root === null) return res
  let queue: TreeNode<number>[] = [root]
  while (queue.length !== 0) {
    let queueLen = queue.length
    for (let i = 0; i < queueLen; i++) {
      //* 由于每次会增加节点(push),所以这里从头部去除节点
      let curNode = queue.shift()
      //* 从右往左遍历
      if (i === 0) {
        res.push(curNode!.key)
      }
      //* 从右往左入队
      if (curNode!.right) queue.push(curNode!.right)
      if (curNode!.left) queue.push(curNode!.left)
    }
  }
  return res
}
// @lc code=end
