/**
 * /*
 *
 * [94] 二叉树的中序遍历
 *
 * @format
 * @lc app=leetcode.cn id=94 lang=typescript
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
//function inorderTraversal(root: TreeNode<number> | null): number[] {
//  function traverse(
//    root: TreeNode<number> | null,
//    result: number[]
//  ): number[] | undefined {
//    if (root === null) return
//    traverse(root.left, result)
//    result.push(root.key)
//    traverse(root.right, result)
//  }
//  let result: number[] = []
//  traverse(root, result)
//  return result
//}

function inorderTraversal(root: TreeNode<number> | null): number[] {
  if (root === null) return []
  let res: number[] = []
  const stack: TreeNode<number>[] = []
  while (stack.length > 0 || root !== null) {
    if (root !== null) {
      stack.push(root)
      root = root.left
    } else {
      root = stack.pop()!
      res.push(root.key)
      root = root.right
    }
  }
  return res
}
// @lc code=end
