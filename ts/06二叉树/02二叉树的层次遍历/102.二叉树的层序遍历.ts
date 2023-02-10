/**
 * /*
 *
 * [102] 二叉树的层序遍历
 *
 * @format
 * @lc app=leetcode.cn id=102 lang=typescript
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
function levelOrder(root: TreeNode<number> | null): number[][] {
  let res: number[][] = []
  if (root === null) return res
  let stack: TreeNode<number>[] = [root]
  while (stack.length !== 0) {
    //记录当前层级的节点数
    let len = stack.length
    //记录当前层级的节点
    let curStack: number[] = []
    for (let i = 0; i < len; i++) {
      //* 由于每次会增加节点(push),所以这里从头部去除节点
      let curNode = stack.shift()
      curStack.push(curNode!.key)
      if (curNode!.right) stack.push(curNode!.right)
      if (curNode!.left) stack.push(curNode!.left)
    }
    res.push(curStack)
  }
  return res
}
// @lc code=end
