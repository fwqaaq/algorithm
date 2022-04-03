/** @format */

/*
 * @lc app=leetcode.cn id=144 lang=typescript
 *
 * [144] 二叉树的前序遍历
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
//* 递归
//function preorderTraversal(root: TreeNode<number> | null): number[] {
//  function traverse(
//    root: TreeNode<number> | null,
//    result: number[]
//  ): number[] | undefined {
//    if (root === null) return
//    result.push(root.key)
//    traverse(root.left, result)
//    traverse(root.right, result)
//  }
//  let result: number[] = []
//  traverse(root, result)
//  return result
//}

//* 迭代
function preorderTraversal(root: TreeNode<number> | null): number[] {
  if (root === null) return []
  const res: number[] = []
  const stack: TreeNode<number>[] = [root]
  while (stack.length > 0) {
    const node = stack.pop()
    //* 只要栈内有节点,就把节点的值放入结果数组
    res.push(node!.key)
    if (node!.right !== null) stack.push(node!.right)
    if (node!.left !== null) stack.push(node!.left)
  }
  return res
}

// @lc code=end
