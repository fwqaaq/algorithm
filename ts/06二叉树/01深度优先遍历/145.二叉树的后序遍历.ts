/**
 * /*
 *
 * [145] 二叉树的后序遍历
 *
 * @format
 * @lc app=leetcode.cn id=145 lang=typescript
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

import { binaryTrees as TreeNode } from "../nodeTree"

//function postorderTraversal(root: TreeNode<number> | null): number[] {
//  function traverse(
//    root: TreeNode<number> | null,
//    result: number[]
//  ): number[] | undefined {
//    if (root === null) return
//    traverse(root.left, result)
//    traverse(root.right, result)
//    result.push(root.key)
//  }
//
//  let result: number[] = []
//  traverse(root, result)
//  return result
//}

function postorderTraversal(root: TreeNode<number>): number[] {
  if (root === null) return []
  const res: number[] = []
  const stack: TreeNode<number>[] = [root]
  while (stack.length > 0) {
    let node = stack.pop()
    res.push(node!.key)
    if (node!.left !== null) stack.push(node!.left)
    if (node!.right !== null) stack.push(node!.right)
  }
  return res.reverse()
}
// @lc code=end
